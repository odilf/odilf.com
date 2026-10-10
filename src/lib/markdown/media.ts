import matter from 'gray-matter';
import * as yaml from 'js-yaml';
import rehypeKatex from 'rehype-katex';
import rehypeRaw from 'rehype-raw';
import rehypeStringify from 'rehype-stringify';
import remarkGfm from 'remark-gfm';
import remarkMath from 'remark-math';
import remarkParse from 'remark-parse';
import remarkRehype from 'remark-rehype';
import { unified } from 'unified';
import { MediaLogFrontmatter } from '#routes/media-log/index.js';
import { rehypeFigure } from './shared.js';

export function parseMediaMetadata(content: string): MediaLogFrontmatter {
	const { data } = matter(content, {
		engines: {
			yaml: (s) => yaml.load(s, { schema: yaml.CORE_SCHEMA }) as object,
			// If we wrote back, this would be needed. But we never write back.
			// stringify: (o) => yaml.dump(o),
		},
	});

	if (data.review != null) {
		throw new Error('`review` field present in frontmatter');
	}

	return MediaLogFrontmatter.parse(data);
}

export function renderMediaReview(content: string): string {
	const { content: body } = matter(content);

	return String(
		unified()
			.use(remarkParse)
			.use(remarkGfm)
			.use(remarkMath)
			.use(remarkRehype, { allowDangerousHtml: true })
			.use(rehypeRaw)
			.use(rehypeKatex)
			.use(rehypeFigure)
			.use(rehypeStringify, { allowDangerousHtml: true })
			.processSync(body),
	);
}
