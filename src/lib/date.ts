import { z } from 'zod';

const MONTHS = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];

export const Ymd = z
	.string()
	.regex(/^\d{4}-\d{2}-\d{2}$/, { message: 'Expected YYYY-MM-DD' })
	.brand<'YYYY-MM-DD'>();

type Ymd = z.infer<typeof Ymd>;

export const DateRange = z.union([Ymd, z.tuple([Ymd, Ymd]).refine(([a, b]) => a <= b)]);
export type DateRange = z.infer<typeof DateRange>;

// TODO: This should also just use `Date` API?
export function toIsoDate(value: unknown): string | null {
	if (value instanceof Date && !Number.isNaN(value.getTime())) {
		return value.toISOString().slice(0, 10);
	}

	if (typeof value === 'string') {
		const match = /^(\d{4}-\d{2}-\d{2})/.exec(value);
		if (match) return match[1];
	}

	return null;
}

// TODO: not in love with the name
function pad2(value: number): string {
	return String(value).padStart(2, '0');
}

// TODO: I feel this should not be this manual? Use `Date`
function parseYmd(ymd: string): { year: number, month: number, day: number } {
	const match = /^(\d{4})-(\d{2})-(\d{2})/.exec(ymd);
	if (!match) throw new Error(`Expected YYYY-MM-DD, got ${ymd}`);
	return { year: Number(match[1]), month: Number(match[2]), day: Number(match[3]) };
}

/** An arbitrary date in a way that is easily parsable by humans (as opposed to plain YYYY-MM-DD) */
export function formatHumanAbsoluteDate(iso: string): string {
	const { year, month, day } = parseYmd(iso);
	return `${pad2(day)} ${MONTHS[month - 1]}, ${year}`;
}

export function formatMonthYear(iso: string): string {
	const { year, month } = parseYmd(iso);
	return `${MONTHS[month - 1]} ${year}`;
}

export function formatMediaDate(date: DateRange): string {
	if (typeof date === 'string') return formatHumanAbsoluteDate(date);

	const [startIso, endIso] = date;
	const start = parseYmd(startIso);
	const end = parseYmd(endIso);
	const sameYear = start.year === end.year;
	const sameMonth = sameYear && start.month === end.month;

	let out = sameMonth ? pad2(start.day) : `${pad2(start.day)} ${MONTHS[start.month - 1]}`;
	if (!sameYear) out += `, ${start.year}`;

	return `${out} - ${pad2(end.day)} ${MONTHS[end.month - 1]}, ${end.year}`;
}

export function mediaDateTimestamp(date: DateRange): number {
	if (typeof date === 'string') return Date.parse(`${date}T00:00:00Z`);

	const [start, end] = date;
	const startTime = Date.parse(`${start}T00:00:00Z`);
	const endTime = Date.parse(`${end}T00:00:00Z`);
	return startTime + (endTime - startTime) / 2;
}

export function formatRating(rating: number): string {
	const full = Math.trunc(rating);
	return '★'.repeat(full) + (rating % 1 === 0 ? '' : '⯨');
}
