import { Data } from '#lib';
import { Ymd } from '#lib/date.js';
import { z } from 'zod';

export const Language = z.enum(['english', 'spanish']);
export type Language = z.infer<typeof Language>;

export const BlogMetadata = z
	.object({
		title: z.string(),
		date: Ymd,
		draft: z
			.unknown()
			.transform(x => x === false ? false : true),
		topics: z.array(z.coerce.string()).default([]),
		lang: z
			.string()
			.optional()
			.transform((value): Language => (value?.toLowerCase() === 'spanish' ? 'spanish' : 'english')),
		'numbered-headings': z.boolean().default(true),
	});

export type BlogMetadata = z.infer<typeof BlogMetadata>;

export interface BlogEntry {
	slug: string;
	html: string;
	summary: string;
	wordCount: number;
	metadata: BlogMetadata;
}

export function blogTags({ metadata: { topics }, wordCount }: BlogEntry): string[] {
	return [...topics, `${(wordCount / 1000).toFixed(1)}k words`]
}
