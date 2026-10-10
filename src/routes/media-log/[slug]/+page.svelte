<script lang="ts">
	import Back from '#lib/components/Back.svelte';
	import Markdown from '#lib/components/Markdown.svelte';
	import Rating from '../Rating.svelte';
	import { formatMediaDate } from '#lib/date.js';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<Back />

<h1>{data.entry.title}</h1>

<div class="flex gap-2">
	<div class="flex-1">
		<div class="flex justify-between">
			<div class="text-2xl text-primary"><Rating rating={data.entry.rating} /></div>
			<div class="faint text-tertiary">{formatMediaDate(data.entry.date)}</div>
		</div>

		<div class="prose faint text-primary">
			{#if data.entry.review}
				<Markdown html={data.entry.review} />
			{/if}
		</div>
	</div>

	<img class="w-[30%] object-contain" src={data.entry.imageUrl} alt={data.entry.title} />
</div>
