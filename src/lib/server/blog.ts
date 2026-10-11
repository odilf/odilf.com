import { building, dev } from '$app/env';
import { ODILF_BLOG_PATH } from '$app/env/private';
import { loadBlogSource, type BlogContent } from '#lib/blog-source.js';

export type { BlogContent };

let buildCache: Promise<BlogContent> | undefined;

export function loadBlogEntries(): Promise<BlogContent> {
	if (!building) return loadBlogSource(ODILF_BLOG_PATH, { hideDrafts: !dev });
	buildCache ??= loadBlogSource(ODILF_BLOG_PATH, { hideDrafts: !dev });
	return buildCache;
}
