import { readFile } from 'node:fs/promises';
import { join } from 'node:path';
import type { Photo } from '#lib/types.js';

const METADATA_FILE = join(process.cwd(), '.cache/pics/metadata.json');

export async function loadPhotos(): Promise<Photo[]> {
	let photos: Photo[];
	try {
		// TODO(zod)
		photos = JSON.parse(await readFile(METADATA_FILE, 'utf8')) as Photo[];
	} catch {
		return [];
	}

	const last = photos.findIndex(
		(photo) => photo.caption === "and in every timeline, you're still there",
	);
	if (last !== -1) photos.push(...photos.splice(last, 1));

	return photos;
}
