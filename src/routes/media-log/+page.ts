import { mediaDateTimestamp } from '#lib/date.js';
import { mediaLogEntries } from '#lib/fixtures/index.js';

export function load() {
	const entries = [...mediaLogEntries].sort(
		(a, b) => mediaDateTimestamp(b.date) - mediaDateTimestamp(a.date),
	);

	return { entries };
}
