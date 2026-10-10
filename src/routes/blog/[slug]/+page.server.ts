import { error } from '@sveltejs/kit';
import { loadBlogEntries } from '#lib/server/blog.js';
import type { EntryGenerator, PageServerLoad } from './$types';

export const csr = false;

export const entries: EntryGenerator = async () => {
	const { entries: posts } = await loadBlogEntries();
	return posts.map((post) => ({ slug: post.slug }));
};

export const load: PageServerLoad = async ({ params }) => {
	const { entries: posts } = await loadBlogEntries();
	const entry = posts.find((post) => post.slug === params.slug);
	if (!entry) error(404, 'Post not found');

	return { entry };
};
