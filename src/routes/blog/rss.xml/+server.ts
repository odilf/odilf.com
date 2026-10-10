import { loadBlogEntries } from '#lib/server/blog.js';
import { renderRss } from '#lib/server/feeds.js';

export const prerender = true;

export async function GET() {
	const { entries } = await loadBlogEntries();

	return new Response(renderRss(entries), {
		headers: { 'content-type': 'application/rss+xml; charset=utf-8' },
	});
}
