<script lang="ts">
	import { on } from 'svelte/events';
	import { theme, type Theme } from '#lib/theme.svelte.js';

	const options: { value: Theme; label: string; glyph: string }[] = [
		{ value: 'dark', label: 'Dark', glyph: '●' },
		{ value: 'system', label: 'System', glyph: '◐' },
		{ value: 'light', label: 'Light', glyph: '○' },
	];

	let open = $state(false);
	const current = $derived(options.find((option) => option.value === theme.current) ?? options[0]);

	function select(value: Theme) {
		theme.select(value);
		open = false;
	}
</script>

<div
	class="relative w-fit"
	{@attach (element: Element) => {
		if (!open) return;

		const focusTrigger = () => element.querySelector('button')?.focus();

		const removePointerListener = on(window, 'pointerdown', (event) => {
			if (!element.contains(event.target as Node)) open = false;
		});
		const removeKeyListener = on(window, 'keydown', (event) => {
			if (event.key !== 'Escape') return;
			open = false;
			focusTrigger();
		});

		return () => {
			removePointerListener();
			removeKeyListener();
		};
	}}
>
	<button
		type="button"
		class=""
		aria-expanded={open}
		aria-controls="theme-options"
		onclick={() => (open = !open)}
	>
		<span class="w-[2ch]" aria-hidden="true">{current.glyph}</span>
		{current.label}
	</button>

	{#if open}
		<ul id="theme-options" class="absolute right-0 z-10">
			{#each options as option (option.value)}
				<li>
					<button
						type="button"
						class="flex w-full items-center py-1 text-center text-xs leading-none"
						disabled={option.value === theme.current}
						onclick={() => select(option.value)}
					>
						<span class="w-[2ch]" aria-hidden="true">{option.glyph}</span>
						{option.label}
					</button>
				</li>
			{/each}
		</ul>
	{/if}
</div>
