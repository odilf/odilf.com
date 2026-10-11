<script lang="ts">
	import ThemeSelector from '#lib/components/ThemeSelector.svelte';

	const pages = {
		blog: 'some thoughts, stories and reflections from throughout the years.',
		pics: "pictures I've taken.",
		'media log': 'logging and reviews of books, movies and videogames.',
		projects: 'most of my personal coding projects.',
		about: 'information about me and CV',
	} as const;
</script>

<div class="mx-auto max-w-line-length px-4">
	<div class="flex">
		<h1 class="flex-1">hi, i'm Ody 👋</h1>
		<ThemeSelector />
	</div>

	<p class="faint mb-4">go ahead and take a look at what's here ^^</p>

	<ol class="flex flex-col gap-2">
		{#each Object.entries(pages) as [label, desc] (label)}
			<li class="text-2xl text-secondary hover:underline">
				<a href="/{label.split(' ').join('-')}">
					<p class="font-semibold">> {label}</p>
					<p class="text-lg text-primary-soft">{desc}</p>
				</a>
			</li>
		{/each}
	</ol>
</div>
