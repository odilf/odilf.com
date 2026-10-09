import type { BlogFrontmatter } from '../types.js';

export interface RenderedMarkdown {
	html: string;
	summary: string;
	wordCount: number;
	referencedLinks: string[];
}

export function parseBlogMetadata(content: string): BlogFrontmatter | null {
	throw new Error(`parseBlogMetadata is not implemented (received ${content.length} characters)`);
}

export function renderBlogMarkdown(content: string): RenderedMarkdown {
	throw new Error(`renderBlogMarkdown is not implemented (received ${content.length} characters)`);
}
