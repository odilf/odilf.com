<script lang="ts">
	import Tag from '#lib/components/Tag.svelte';
	import { formatBlogDate } from '#lib/date.js';
	import type { BlogEntry } from '#lib/schemas/blog.js';

	let { entry }: { entry: BlogEntry } = $props();

	let tags = $derived([...entry.metadata.topics, `${(entry.wordCount / 1000).toFixed(1)}k words`]);
	let half = $derived(Math.ceil(tags.length / 2));
</script>

<a href="/blog/{entry.slug}/" class="block" class:opacity-50={entry.metadata.draft !== false}>
	<div class="flex">
		<div class="pr-[1ch] text-lg text-primary">></div>
		<div class="flex-1 text-lg font-bold text-balance">{entry.metadata.title}</div>
		{#if entry.metadata.date}
			<div class="font-light text-primary">{formatBlogDate(entry.metadata.date)}</div>
		{/if}
	</div>

	<div class="no-no-underline flex gap-2 text-primary">
		<div class="line-clamp-2 flex-1 text-sm text-ellipsis opacity-50">{entry.summary}</div>

		<div class="no-no-underline grid gap-1">
			<div class="flex justify-evenly gap-1">
				{#each tags.slice(0, half) as tag, index (index)}
					<Tag topic={tag} />
				{/each}
			</div>
			<div class="flex justify-evenly gap-1">
				{#each tags.slice(half) as tag, index (index)}
					<Tag topic={tag} />
				{/each}
			</div>
		</div>
	</div>
</a>
