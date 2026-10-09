import { projects, projectsLastFetched } from '#lib/fixtures/index.js';

export function load() {
	return { projects, lastFetched: projectsLastFetched };
}
