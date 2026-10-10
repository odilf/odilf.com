import { loadBlogEntries } from '#lib/server/blog.js';
import { renderAtom } from '#lib/server/feeds.js';

export const prerender = true;

export async function GET() {
	const { entries } = await loadBlogEntries();

	return new Response(renderAtom(entries), {
		headers: { 'content-type': 'application/atom+xml; charset=utf-8' },
	});
}
