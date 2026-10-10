import { Ymd } from '#lib/date.js';
import { z } from 'zod';

export const Language = z.enum(['english', 'spanish']);
export type Language = z.infer<typeof Language>;

export const BlogMetadata = z
	.object({
		title: z.string(),
		date: Ymd,
		draft: z
			.boolean()
			.nullish()
			.transform((value) => value ?? null),
		topics: z.array(z.coerce.string()).default([]),
		lang: z
			.string()
			.optional()
			.transform((value): Language => (value?.toLowerCase() === 'spanish' ? 'spanish' : 'english')),
		'numbered-headings': z.boolean().default(true),
	})
	.transform(({ 'numbered-headings': numberedHeadings, ...metadata }) => ({
		...metadata,
		numberedHeadings,
	}));

export type BlogMetadata = z.infer<typeof BlogMetadata>;

export interface BlogEntry {
	slug: string;
	html: string;
	summary: string;
	wordCount: number;
	metadata: BlogMetadata;
}
