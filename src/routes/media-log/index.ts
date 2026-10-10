import { DateRange } from '#lib/date.js';
import { z } from 'zod';

export const MediaType = z.enum(['book', 'movie', 'videogame', 'music']);
export type MediaType = z.infer<typeof MediaType>;

export const MediaLogFrontmatter = z.object({
	title: z.string(),
	type: MediaType,
	rating: z.coerce.number(),
	date: DateRange,
	urls: z.array(z.coerce.string()).default([]),
	'image-url': z.string().optional(),
});
export type MediaLogFrontmatter = z.infer<typeof MediaLogFrontmatter>;

export interface MediaLog {
	slug: string;
	title: string;
	type: MediaType;
	rating: number;
	date: DateRange;
	urls: string[];
	review: string | null;
	imageUrl: string;
}
