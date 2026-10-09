import { photos } from '#lib/fixtures/index.js';

export function load() {
	return { photos, ids: photos.map((photo) => photo.id) };
}
