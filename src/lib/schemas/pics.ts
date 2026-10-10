import { z } from 'zod';

export const photoSchema = z.object({
	id: z.string(),
	caption: z.string(),
	filename: z.string(),
});
export type Photo = z.infer<typeof photoSchema>;

export const photosSchema = z.array(photoSchema);
