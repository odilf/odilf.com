<script lang="ts">
	import { goto } from '$app/navigation';
	import Back from '#lib/components/Back.svelte';
	import Thumb from './Thumb.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	// TODO: Instead of rolling on click, we can roll on page load and just have the
	// destination determined (so we get, for instance, better preloading).
	//
	// Note however, that an `$effect` would work because that would prerender one
	// specific random choice, I think.
	function random() {
		const id = data.ids[Math.floor(Math.random() * data.ids.length)];
		goto('/pics/' + id + '/');
	}
</script>

<Back />

<h1>pics</h1>
<p class="faint pb-4">pictures I've taken.</p>

<button class="mx-auto block rounded-sm px-6 py-4 text-center text-2xl font-black" onclick={random}>
	random
</button>

<div class="grid grid-cols-2 gap-4 p-4 md:grid-cols-3 lg:grid-cols-4">
	{#each data.photos as photo (photo.id)}
		<Thumb {photo} />
	{/each}
</div>
