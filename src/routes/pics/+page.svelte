<script lang="ts">
	import Back from '#lib/components/Back.svelte';
	import { pickRandom } from '#lib/random.js';
	import Thumb from './Thumb.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	// Must resolve in the browser; resolving during prerender would fix one
	// choice per build.
	function randomLink(node: HTMLAnchorElement) {
		const id = pickRandom(data.ids);
		if (id) node.href = `/pics/${id}/`;
	}
</script>

<Back />

<div class="mx-auto max-w-line-length px-4">
	<h1>pics</h1>
	<p class="faint pb-4">pictures I've taken.</p>

	<a
		class="mx-auto block rounded-sm px-6 py-4 text-center text-2xl font-black"
		href="/pics/"
		{@attach randomLink}
	>
		random
	</a>

	<div class="grid grid-cols-2 gap-4 p-4 md:grid-cols-3 lg:grid-cols-4">
		{#each data.photos as photo (photo.id)}
			<Thumb {photo} />
		{/each}
	</div>
</div>
