import { readdir, readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { building } from '$app/env';
import { ODILF_BLOG_PATH } from '$app/env/private';
import { parseBlogMetadata, renderBlogMarkdown } from '#lib/markdown/index.js';
import type { BlogEntry } from '#lib/schemas/blog.js';

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

async function loadBlogEntriesUncached(): Promise<BlogContent> {
	const files = await readdir(ODILF_BLOG_PATH, { withFileTypes: true });
	const entries: BlogEntry[] = [];
	const referencedLinks: string[] = [];

	for (const file of files) {
		// TODO: Warn or log skipped files.
		if (!file.isFile() || !file.name.endsWith('.md')) continue;

		const content = await readFile(join(ODILF_BLOG_PATH, file.name), 'utf8');
		const metadata = parseBlogMetadata(content);
		if (!metadata) continue;
		if (building && metadata.draft !== false) continue;

		const rendered = renderBlogMarkdown(content);
		referencedLinks.push(...rendered.referencedLinks);
		entries.push({
			slug: file.name.slice(0, -'.md'.length),
			html: rendered.html,
			summary: rendered.summary,
			wordCount: rendered.wordCount,
			metadata,
		});
	}

	entries.sort((a, b) => (b.metadata.date ?? '').localeCompare(a.metadata.date ?? ''));

	return { entries, referencedLinks };
}
