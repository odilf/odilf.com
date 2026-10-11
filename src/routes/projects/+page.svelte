<script lang="ts">
	import Back from '#lib/components/Back.svelte';
	import Card from './Card.svelte';
	import { formatHumanAbsoluteDate } from '#lib/date.js';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<Back />

<div class="mx-auto max-w-line-length px-4">
	<h1>programming projects</h1>

	<p class="pb-2">most of my personal coding projects.</p>
	<p>
		Feel free to check out my
		<a href="https://github.com/odilf" target="_blank" rel="noopener noreferrer">GitHub</a>, my
		<a href="https://git-archive.odilf.com" target="_blank" rel="noopener noreferrer"
			>personal git
		</a>
		and my
		<a href="https://tangled.org/odilf.com" target="_blank" rel="noopener noreferrer">tangled</a>
		for a more comprehensive list.
	</p>

	<ul class="mt-8 grid grid-cols-1 gap-3">
		{#each Object.entries(data.projects) as [name, project] (name)}
			<Card {name} {project} />
		{/each}
	</ul>

	<p class="faint mt-8 text-xs">Last updated: {formatHumanAbsoluteDate(data.lastFetched)}</p>
</div>
