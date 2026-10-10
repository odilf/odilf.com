import { loadBlogEntries } from '#lib/server/blog.js';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const { entries } = await loadBlogEntries();
	return { entries };
};
