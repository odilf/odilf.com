<script lang="ts">
	// KaTeX markup in `html` renders unstyled without its stylesheet.
	import 'katex/dist/katex.min.css';

	let { html }: { html: string } = $props();
</script>

<!-- eslint-disable-next-line svelte/no-at-html-tags -- rendered from trusted local markdown -->
{@html html}
