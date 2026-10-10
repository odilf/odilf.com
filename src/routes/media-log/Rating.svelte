<script lang="ts">
	import { formatRating } from '#lib/date.js';

	let { rating }: { rating: number } = $props();
</script>

<div
	class="w-[10ch] text-2xl text-primary-intense"
	class:glow={rating > 4}
	class:glow-intense={rating > 4.5}
>
	{formatRating(rating)}
</div>
