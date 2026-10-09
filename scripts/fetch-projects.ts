import { writeFile } from 'node:fs/promises';
import * as toml from 'smol-toml';
import type { ProjectFileEntry, ProjectsFile } from '#lib/types.js';

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

interface GithubRepo {
	name: string;
	html_url: string;
	description: string | null;
	homepage: string | null;
	created_at: string;
	pushed_at: string;
	language: string | null;
	topics: string[] | null;
}

const token = process.env.GITHUB_TOKEN;
if (!token) throw new Error('GITHUB_TOKEN is not set');

const projects: Record<string, ProjectFileEntry> = {};

for (const name of PERSONAL_PROJECTS) {
	const response = await fetch(`https://api.github.com/repos/odilf/${name}`, {
		headers: { authorization: `Bearer ${token}`, 'user-agent': 'odilf.com fetch' },
	});
	if (!response.ok) throw new Error(`Failed to fetch ${name}: ${response.status}`);

	// TODO(zod)
	const repo = (await response.json()) as GithubRepo;
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
