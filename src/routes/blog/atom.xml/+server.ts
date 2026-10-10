import { loadBlogEntries } from '#lib/server/blog.js';
import type { BlogEntry } from '#routes/blog/index.js';

const SITE = 'https://odilf.com';
const FEED_TITLE = "Odilf's blog";
const FEED_DESCRIPTION = "Odilf's personal blog.";
const AUTHOR_EMAIL = 'odysseas.maheras@gmail.com';
/** Date-only blog entries are published at 10:00 UTC+2. */
const PUBLISH_TIME = '10:00:00+02:00';

export const prerender = true;

export async function GET() {
	const { entries } = await loadBlogEntries();

	return new Response(renderAtom(entries), {
		headers: { 'content-type': 'application/atom+xml; charset=utf-8' },
	});
}

function renderAtom(entries: BlogEntry[]): string {
	return [
		'<?xml version="1.0" encoding="UTF-8"?>',
		'<feed xmlns="http://www.w3.org/2005/Atom">',
		`<title>${escapeXml(FEED_TITLE)}</title>`,
		`<link href="${SITE}/blog"></link>`,
		`<link href="${SITE}/blog/atom.xml" rel="self"></link>`,
		`<id>${SITE}/blog</id>`,
		`<author><name>Odilf</name><email>${AUTHOR_EMAIL}</email></author>`,
		`<subtitle>${escapeXml(FEED_DESCRIPTION)}</subtitle>`,
		`<updated>${new Date().toISOString()}</updated>`,
		'<generator uri="https://github.com/odilf/odilf.com">Custom Generator</generator>',
		entries.map(renderItem).join(''),
		'</feed>',
	].join('');
}

function renderItem(entry: BlogEntry): string {
	const url = `${SITE}/blog/${entry.slug}`;
	const updated = entry.metadata.date
		? `<updated>${new Date(`${entry.metadata.date}T${PUBLISH_TIME}`).toISOString()}</updated>`
		: '';

	return `<entry><title>${escapeXml(entry.metadata.title)}</title><link href="${url}"></link><id>${url}</id>${updated}<summary>${escapeXml(entry.summary)}</summary><content type="html"><![CDATA[${cdata(entry.html)}]]></content></entry>`;
}

function escapeXml(value: string): string {
	return value
		.replace(/&/g, '&amp;')
		.replace(/</g, '&lt;')
		.replace(/>/g, '&gt;')
		.replace(/"/g, '&quot;');
}

function cdata(value: string): string {
	return value.replace(/]]>/g, ']]]]><![CDATA[>');
}
