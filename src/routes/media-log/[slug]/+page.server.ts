import { error } from '@sveltejs/kit';
import { loadMediaLogEntries } from '#lib/server/media-log.js';
import type { EntryGenerator, PageServerLoad } from './$types';

export const csr = false;

export const entries: EntryGenerator = async () => {
	const logs = await loadMediaLogEntries();
	return logs.map((log) => ({ slug: log.slug }));
};

export const load: PageServerLoad = async ({ params }) => {
	const logs = await loadMediaLogEntries();
	const entry = logs.find((log) => log.slug === params.slug);
	if (!entry) error(404, 'Media log entry not found');

	return { entry };
};
