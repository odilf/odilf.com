import { error } from '@sveltejs/kit';
import { photos } from '#lib/fixtures/index.js';
import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	const index = photos.findIndex((photo) => photo.id === params.id);
	if (index === -1) error(404, 'Photo not found');

	return { photo: photos[index], index, ids: photos.map((photo) => photo.id) };
};
