<script lang="ts">
	import { thumbImage } from '#lib/images.js';
	import type { Photo } from '#lib/types.js';

	let { photo }: { photo: Photo } = $props();

	const image = $derived(thumbImage(photo.id));
</script>

<a class="block aspect-square overflow-hidden" href="/pics/{photo.id}/">
	{#if image}
		<enhanced:img
			class="h-full w-full object-cover opacity-90 transition-opacity duration-50 hover:opacity-100"
			src={image}
			alt={photo.caption}
			sizes="(min-width: 1024px) 25vw, (min-width: 768px) 33vw, 50vw"
			loading="lazy"
		/>
	{/if}
</a>
