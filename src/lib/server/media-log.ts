import { mkdir, readdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { z } from 'zod';
import { building } from '$app/env';
import { ODILF_MEDIA_LOG_PATH } from '$app/env/private';
import { mediaDateTimestamp } from '#lib/date.js';
import { parseMediaMetadata, renderMediaReview } from '#lib/markdown/index.js';
import type { MediaLog } from '#routes/media-log/index.js';

const COVER_CACHE_DIR = join(process.cwd(), '.cache/media-log-covers');
const USER_AGENT = 'odilf.com build';
const MAX_ATTEMPTS = 3;

const wikipediaPageSchema = z.object({
	pageprops: z.object({ page_image: z.string().optional() }).optional(),
	imageinfo: z.array(z.object({ url: z.string().optional() })).optional(),
	original: z.object({ source: z.string().optional() }).optional(),
});

const wikipediaResponseSchema = z.object({
	query: z.object({ pages: z.record(z.string(), wikipediaPageSchema) }).optional(),
});
type WikipediaResponse = z.infer<typeof wikipediaResponseSchema>;

let buildCache: Promise<MediaLog[]> | undefined;

export function loadMediaLogEntries(): Promise<MediaLog[]> {
	if (!building) return loadMediaLogEntriesUncached();
	buildCache ??= loadMediaLogEntriesUncached();
	return buildCache;
}

async function loadMediaLogEntriesUncached(): Promise<MediaLog[]> {
	const files = await readdir(ODILF_MEDIA_LOG_PATH, { withFileTypes: true });
	const entries: MediaLog[] = [];

	for (const file of files) {
		// TODO: This is repeated, and in the other instance I want to log, so maybe write shared code.
		if (!file.isFile() || !file.name.endsWith('.md')) continue;

		const content = await readFile(join(ODILF_MEDIA_LOG_PATH, file.name), 'utf8');
		const metadata = parseMediaMetadata(content);
		const slug = file.name.slice(0, -'.md'.length);

		// TODO: A `map` here is simpler.
		entries.push({
			slug,
			title: metadata.title,
			type: metadata.type,
			rating: metadata.rating,
			date: metadata.date,
			urls: metadata.urls,
			review: renderMediaReview(content),
			imageUrl: metadata['image-url'] ?? (await loadCover(metadata.urls, slug)),
		});
	}

	entries.sort((a, b) => mediaDateTimestamp(b.date) - mediaDateTimestamp(a.date));

	return entries;
}

async function loadCover(urls: string[], slug: string): Promise<string> {
	const cacheFile = join(COVER_CACHE_DIR, slug);
	try {
		return await readFile(cacheFile, 'utf8');
	} catch {
		// no cached cover yet
	}

	for (const url of urls) {
		const cover = await fetchWikipediaCover(url);
		if (cover) {
			await mkdir(COVER_CACHE_DIR, { recursive: true });
			await writeFile(cacheFile, cover);
			return cover;
		}
	}

	// TODO: This should fail more loudly here.
	return '';
}

async function fetchWikipediaCover(url: string): Promise<string | null> {
	let parsed: URL;
	try {
		parsed = new URL(url);
	} catch {
		return null;
	}
	if (!parsed.hostname.includes('wikipedia.org')) return null;

	const title = parsed.pathname.split('/').filter(Boolean).pop();
	if (!title) return null;

	for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt++) {
		try {
			return (await fetchPageImage(title)) ?? (await fetchOriginalImage(title));
		} catch {
			if (attempt < MAX_ATTEMPTS) await sleep(attempt * 250);
		}
	}

	return null;
}

async function fetchPageImage(title: string): Promise<string | null> {
	const response = await wikipediaQuery({ action: 'query', titles: title, prop: 'pageprops' });

	for (const page of Object.values(response.query?.pages ?? {})) {
		const pageImage = page.pageprops?.page_image;
		if (!pageImage) continue;

		const url = await fetchImageInfo(`File:${pageImage}`);
		if (url) return url;
	}

	return null;
}

async function fetchImageInfo(title: string): Promise<string | null> {
	const response = await wikipediaQuery({
		action: 'query',
		titles: title,
		prop: 'imageinfo',
		iiprop: 'url',
	});

	for (const page of Object.values(response.query?.pages ?? {})) {
		const url = page.imageinfo?.[0]?.url;
		if (url) return url;
	}

	return null;
}

async function fetchOriginalImage(title: string): Promise<string | null> {
	const response = await wikipediaQuery({
		action: 'query',
		titles: title,
		prop: 'pageimages',
		piprop: 'original',
	});

	for (const page of Object.values(response.query?.pages ?? {})) {
		const source = page.original?.source;
		if (source) return source;
	}

	return null;
}

async function wikipediaQuery(params: Record<string, string>): Promise<WikipediaResponse> {
	const query = new URLSearchParams({ format: 'json', ...params });
	const response = await fetch(`https://en.wikipedia.org/w/api.php?${query}`, {
		headers: { 'user-agent': USER_AGENT },
	});
	if (!response.ok) throw new Error(`Wikipedia request failed with ${response.status}`);

	return wikipediaResponseSchema.parse(await response.json());
}

function sleep(ms: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, ms));
}
