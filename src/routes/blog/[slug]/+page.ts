import { error } from '@sveltejs/kit';
import { blogEntries } from '#lib/fixtures/index.js';
import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	const entry = blogEntries.find((candidate) => candidate.slug === params.slug);
	if (!entry) error(404, 'Post not found');

	return { entry };
};
