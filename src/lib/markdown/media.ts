import matter from 'gray-matter';
import rehypeKatex from 'rehype-katex';
import rehypeRaw from 'rehype-raw';
import rehypeStringify from 'rehype-stringify';
import remarkGfm from 'remark-gfm';
import remarkMath from 'remark-math';
import remarkParse from 'remark-parse';
import remarkRehype from 'remark-rehype';
import { unified } from 'unified';
import { toIsoDate } from '../date.js';
import type { MediaDate, MediaLogFrontmatter, MediaType } from '../types.js';
import { rehypeFigure } from './shared.js';

export function parseMediaMetadata(content: string): MediaLogFrontmatter {
	const { data } = matter(content);
	if (data.review != null) {
		throw new Error('`review` field present in frontmatter');
	}

	return {
		title: String(data.title),
		type: data.type as MediaType,
		rating: Number(data.rating),
		date: toMediaDate(data.date),
		urls: Array.isArray(data.urls) ? data.urls.map((url) => String(url)) : [],
		'image-url': typeof data['image-url'] === 'string' ? data['image-url'] : undefined,
	};
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

function toMediaDate(value: unknown): MediaDate {
	if (Array.isArray(value)) {
		const start = toIsoDate(value[0]);
		const end = toIsoDate(value[1]);
		if (start && end) return [start, end];
	}

	const single = toIsoDate(value);
	if (single) return single;

	throw new Error('Invalid media date');
}
