import { loadProjects } from '#lib/server/projects.js';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const { projects, lastFetched } = await loadProjects();
	return { projects, lastFetched };
};
