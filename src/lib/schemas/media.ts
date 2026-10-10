import { z } from 'zod';
import { toIsoDate } from '../date.js';

export const mediaTypeSchema = z.enum(['book', 'movie', 'videogame', 'music']);
export type MediaType = z.infer<typeof mediaTypeSchema>;

export const mediaDateSchema = z.union([z.string(), z.tuple([z.string(), z.string()])]);
export type MediaDate = z.infer<typeof mediaDateSchema>;

const mediaDateFromFrontmatter = z.unknown().transform((value): MediaDate => {
	if (Array.isArray(value)) {
		const start = toIsoDate(value[0]);
		const end = toIsoDate(value[1]);
		if (start && end) return [start, end];
	}

	const single = toIsoDate(value);
	if (single) return single;

	throw new Error('Invalid media date');
});

export const mediaLogFrontmatterSchema = z.object({
	title: z.string(),
	type: mediaTypeSchema,
	rating: z.coerce.number(),
	date: mediaDateFromFrontmatter,
	urls: z.array(z.coerce.string()).default([]),
	'image-url': z.string().optional(),
});
export type MediaLogFrontmatter = z.infer<typeof mediaLogFrontmatterSchema>;

export interface MediaLog {
	slug: string;
	title: string;
	type: MediaType;
	rating: number;
	date: MediaDate;
	urls: string[];
	review: string | null;
	imageUrl: string;
}
