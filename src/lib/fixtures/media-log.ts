import type { MediaLog } from '../types.js';

export const mediaLogEntries: MediaLog[] = [
	{
		slug: 'mistborn',
		title: 'Mistborn: The final empire',
		type: 'book',
		rating: 5,
		date: ['2025-12-31', '2026-01-17'],
		urls: ['https://en.wikipedia.org/wiki/Mistborn:_The_Final_Empire'],
		review:
			"<p>It's excellent. Vin might be one of my favorite protagonists in any story ever.</p>",
		imageUrl:
			'https://upload.wikimedia.org/wikipedia/en/thumb/8/8f/Mistborn_-_The_Final_Empire.jpg/220px-Mistborn_-_The_Final_Empire.jpg',
	},
	{
		slug: 'animal-farm',
		title: 'Animal Farm',
		type: 'book',
		rating: 2,
		date: '2021-09-10',
		urls: ['https://en.wikipedia.org/wiki/Animal_Farm'],
		review: null,
		imageUrl:
			'https://upload.wikimedia.org/wikipedia/commons/thumb/f/fb/Animal_Farm_-_1st_edition.jpg/220px-Animal_Farm_-_1st_edition.jpg',
	},
	{
		slug: 'across-the-spiderverse',
		title: 'Across the Spider-Verse',
		type: 'movie',
		rating: 4.5,
		date: '2023-06-04',
		urls: ['https://en.wikipedia.org/wiki/Spider-Man:_Across_the_Spider-Verse'],
		review: '<p>A visual feast from start to finish.</p>',
		imageUrl:
			'https://upload.wikimedia.org/wikipedia/en/thumb/b/b4/Spider-Man_Across_the_Spider-Verse_poster.jpg/220px-Spider-Man_Across_the_Spider-Verse_poster.jpg',
	},
];
