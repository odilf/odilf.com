<script lang="ts">
	import ThemeToggle from '#lib/components/ThemeToggle.svelte';

	const links = [
		{
			href: '/blog/',
			label: 'blog',
			description: 'some thoughts, stories and reflections from throughout the years.',
		},
		{ href: '/pics/', label: 'pics', description: "pictures I've taken." },
		{
			href: '/media-log/',
			label: 'media log',
			description: 'logging and reviews of books, movies and videogames.',
		},
		{ href: '/projects/', label: 'projects', description: 'most of my personal coding projects.' },
		{ href: '/about/', label: 'about', description: 'information about me and CV' },
	];
</script>

<div class="flex">
	<h1 class="flex-1">hi, i'm Ody 👋</h1>
	<ThemeToggle />
</div>

<p class="faint mb-4">go ahead and take a look at what's here ^^</p>

<ol class="flex flex-col gap-2">
	{#each links as link (link.href)}
		<li class="text-xl text-secondary hover:underline">
			<a href={link.href}>
				<p>> {link.label}</p>
				<p class="text-sm text-primary-soft">{link.description}</p>
			</a>
		</li>
	{/each}
</ol>
