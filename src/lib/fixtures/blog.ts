import type { BlogEntry } from '../types.js';

export const blogEntries: BlogEntry[] = [
	{
		slug: 'the-declarativeness-ladder',
		html: '<p>One way to classify programming paradigms is whether it is <em>imperative</em> or <em>declarative</em>.</p>',
		summary:
			'One way to classify programming paradigms is whether it is imperative or declarative. In an imperative setting you tell the computer how to do something.',
		wordCount: 1240,
		metadata: {
			title: 'The declarativeness ladder',
			date: '2025-03-23',
			draft: false,
			topics: ['musings', 'dev'],
			lang: 'english',
			numberedHeadings: false,
		},
	},
	{
		slug: 'abstraction-is-bad',
		html: '<p>This is not a very deep or new idea, but I like to spell it out as so:</p>',
		summary:
			'This is not a very deep or new idea, but I like to spell it out as so: "Abstraction", by itself, as a word, is bad.',
		wordCount: 890,
		metadata: {
			title: "Abstraction is bad (so it'd better be justified)",
			date: '2024-09-01',
			draft: false,
			topics: ['dev', 'musings'],
			lang: 'english',
			numberedHeadings: true,
		},
	},
	{
		slug: 'websites-are-so-cool',
		html: '<p>I saw this tweet. It says we used to build websites.</p>',
		summary: 'I saw this tweet. It says we used to build websites, and now we build pipelines.',
		wordCount: 120,
		metadata: {
			title: '"Websites are just JavaScript complexity" (websites are so cool)',
			date: '2025-06-20',
			draft: true,
			topics: [],
			lang: 'english',
			numberedHeadings: true,
		},
	},
];
