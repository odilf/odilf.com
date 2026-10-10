<script lang="ts">
	import Markdown from '#lib/components/Markdown.svelte';
	import Rating from './Rating.svelte';
	import { formatMediaDate } from '#lib/date.js';
	import type { MediaLog } from '.';

	let { entry }: { entry: MediaLog } = $props();
</script>

<a href="/media-log/{entry.slug}/" class="block">
	<div class="flex gap-2">
		<div class="flex-1">
			<div class="flex w-[calc(100%+2ch)] translate-x-[-2ch] gap-[1ch] text-2xl">
				<div class="text-primary">></div>
				<div class="flex-1 font-black">{entry.title}</div>
			</div>

			<div class="flex">
				<div class="font-light text-primary">{formatMediaDate(entry.date)}</div>
				<div class="faint pl-[1ch] text-tertiary">({entry.type})</div>
				<div class="flex-1"></div>
				<Rating rating={entry.rating} />
			</div>

			<div class="no-no-underline faint text-primary">
				<div class="prose line-clamp-3 h-min flex-1 text-sm text-ellipsis opacity-50">
					{#if entry.review}
						<Markdown html={entry.review} />
					{/if}
				</div>
			</div>
		</div>

		<img class="h-full w-[30%]" src={entry.imageUrl} alt="Cover of {entry.title}" />
	</div>
</a>
