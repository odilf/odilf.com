<script lang="ts">
	import { theme } from '#lib/theme.svelte.js';
	import iosevkaRegular from '../fonts/iosevka-400-normal.woff2?url';
	import './layout.css';
	import type { LayoutProps } from './$types';

	let { children }: LayoutProps = $props();

	$effect(() => {
		document.documentElement.dataset.theme = theme.current;
	});
</script>

<svelte:head>
	<link rel="preload" href={iosevkaRegular} as="font" type="font/woff2" crossorigin="anonymous" />
	<link rel="icon" href="/favicon.svg" />
	<link rel="icon" href="/favicon.png" />
	<link rel="alternate" type="application/rss+xml" title="RSS Feed" href="/blog/rss.xml" />
	<link rel="alternate" type="application/atom+xml" title="Atom Feed" href="/blog/atom.xml" />
</svelte:head>

<main class="py-4">
	{@render children()}
</main>
