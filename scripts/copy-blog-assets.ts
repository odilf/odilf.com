import { copyFile, mkdir, rm } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { loadEnvFile } from 'node:process';
import { loadBlogSource } from '../src/lib/blog-source.js';

try {
	loadEnvFile('.env');
} catch {
	// no .env file; rely on the ambient environment
}

const blogPath = process.env.ODILF_BLOG_PATH;
if (!blogPath) throw new Error('ODILF_BLOG_PATH is not set');

const includeDrafts = process.argv.includes('--include-drafts');
const outputDir = join(process.cwd(), 'static/blog');

await rm(outputDir, { recursive: true, force: true });

const { referencedLinks } = await loadBlogSource(blogPath, { hideDrafts: !includeDrafts });

const copied = new Set<string>();

for (const link of referencedLinks) {
	if (copied.has(link)) continue;
	// Absolute paths and URLs are served elsewhere; only blog-relative assets are copied.
	if (link.startsWith('/') || /^[a-z][a-z0-9+.-]*:/i.test(link)) continue;
	copied.add(link);

	await mkdir(dirname(join(outputDir, link)), { recursive: true });
	await copyFile(join(blogPath, link), join(outputDir, link));
}

console.log(`Copied ${copied.size} blog assets to ${outputDir}`);
