import { execFile } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, extname, join } from 'node:path';
import { promisify } from 'node:util';
import type { Photo } from '#lib/types.js';

const execFileAsync = promisify(execFile);

const immichUrl = process.env.IMMICH_URL?.replace(/\/$/, '') ?? '';
const albumId = process.env.IMMICH_ALBUM_ID ?? '';
const apiKey = process.env.IMMICH_API_KEY ?? '';
if (!immichUrl || !albumId || !apiKey) {
	throw new Error('IMMICH_URL, IMMICH_ALBUM_ID and IMMICH_API_KEY must be set');
}

const IMAGES_DIR = join(process.cwd(), 'assets/pics');
const METADATA_FILE = join(process.cwd(), '.cache/pics/metadata.json');
const PAGE_SIZE = 1000;

// Formats the build reads directly; everything else gets a side-by-side JPEG derivative.
const DIRECT_FORMATS = new Set(['.jpg', '.jpeg', '.png', '.webp']);

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

async function download(path: string, accept?: string): Promise<Buffer> {
	const headers: Record<string, string> = { 'x-api-key': apiKey };
	if (accept) headers.accept = accept;

	const response = await fetch(`${immichUrl}${path}`, { headers });
	if (!response.ok) throw new Error(`Immich request failed with ${response.status}`);

	return Buffer.from(await response.arrayBuffer());
}

function describeError(error: unknown): string {
	return error instanceof Error ? error.message : String(error);
}

async function magick(inputPath: string, outputPath: string): Promise<void> {
	try {
		await execFileAsync('magick', [
			inputPath,
			'-auto-orient',
			'-strip',
			'-quality',
			'92',
			outputPath,
		]);
	} catch (error) {
		const stderr = (error as { stderr?: string }).stderr?.trim();
		throw new Error(stderr || (error instanceof Error ? error.message : String(error)), {
			cause: error,
		});
	}
}

async function ensureOriginal(id: string, originalPath: string): Promise<void> {
	if (existsSync(originalPath)) return;

	await writeFile(originalPath, await download(`/api/assets/${id}/original?edited=true`));
}

async function ensureRender(
	id: string,
	ext: string,
	originalPath: string,
	jpgPath: string,
): Promise<void> {
	if (DIRECT_FORMATS.has(ext) || existsSync(jpgPath)) return;

	try {
		await magick(originalPath, jpgPath);
		return;
	} catch (directError) {
		await rm(jpgPath, { force: true });

		// Originals ImageMagick can't read fall back to Immich's server-rendered preview.
		const previewPath = `${jpgPath}.preview`;
		try {
			await writeFile(
				previewPath,
				await download(`/api/assets/${id}/thumbnail?size=preview`, 'image/jpeg'),
			);
			await magick(previewPath, jpgPath);
		} catch (previewError) {
			throw new Error(
				`could not render ${id} from original (${describeError(directError)}) nor preview (${describeError(previewError)})`,
				{ cause: previewError },
			);
		} finally {
			await rm(previewPath, { force: true });
		}
	}
}

await mkdir(IMAGES_DIR, { recursive: true });
await mkdir(dirname(METADATA_FILE), { recursive: true });

const assets = await fetchAssets();
const photos: Photo[] = [];
let skipped = 0;

for (const asset of assets) {
	const ext = (extname(asset.originalFileName ?? '') || '.jpg').toLowerCase();
	const originalPath = join(IMAGES_DIR, `${asset.id}${ext}`);
	const jpgPath = join(IMAGES_DIR, `${asset.id}.jpg`);

	try {
		const missing = !existsSync(originalPath);
		await ensureOriginal(asset.id, originalPath);
		if (missing) console.log(`Downloaded ${asset.id}${ext}`);
		await ensureRender(asset.id, ext, originalPath, jpgPath);
	} catch (error) {
		skipped++;
		console.warn(`Skipping ${asset.id}: ${describeError(error)}`);
		continue;
	}

	photos.push({
		id: asset.id,
		caption: asset.exifInfo?.description ?? '',
		filename: asset.originalFileName ?? '',
	});
}

await writeFile(METADATA_FILE, JSON.stringify(photos, null, 2));
console.log(`Wrote ${photos.length} photos to ${METADATA_FILE} (${skipped} skipped)`);
