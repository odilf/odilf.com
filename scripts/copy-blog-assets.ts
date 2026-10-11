import { copyFile, mkdir, readdir, readFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { loadEnvFile } from 'node:process';
import { parseBlogMetadata, renderBlogMarkdown } from '../src/lib/markdown/index.js';

try {
	loadEnvFile('.env');
} catch {
	// no .env file; rely on the ambient environment
}

const blogPath = process.env.ODILF_BLOG_PATH;
if (!blogPath) throw new Error('ODILF_BLOG_PATH is not set');

const outputDir = join(process.cwd(), 'static/blog');

const files = await readdir(blogPath, { withFileTypes: true });
const copied = new Set<string>();

for (const file of files) {
	if (!file.isFile() || !file.name.endsWith('.md')) continue;

	const content = await readFile(join(blogPath, file.name), 'utf8');
	const metadata = parseBlogMetadata(content);
	if (!metadata || metadata.draft !== false) continue;

	const rendered = await renderBlogMarkdown(content);
	for (const link of rendered.referencedLinks) {
		if (copied.has(link)) continue;
		copied.add(link);

		await mkdir(dirname(join(outputDir, link)), { recursive: true });
		await copyFile(join(blogPath, link), join(outputDir, link));
	}
}

console.log(`Copied ${copied.size} blog assets to ${outputDir}`);
