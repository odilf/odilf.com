import { loadBlogEntries } from '#lib/server/blog.js';
import type { BlogEntry } from '#routes/blog/index.js';

const SITE = 'https://odilf.com';
const FEED_TITLE = "Odilf's blog";
const FEED_DESCRIPTION = "Odilf's personal blog.";
const AUTHOR_EMAIL = 'contact@odilf.com';
/** Date-only blog entries are published at 10:00 UTC+2. */
const PUBLISH_TIME = '10:00:00+02:00';

export const prerender = true;

export async function GET() {
	const { entries } = await loadBlogEntries();

	return new Response(renderRss(entries), {
		headers: { 'content-type': 'application/rss+xml; charset=utf-8' },
	});
}

function renderRss(entries: BlogEntry[]): string {
	return [
		'<?xml version="1.0" encoding="UTF-8"?>',
		'<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">',
		'<channel>',
		`<title>${escapeXml(FEED_TITLE)}</title>`,
		`<link>${SITE}/blog</link>`,
		`<description>${escapeXml(FEED_DESCRIPTION)}</description>`,
		'<language>en</language>',
		`<webMaster>${AUTHOR_EMAIL} (Odilf)</webMaster>`,
		`<lastBuildDate>${new Date().toUTCString()}</lastBuildDate>`,
		'<generator>Custom Generator at https://github.com/odilf/odilf.com</generator>',
		`<atom:link href="${SITE}/blog/rss.xml" rel="self" type="application/rss+xml"></atom:link>`,
		entries.map(renderItem).join(''),
		'</channel>',
		'</rss>',
	].join('');
}

function renderItem(entry: BlogEntry): string {
	const url = `${SITE}/blog/${entry.slug}`;
	const pubDate = entry.metadata.date
		? `<pubDate>${new Date(`${entry.metadata.date}T${PUBLISH_TIME}`).toUTCString()}</pubDate>`
		: '';

	return `<item><title>${escapeXml(entry.metadata.title)}</title><link>${url}</link><description>${escapeXml(entry.summary)}</description>${pubDate}<guid isPermaLink="true">${url}</guid></item>`;
}

function escapeXml(value: string): string {
	return value
		.replace(/&/g, '&amp;')
		.replace(/</g, '&lt;')
		.replace(/>/g, '&gt;')
		.replace(/"/g, '&quot;');
}
