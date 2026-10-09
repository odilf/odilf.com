import type { Project } from '../types.js';

export const projectsLastFetched = '2025-08-10T17:50:55.535359Z';

export const projects: Record<string, Project> = {
	gazpacho: {
		description: 'Good video editing for all',
		sourceCodeUrl: 'https://github.com/odilf/gazpacho',
		websiteUrl: null,
		documentationUrl: null,
		creationDate: '2026-03-31T00:00:00Z',
		lastUpdate: '2026-08-29T00:00:00Z',
		imageUrl: null,
		language: 'Rust',
		topics: [],
	},
	'odilf.com': {
		description: "Source code for the website you're currently viewing",
		sourceCodeUrl: 'https://github.com/odilf/odilf.com',
		websiteUrl: 'http://www.odilf.com/',
		documentationUrl: null,
		creationDate: '2021-10-28T21:37:08Z',
		lastUpdate: '2025-08-08T15:49:52Z',
		imageUrl: null,
		language: 'Rust',
		topics: ['homepage', 'personal'],
	},
	norcina: {
		description: "Rubik's cube library and timer",
		sourceCodeUrl: 'https://github.com/odilf/norcina',
		websiteUrl: null,
		documentationUrl: null,
		creationDate: '2025-07-03T17:19:06Z',
		lastUpdate: '2025-07-06T22:16:13Z',
		imageUrl: null,
		language: 'Rust',
		topics: [],
	},
};
