import { z } from 'zod';

export const projectFileEntrySchema = z.object({
	description: z.string(),
	source_code_url: z.string(),
	website_url: z.string().optional(),
	documentation_url: z.string().optional(),
	creation_date: z.string(),
	last_update: z.string(),
	image_url: z.string().optional(),
	language: z.string().optional(),
	topics: z.array(z.string()).optional(),
});
export type ProjectFileEntry = z.infer<typeof projectFileEntrySchema>;

export const projectsFileSchema = z.object({
	last_fetched: z.string(),
	projects: z.record(z.string(), projectFileEntrySchema),
});
export type ProjectsFile = z.infer<typeof projectsFileSchema>;

export interface Project {
	description: string;
	sourceCodeUrl: string;
	websiteUrl: string | null;
	documentationUrl: string | null;
	creationDate: string;
	lastUpdate: string;
	imageUrl: string | null;
	language: string | null;
	topics: string[];
}

export interface Projects {
	projects: Record<string, Project>;
	lastFetched: string;
}
