import { blogEntries } from '#lib/fixtures/index.js';

export function load() {
	const entries = [...blogEntries].sort((a, b) =>
		(b.metadata.date ?? '').localeCompare(a.metadata.date ?? ''),
	);

	return { entries };
}
