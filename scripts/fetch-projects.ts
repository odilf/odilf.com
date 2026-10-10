import { writeFile } from 'node:fs/promises';
import * as toml from 'smol-toml';
import { z } from 'zod';
import type { ProjectFileEntry, ProjectsFile } from '#lib/schemas/projects.js';

const PERSONAL_PROJECTS = [
	'identity-crisis',
	'norcina',
	'dotfiles',
	'churri',
	'odilf.com',
	'FAM',
	'smt-guidance-experiments',
	'bachelor-thesis',
	'chessagon',
	'sentouki',
	'quantum-key-exchange',
	'indecision',
	'hext-boards',
	'incipit',
	'elvish',
	'pink',
	'barbarosa',
	'modern-docs-rs',
	'fosbury',
	'daedalus',
	'Pistol',
	'Dveco',
	'Minigames',
	'spasmodic-consonizer',
	'rummy',
	'baselga-timer',
];

const githubRepoSchema = z.object({
	name: z.string(),
	html_url: z.string(),
	description: z.string().nullish(),
	homepage: z.string().nullish(),
	created_at: z.string(),
	pushed_at: z.string(),
	language: z.string().nullish(),
	topics: z.array(z.string()).nullish(),
});

const token = process.env.GITHUB_TOKEN;
if (!token) throw new Error('GITHUB_TOKEN is not set');

const projects: Record<string, ProjectFileEntry> = {};

for (const name of PERSONAL_PROJECTS) {
	const response = await fetch(`https://api.github.com/repos/odilf/${name}`, {
		headers: { authorization: `Bearer ${token}`, 'user-agent': 'odilf.com fetch' },
	});
	if (!response.ok) throw new Error(`Failed to fetch ${name}: ${response.status}`);

	const repo = githubRepoSchema.parse(await response.json());
	const entry: ProjectFileEntry = {
		description: repo.description ?? '',
		source_code_url: repo.html_url,
		creation_date: repo.created_at,
		last_update: repo.pushed_at,
		topics: repo.topics ?? [],
	};
	if (repo.homepage) entry.website_url = repo.homepage;
	if (repo.language) entry.language = repo.language;

	projects[repo.name] = entry;
	console.log(`Fetched ${repo.name}`);
}

const file: ProjectsFile = { last_fetched: new Date().toISOString(), projects };
await writeFile('projects-fetched.toml', toml.stringify(file));
console.log(`Wrote projects-fetched.toml (${Object.keys(projects).length} projects)`);
