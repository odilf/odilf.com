<script lang="ts">
	import Back from '#lib/components/Back.svelte';
	import Card from './Card.svelte';
	import type { MediaType } from '#lib/types.js';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	type TypeFilter = 'all' | MediaType;

	const types: { value: TypeFilter; label: string }[] = [
		{ value: 'all', label: 'all' },
		{ value: 'book', label: 'books' },
		{ value: 'movie', label: 'movies' },
		{ value: 'music', label: 'music' },
		{ value: 'videogame', label: 'videogames' },
	];

	let type = $state<TypeFilter>('all');
	let minRating = $state(0);

	let visible = $derived(
		data.entries.filter(
			(entry) => (type === 'all' || entry.type === type) && entry.rating >= minRating,
		),
	);

	function toggleRating(value: number) {
		minRating = minRating === value ? 0 : value;
	}
</script>

<Back />

<h1>media log</h1>
<p class="faint pb-2">
	logging and reviews of books, movies and videogames.
	<span class="opacity-50">
		(also available as <a href="/static/media-log.json">JSON</a>)
	</span>
</p>

<div class="mb-4 flex gap-2">
	{#each types as option (option.value)}
		<button disabled={type === option.value} onclick={() => (type = option.value)}>
			{option.label}
		</button>
	{/each}

	<div class="flex-1"></div>

	{#each [1, 2, 3, 4, 5] as star (star)}
		<button
			class="star-btn bg-neutral px-0 text-2xl text-primary transition hover:text-secondary"
			class:text-primary-intense={star <= minRating}
			aria-label="{star} star{star === 1 ? '' : 's'} and up"
			onclick={() => toggleRating(star)}
		>
			★
		</button>
	{/each}
</div>

<ul>
	{#each visible as entry (entry.slug)}
		<li class="mb-4"><Card {entry} /></li>
	{/each}
</ul>
