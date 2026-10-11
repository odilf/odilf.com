import { readdir, readFile } from 'node:fs/promises';
import { join } from 'node:path';
import type { BlogEntry } from '#routes/blog/index.js';
import { parseBlogMetadata, renderBlogMarkdown } from './markdown/index.js';

export interface BlogContent {
	entries: BlogEntry[];
	referencedLinks: string[];
}

export interface LoadBlogSourceOptions {
	hideDrafts: boolean;
}

export async function loadBlogSource(
	blogPath: string,
	{ hideDrafts }: LoadBlogSourceOptions,
): Promise<BlogContent> {
	const files = await readdir(blogPath, { withFileTypes: true });
	const referencedLinks: string[] = [];

	const entries = (
		await Promise.all(
			files.map(async (file) => {
				if (!file.isFile() || !file.name.endsWith('.md')) return null;

				const content = await readFile(join(blogPath, file.name), 'utf8');
				const metadata = parseBlogMetadata(content);
				if (!metadata) {
					console.warn(`Couldn't parse metadata of ${file.name}`);
					return null;
				}
				if (hideDrafts && metadata.draft) return null;

				const rendered = await renderBlogMarkdown(content);

				referencedLinks.push(...rendered.referencedLinks);
				return {
					slug: file.name.slice(0, -'.md'.length),
					html: rendered.html,
					summary: rendered.summary,
					wordCount: rendered.wordCount,
					metadata,
				};
			}),
		)
	).filter((entry) => entry !== null);

	entries.sort((a, b) => (b.metadata.date ?? '').localeCompare(a.metadata.date ?? ''));

	return { entries, referencedLinks };
}
