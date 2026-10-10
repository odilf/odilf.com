import type { BlogEntry } from '#routes/blog/index.js';

const SITE = 'https://odilf.com';
const FEED_TITLE = "Odilf's blog";
const FEED_DESCRIPTION = "Odilf's personal blog.";
const AUTHOR_EMAIL = 'odysseas.maheras@gmail.com';

const DAYS = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
const MONTHS = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];

export function renderRss(entries: BlogEntry[]): string {
	return [
		'<?xml version="1.0" encoding="UTF-8"?>',
		'<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">',
		'<channel>',
		`<title>${escapeXml(FEED_TITLE)}</title>`,
		`<link>${SITE}/blog</link>`,
		`<description>${escapeXml(FEED_DESCRIPTION)}</description>`,
		'<language>en</language>',
		`<webMaster>${AUTHOR_EMAIL} (Odilf)</webMaster>`,
		`<lastBuildDate>${rfc2822(new Date())}</lastBuildDate>`,
		'<generator>Custom Generator at https://github.com/odilf/odilf.com</generator>',
		`<atom:link href="${SITE}/blog/rss.xml" rel="self" type="application/rss+xml"></atom:link>`,
		entries.map(renderRssItem).join(''),
		'</channel>',
		'</rss>',
	].join('');
}

export function renderAtom(entries: BlogEntry[]): string {
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
		entries.map(renderAtomItem).join(''),
		'</feed>',
	].join('');
}

function renderRssItem(entry: BlogEntry): string {
	const url = `${SITE}/blog/${entry.slug}`;
	const pubDate = entry.metadata.date
		? `<pubDate>${rfc2822(localDate(entry.metadata.date))}</pubDate>`
		: '';

	return `<item><title>${escapeXml(entry.metadata.title)}</title><link>${url}</link><description>${escapeXml(entry.summary)}</description>${pubDate}<guid isPermaLink="true">${url}</guid></item>`;
}

function renderAtomItem(entry: BlogEntry): string {
	const url = `${SITE}/blog/${entry.slug}`;
	const updated = entry.metadata.date
		? `<updated>${isoWithOffset(localDate(entry.metadata.date))}</updated>`
		: '';

	return `<entry><title>${escapeXml(entry.metadata.title)}</title><link href="${url}"></link><id>${url}</id>${updated}<summary>${escapeXml(entry.summary)}</summary><content type="html"><![CDATA[${cdata(entry.html)}]]></content></entry>`;
}

function localDate(iso: string): Date {
	return new Date(`${iso}T00:00:00`);
}

function rfc2822(date: Date): string {
	const offset = -date.getTimezoneOffset();
	const sign = offset >= 0 ? '+' : '-';
	const absolute = Math.abs(offset);
	const zone = `${pad(Math.floor(absolute / 60))}${pad(absolute % 60)}`;

	return `${DAYS[date.getDay()]}, ${pad(date.getDate())} ${MONTHS[date.getMonth()]} ${date.getFullYear()} ${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())} ${sign}${zone}`;
}

function isoWithOffset(date: Date): string {
	const offset = -date.getTimezoneOffset();
	const sign = offset >= 0 ? '+' : '-';
	const absolute = Math.abs(offset);
	const zone = `${pad(Math.floor(absolute / 60))}:${pad(absolute % 60)}`;

	return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}T${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}${sign}${zone}`;
}

function pad(value: number): string {
	return String(value).padStart(2, '0');
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
