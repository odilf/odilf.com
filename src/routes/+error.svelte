<script lang="ts">
	import { page } from '$app/state';
</script>

<div class="mx-auto max-w-line-length px-4">
	<h1>{page.status}</h1>
	<p>{page.error?.message ?? 'Something went wrong.'}</p>
	<p><a href="/">go home</a></p>
</div>
