import { loadPhotos } from '#lib/server/pics.js';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const photos = await loadPhotos();
	return { photos, ids: photos.map((photo) => photo.id) };
};
