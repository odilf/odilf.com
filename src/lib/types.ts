export type Language = 'english' | 'spanish';

export interface BlogMetadata {
	title: string;
	date: string | null;
	draft: boolean | null;
	topics: string[];
	lang: Language;
	numberedHeadings: boolean;
}

export interface BlogEntry {
	slug: string;
	html: string;
	summary: string;
	wordCount: number;
	metadata: BlogMetadata;
}

export interface BlogFrontmatter {
	title: string;
	date?: string;
	draft?: boolean;
	topics?: string[];
	lang?: Language;
	'numbered-headings'?: boolean;
}

export type MediaType = 'book' | 'movie' | 'videogame' | 'music';

export type MediaDate = string | [string, string];

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

export interface MediaLogFrontmatter {
	title: string;
	type: MediaType;
	rating: number;
	date: MediaDate;
	urls: string[];
	'image-url'?: string;
}

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

export interface ProjectFileEntry {
	description: string;
	source_code_url: string;
	website_url?: string;
	documentation_url?: string;
	creation_date: string;
	last_update: string;
	image_url?: string;
	language?: string;
	topics?: string[];
}

export interface ProjectsFile {
	last_fetched: string;
	projects: Record<string, ProjectFileEntry>;
}

export interface Photo {
	id: string;
	caption: string;
	filename: string;
}

export function projectMainLink(project: Project): string {
	return project.websiteUrl ?? project.sourceCodeUrl;
}
