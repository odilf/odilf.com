<script lang="ts">
	import { theme, type Theme } from '#lib/theme.svelte.js';

	const options: { value: Theme; label: string }[] = [
		{ value: 'dark', label: 'Dark' },
		{ value: 'system', label: 'System' },
		{ value: 'light', label: 'Light' },
	];
</script>

<div class="flex flex-col opacity-80">
	<div class="text-bold text-center">Theme</div>
	<div class="flex gap-1 transition-all duration-200">
		{#each options as option (option.value)}
			<button
				class="w-[8ch] px-0 py-1 text-center text-xs"
				disabled={theme.current === option.value}
				title="{option.label} mode"
				onclick={() => theme.select(option.value)}
			>
				{option.label}
			</button>
		{/each}
	</div>
</div>
