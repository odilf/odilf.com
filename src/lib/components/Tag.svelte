<script lang="ts">
	let { topic }: { topic: string } = $props();
</script>

<div
	class="h-fit w-fit content-center rounded-xs px-1 py-[1px] text-center text-xs whitespace-nowrap text-primary opacity-80 outline-1 outline-primary/50"
>
	{topic}
</div>
