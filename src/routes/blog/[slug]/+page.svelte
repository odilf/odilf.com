<script lang="ts">
	import Back from '#lib/components/Back.svelte';
	import Markdown from '#lib/components/Markdown.svelte';
	import Tag from '#lib/components/Tag.svelte';
	import { formatHumanAbsoluteDate } from '#lib/date.js';
	import { blogTags } from '..';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<!-- TODO: Is there a better way to bundle just the necessary katex or something like that? or prerender it, essentially. -->
<svelte:head>
	<link
		rel="stylesheet"
		href="https://cdn.jsdelivr.net/npm/katex@0.16.21/dist/katex.min.css"
		integrity="sha384-zh0CIslj+VczCZtlzBcjt5ppRcsAmDnRem7ESsYwWwg3m/OaJ2l4x7YBZl9Kxxib"
		crossorigin="anonymous"
	/>
</svelte:head>

<Back />

<h1>{data.entry.metadata.title}</h1>

<div class="mb-6 flex gap-2">
	{#if data.entry.metadata.date}
		<div class="font-light text-primary">{formatHumanAbsoluteDate(data.entry.metadata.date)}</div>
	{/if}
	<div class="flex-1"></div>
	{#each blogTags(data.entry) as tag (tag)}
		<Tag topic={tag} />
	{/each}
</div>

<div
	class="prose pb-8"
	class:numbered-headings={data.entry.metadata['numbered-headings']}
	lang={data.entry.metadata.lang === 'spanish' ? 'es' : 'en'}
>
	<Markdown html={data.entry.html} />
</div>
