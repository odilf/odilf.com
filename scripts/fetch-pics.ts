import { existsSync } from 'node:fs';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import sharp from 'sharp';
import type { Photo } from '#lib/types.js';

const immichUrl = process.env.IMMICH_URL?.replace(/\/$/, '') ?? '';
const albumId = process.env.IMMICH_ALBUM_ID ?? '';
const apiKey = process.env.IMMICH_API_KEY ?? '';
if (!immichUrl || !albumId || !apiKey) {
	throw new Error('IMMICH_URL, IMMICH_ALBUM_ID and IMMICH_API_KEY must be set');
}

const IMAGES_DIR = join(process.cwd(), 'static/pics');
const METADATA_FILE = join(process.cwd(), '.cache/pics/metadata.json');
const THUMB_SIZE = 400;
const PAGE_SIZE = 1000;

type Source = 'original' | 'preview';

interface ImmichAsset {
	id: string;
	originalFileName?: string;
	exifInfo?: { description?: string | null } | null;
}

interface ImmichSearchResponse {
	assets?: { items?: ImmichAsset[]; nextPage?: string | number | null };
}

async function fetchAssets(): Promise<ImmichAsset[]> {
	const assets: ImmichAsset[] = [];
	let page = 1;

	while (true) {
		const response = await fetch(`${immichUrl}/api/search/metadata`, {
			method: 'POST',
			headers: { 'x-api-key': apiKey, 'content-type': 'application/json' },
			body: JSON.stringify({ albumIds: [albumId], withExif: true, page, size: PAGE_SIZE }),
		});
		if (!response.ok) throw new Error(`Immich search failed with ${response.status}`);

		// TODO(zod)
		const data = (await response.json()) as ImmichSearchResponse;
		assets.push(...(data.assets?.items ?? []));

		const nextPage = data.assets?.nextPage;
		if (!nextPage) break;
		page = Number(nextPage);
	}

	return assets;
}

async function download(assetId: string, source: Source): Promise<Buffer> {
	const path =
		source === 'original'
			? `/api/assets/${assetId}/original?edited=true`
			: `/api/assets/${assetId}/thumbnail?size=preview`;

	const response = await fetch(`${immichUrl}${path}`, { headers: { 'x-api-key': apiKey } });
	if (!response.ok) throw new Error(`${source} request failed with ${response.status}`);

	return Buffer.from(await response.arrayBuffer());
}

async function convert(buffer: Buffer, fullPath: string, thumbPath: string): Promise<void> {
	await sharp(buffer).rotate().webp({ quality: 85 }).toFile(fullPath);
	await sharp(buffer)
		.rotate()
		.resize(THUMB_SIZE, THUMB_SIZE, { fit: 'inside', withoutEnlargement: true })
		.webp()
		.toFile(thumbPath);
}

await mkdir(IMAGES_DIR, { recursive: true });
await mkdir(join(process.cwd(), '.cache/pics'), { recursive: true });

const assets = await fetchAssets();
const photos: Photo[] = [];
let skipped = 0;

for (const asset of assets) {
	const fullPath = join(IMAGES_DIR, `${asset.id}.webp`);
	const thumbPath = join(IMAGES_DIR, `${asset.id}_thumb.webp`);

	if (!existsSync(fullPath) || !existsSync(thumbPath)) {
		// HEIC needs an HEVC decoder the prebuilt sharp lacks, so it goes straight to Immich's
		// server-rendered preview, which requires the `asset.view` permission on the API key.
		const filename = (asset.originalFileName ?? '').toLowerCase();
		const sources: Source[] = /\.(heic|heif)$/.test(filename)
			? ['preview']
			: ['original', 'preview'];

		let converted = false;
		let failure: unknown;

		for (const source of sources) {
			try {
				await convert(await download(asset.id, source), fullPath, thumbPath);
				converted = true;
				break;
			} catch (error) {
				failure = error;
			}
		}

		if (!converted) {
			await rm(fullPath, { force: true });
			await rm(thumbPath, { force: true });
			skipped++;
			console.warn(
				`Skipping ${asset.id}: ${failure instanceof Error ? failure.message : String(failure)}`,
			);
			continue;
		}

		console.log(`Processed ${asset.id}`);
	}

	photos.push({
		id: asset.id,
		caption: asset.exifInfo?.description ?? '',
		filename: asset.originalFileName ?? '',
	});
}

await writeFile(METADATA_FILE, JSON.stringify(photos, null, 2));
console.log(`Wrote ${photos.length} photos to ${METADATA_FILE} (${skipped} skipped)`);
