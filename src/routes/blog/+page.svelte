<script lang="ts">
	import Back from '#lib/components/Back.svelte';
	import Card from './Card.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	const topics = ['all', 'dev', 'personal'] as const;

	let topic = $state<(typeof topics)[number]>('all');

	let visible = $derived(
		topic === 'all'
			? data.entries
			: data.entries.filter((entry) => entry.metadata.topics.includes(topic)),
	);
</script>

<Back />

<h1>blog</h1>
<p class="faint pb-4">some thoughts, stories and reflections from throughout the years.</p>

<div class="mb-4 flex gap-2">
	{#each topics as option (option)}
		<button disabled={topic === option} onclick={() => (topic = option)}>
			{option}
		</button>
	{/each}
</div>

<ul>
	{#each visible as entry (entry.slug)}
		<li class="mb-4"><Card {entry} /></li>
	{/each}
</ul>
