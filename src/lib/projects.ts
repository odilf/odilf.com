import type { Project } from './schemas/projects.js';

export function projectMainLink(project: Project): string {
	return project.websiteUrl ?? project.sourceCodeUrl;
}
