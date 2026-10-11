import { readdir, readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { building, dev } from '$app/env';
import { ODILF_BLOG_PATH } from '$app/env/private';
import { parseBlogMetadata, renderBlogMarkdown } from '#lib/markdown/index.js';
import type { BlogEntry } from '#routes/blog/index.js';

export interface BlogContent {
	entries: BlogEntry[];
	referencedLinks: string[];
}

let buildCache: Promise<BlogContent> | undefined;

export function loadBlogEntries(): Promise<BlogContent> {
	if (!building) return loadBlogEntriesUncached();
	buildCache ??= loadBlogEntriesUncached();
	return buildCache;
}

const HIDE_DRAFTS = dev || true;

async function loadBlogEntriesUncached(): Promise<BlogContent> {
	const files = await readdir(ODILF_BLOG_PATH, { withFileTypes: true });
	const referencedLinks: string[] = [];

	const entries = (await Promise.all(files.map(async (file) => {
		if (!file.isFile() || !file.name.endsWith('.md')) return null;

		const content = await readFile(join(ODILF_BLOG_PATH, file.name), 'utf8');
		const metadata = parseBlogMetadata(content);
		if (!metadata) {
			console.warn(`Couldn't parse metadata of ${file}`)
			return null
		}
		if (HIDE_DRAFTS && metadata.draft) return null;

		const rendered = await renderBlogMarkdown(content);

		referencedLinks.push(...rendered.referencedLinks);
		return {
			slug: file.name.slice(0, -'.md'.length),
			html: rendered.html,
			summary: rendered.summary,
			wordCount: rendered.wordCount,
			metadata,
		};
	}))).filter(entry => entry !== null)

	entries.sort((a, b) => (b.metadata.date ?? '').localeCompare(a.metadata.date ?? ''));

	return { entries, referencedLinks };
}
