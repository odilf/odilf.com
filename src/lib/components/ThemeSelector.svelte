<script lang="ts">
	import { on } from 'svelte/events';
	import { theme, type Theme } from '#lib/theme.svelte.js';

	/** Glyphs must exist in Iosevka; `pnpm subset:fonts` only keeps codepoints found in source. */
	const options: { value: Theme; label: string; glyph: string }[] = [
		{ value: 'dark', label: 'Dark', glyph: '●' },
		{ value: 'system', label: 'System', glyph: '◐' },
		{ value: 'light', label: 'Light', glyph: '○' },
	];

	let open = $state(false);

	const current = $derived(options.find((option) => option.value === theme.current) ?? options[0]);

	/** @type {import('svelte/attachments').Attachment} */
	function dismissWhileOpen(element: Element) {
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
	}

	function select(value: Theme) {
		theme.select(value);
		open = false;
	}
</script>

<div class="relative w-fit" {@attach dismissWhileOpen}>
	<button
		type="button"
		class="flex items-center gap-2 rounded-xs px-2 py-[1px] text-xs leading-none opacity-80 outline-1 outline-primary/50 hover:opacity-100"
		aria-expanded={open}
		aria-controls="theme-options"
		onclick={() => (open = !open)}
	>
		<span class="w-[2ch]" aria-hidden="true">{current.glyph}</span>
		{current.label}
	</button>

	{#if open}
		<ul
			id="theme-options"
			class="absolute right-0 z-10 mt-1 flex w-max flex-col gap-1 rounded-xs bg-neutral p-1 outline-1 outline-primary/50"
		>
			{#each options as option (option.value)}
				<li>
					<button
						type="button"
						class="flex w-full items-center gap-2 rounded-xs px-2 py-[1px] text-left text-xs leading-none whitespace-nowrap opacity-80 outline-1 outline-primary/50 hover:opacity-100"
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
