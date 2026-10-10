import { readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { photosSchema, type Photo } from '#lib/schemas/pics.js';

const METADATA_FILE = join(process.cwd(), '.cache/pics/metadata.json');

export async function loadPhotos(): Promise<Photo[]> {
	let raw: string;
	try {
		raw = await readFile(METADATA_FILE, 'utf8');
	} catch {
		return [];
	}

	let photos: Photo[];
	try {
		photos = photosSchema.parse(JSON.parse(raw));
	} catch (error) {
		throw new Error(`Invalid pics metadata at ${METADATA_FILE}`, { cause: error });
	}

	const last = photos.findIndex(
		(photo) => photo.caption === "and in every timeline, you're still there",
	);
	if (last !== -1) photos.push(...photos.splice(last, 1));

	return photos;
}
