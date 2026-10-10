import { z } from 'zod';
import { toIsoDate } from '../date.js';

export const languageSchema = z.enum(['english', 'spanish']);
export type Language = z.infer<typeof languageSchema>;

export const blogMetadataSchema = z
	.object({
		title: z.string(),
		date: z.unknown().transform(toIsoDate),
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
export type BlogMetadata = z.infer<typeof blogMetadataSchema>;

export interface BlogEntry {
	slug: string;
	html: string;
	summary: string;
	wordCount: number;
	metadata: BlogMetadata;
}
