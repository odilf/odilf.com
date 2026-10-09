import { error } from '@sveltejs/kit';
import { mediaLogEntries } from '#lib/fixtures/index.js';
import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	const entry = mediaLogEntries.find((candidate) => candidate.slug === params.slug);
	if (!entry) error(404, 'Media log entry not found');

	return { entry };
};
