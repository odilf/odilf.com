import { loadMediaLogEntries } from '#lib/server/media-log.js';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	return { entries: await loadMediaLogEntries() };
};
