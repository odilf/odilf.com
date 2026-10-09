<script lang="ts">
	import { goto } from '$app/navigation';
	import Back from '#lib/components/Back.svelte';
	import { photoPath } from '#lib/types.js';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let prevId = $derived(data.index > 0 ? data.ids[data.index - 1] : null);
	let nextId = $derived(data.index < data.ids.length - 1 ? data.ids[data.index + 1] : null);

	// TODO: We could (should?) re-use the random from the parent page.
	function random() {
		const filtered = data.ids.filter((id) => id !== data.photo.id);
		const id = filtered[Math.floor(Math.random() * filtered.length)];
		goto('/pics/' + id + '/');
	}
</script>

<Back />

<div class="flex flex-col items-center">
	<nav class="grid grid-cols-3 justify-center text-center">
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

	<img
		class="mt-8 max-h-[calc(100dvh-200px)] max-w-full"
		src={photoPath(data.photo.id)}
		alt={data.photo.caption}
	/>

	<div class="mt-4 flex items-center justify-center gap-6 text-center">
		<p class="text-primary-intense opacity-80">{data.photo.caption}</p>
		<p class="text-sm text-secondary opacity-50">{data.index + 1} / {data.ids.length}</p>
	</div>
</div>
