<script>
	import { page } from '$app/state';

	const target = $derived(page.url.pathname.split('/').slice(0, -1).join('/') || '/');
</script>

<div class="sticky top-0 right-0 left-0 z-10 bg-neutral opacity-90">
	<!-- eslint-disable svelte/no-useless-mustaches -->
	<a href={target}> {'<--'} (back)</a>
</div>
