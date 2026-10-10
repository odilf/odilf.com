import { readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { parse } from 'smol-toml';
import { projectsFileSchema, type Project, type Projects } from '#lib/schemas/projects.js';

const PROJECTS_FILE = join(process.cwd(), 'projects.toml');

export async function loadProjects(): Promise<Projects> {
	const file = projectsFileSchema.parse(parse(await readFile(PROJECTS_FILE, 'utf8')));

	const projects: Record<string, Project> = {};
	for (const [name, entry] of Object.entries(file.projects)) {
		projects[name] = {
			description: entry.description,
			sourceCodeUrl: entry.source_code_url,
			websiteUrl: entry.website_url ?? null,
			documentationUrl: entry.documentation_url ?? null,
			creationDate: entry.creation_date,
			lastUpdate: entry.last_update,
			imageUrl: entry.image_url ?? null,
			language: entry.language ?? null,
			topics: entry.topics ?? [],
		};
	}

	return { projects, lastFetched: file.last_fetched };
}
