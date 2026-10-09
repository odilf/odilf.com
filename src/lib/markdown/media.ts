import type { MediaLogFrontmatter } from '../types.js';

export function parseMediaMetadata(content: string): MediaLogFrontmatter {
	throw new Error(`parseMediaMetadata is not implemented (received ${content.length} characters)`);
}

export function renderMediaReview(content: string): string {
	throw new Error(`renderMediaReview is not implemented (received ${content.length} characters)`);
}
