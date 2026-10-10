import { loadMediaLogEntries } from '#lib/server/media-log.js';

export const prerender = true;

export async function GET() {
	const entries = await loadMediaLogEntries();

	const json = entries.map((entry) => ({
		title: entry.title,
		type: entry.type,
		rating: entry.rating,
		date: entry.date,
		urls: entry.urls,
		review: entry.review,
		'image-url': entry.imageUrl,
	}));

	return new Response(JSON.stringify(json, null, 2), {
		headers: { 'content-type': 'application/json' },
	});
}
