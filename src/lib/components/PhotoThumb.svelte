<script lang="ts">
	import { photoThumbPath, type Photo } from '#lib/types.js';

	let { photo }: { photo: Photo } = $props();
</script>

<a class="block aspect-square overflow-hidden" href="/pics/{photo.id}/">
	<img
		class="h-full w-full object-cover opacity-90 transition-opacity duration-50 hover:opacity-100"
		src={photoThumbPath(photo.id)}
		alt={photo.caption}
		loading="lazy"
	/>
</a>
