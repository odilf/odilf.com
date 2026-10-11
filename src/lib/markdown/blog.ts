import type { Root } from 'mdast';
import rehypeKatex from 'rehype-katex';
import rehypeRaw from 'rehype-raw';
import rehypeStringify from 'rehype-stringify';
import remarkGfm from 'remark-gfm';
import remarkMath from 'remark-math';
import remarkParse from 'remark-parse';
import remarkRehype from 'remark-rehype';
import { unified, type Transformer } from 'unified';
import { visit } from 'unist-util-visit';
import { BlogMetadata } from '#routes/blog/index.js';
import { frontmatter, rehypeFigure } from './index.js';

const SUMMARY_LENGTH = 250;

export interface RenderedMarkdown {
	html: string;
	summary: string;
	wordCount: number;
	referencedLinks: string[];
}

export function parseBlogMetadata(content: string): BlogMetadata | null {
	const result = BlogMetadata.safeParse(frontmatter(content).data);
	return result.success ? result.data : null;
}

export async function renderBlogMarkdown(content: string): Promise<RenderedMarkdown> {
	const { content: body } = frontmatter(content);
	const referencedLinks: string[] = [];

	const file = await unified()
		.use(remarkParse)
		.use(remarkGfm)
		.use(remarkMath)
		.use(remarkBlogTransforms, referencedLinks)
		.use(remarkRehype, { allowDangerousHtml: true })
		.use(rehypeRaw)
		.use(rehypeKatex)
		.use(rehypeFigure)
		.use(rehypeStringify, { allowDangerousHtml: true })
		.process(body);

	return {
		html: String(file),
		summary: String(file.data.summary ?? ''),
		wordCount: Number(file.data.wordCount ?? 0),
		referencedLinks,
	};
}

function remarkBlogTransforms(referencedLinks: string[]): Transformer<Root, Root> {
	return (tree, file) => {
		let summary = '';
		let wordCount = 0;
		let summaryComplete = false;

		visit(tree, (node) => {
			if (node.type === 'heading') {
				node.depth = Math.min(node.depth + 1, 6) as typeof node.depth;
			} else if (node.type === 'image') {
				referencedLinks.push(node.url);
				node.url = node.url.startsWith('/') ? node.url : `/blog/${node.url}`;
				wordCount += countWords(node.alt ?? '');
			}

			if (node.type === 'text') {
				wordCount += countWords(node.value);
			}

			if (!summaryComplete) {
				const text =
					node.type === 'text' || node.type === 'inlineMath' || node.type === 'math'
						? node.value
						: ' ';
				const remaining = SUMMARY_LENGTH - summary.length;
				summary += text.length >= remaining ? text.slice(0, remaining) : text;
				if (summary.length >= SUMMARY_LENGTH - 1) summaryComplete = true;
			}
		});

		file.data.summary = summary;
		file.data.wordCount = wordCount;
	};
}

function countWords(text: string): number {
	return text.split(/\s+/).filter(Boolean).length;
}
