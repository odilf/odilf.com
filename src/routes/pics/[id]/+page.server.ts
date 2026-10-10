import { error } from '@sveltejs/kit';
import { loadPhotos } from '#lib/server/pics.js';
import type { EntryGenerator, PageServerLoad } from './$types';

export const entries: EntryGenerator = async () => {
	const photos = await loadPhotos();
	return photos.map((photo) => ({ id: photo.id }));
};

export const load: PageServerLoad = async ({ params }) => {
	const photos = await loadPhotos();
	const index = photos.findIndex((photo) => photo.id === params.id);
	if (index === -1) error(404, 'Photo not found');

	return { photo: photos[index], index, ids: photos.map((photo) => photo.id) };
};
