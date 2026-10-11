<script lang="ts">
	import { goto } from '$app/navigation';
	import Back from '#lib/components/Back.svelte';
	import { fullImage } from '#lib/images.js';
	import { pickRandom } from '#lib/random.js';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	const image = $derived(fullImage(data.photo.id));
	const imageRatio = $derived(image ? (image.img.w / image.img.h).toFixed(4) : '1');

	let prevId = $derived(data.index > 0 ? data.ids[data.index - 1] : null);
	let nextId = $derived(data.index < data.ids.length - 1 ? data.ids[data.index + 1] : null);

	function random() {
		const id = pickRandom(data.ids, data.photo.id);
		if (id) goto('/pics/' + id + '/');
	}
</script>

<Back />

<div class="flex flex-col items-center px-4">
	<nav class="grid w-full max-w-md grid-cols-3 justify-center text-center">
		{#if prevId}
			<a class="opacity-70 hover:opacity-100" href="/pics/{prevId}/">&lt;- prev</a>
		{:else}
			<div></div>
		{/if}
		<button
			class="mx-4 bg-neutral px-0 text-secondary opacity-70 hover:opacity-100"
			onclick={random}
		>
			rand
		</button>
		{#if nextId}
			<a class="opacity-70 hover:opacity-100" href="/pics/{nextId}/">next -></a>
		{/if}
	</nav>

	{#if image}
		<enhanced:img
			class="mt-8 h-auto max-h-[calc(100dvh-200px)] w-auto max-w-full"
			src={image}
			alt={data.photo.caption}
			sizes="min(calc(100vw - 2rem), calc((100dvh - 200px) * {imageRatio}))"
			fetchpriority="high"
		/>
	{/if}

	<div class="mt-4 flex max-w-line-length items-center justify-center gap-6 text-center">
		<p class="text-primary-intense opacity-80">{data.photo.caption}</p>
		<p class="text-sm text-secondary opacity-50">{data.index + 1} / {data.ids.length}</p>
	</div>
</div>
