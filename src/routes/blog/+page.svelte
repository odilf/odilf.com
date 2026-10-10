<script lang="ts">
	import Back from '#lib/components/Back.svelte';
	import BlogEntryCard from '#lib/components/BlogEntryCard.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	type TopicFilter = 'all' | 'dev' | 'personal';

	const topics: { value: TopicFilter; label: string }[] = [
		{ value: 'all', label: 'all' },
		{ value: 'dev', label: 'development' },
		{ value: 'personal', label: 'personal' },
	];

	let topic = $state<TopicFilter>('all');

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
	{#each topics as option (option.value)}
		<button disabled={topic === option.value} onclick={() => (topic = option.value)}>
			{option.label}
		</button>
	{/each}
</div>

<ul>
	{#each visible as entry (entry.slug)}
		<li class="mb-4"><BlogEntryCard {entry} /></li>
	{/each}
</ul>
