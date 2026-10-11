<script lang="ts">
	import Tag from '#lib/components/Tag.svelte';
	import { formatMonthYear } from '#lib/date.js';
	import { projectMainLink } from '#lib/projects.js';
	import type { Project } from '#lib/schemas/projects.js';

	let { name, project }: { name: string; project: Project } = $props();
</script>

{#snippet link(url: string, arrow: string, label: string)}
	<a
		class="group/link inline-block w-full translate-x-[-1.5ch] text-inherit transition-[translate] hover:translate-x-[-1ch]"
		href={url}
		target="_blank"
		rel="noopener noreferrer"
	>
		<span class="mr-[1ch] opacity-80 transition-opacity group-hover/link:opacity-100"
			>{arrow}
		</span>{label}
	</a>
{/snippet}

<li
	class="flex justify-between gap-2 rounded-sm px-2 py-6 text-primary-soft opacity-80 transition hover:opacity-100 dark:opacity-90"
	style:--image-url="url({project.imageUrl ?? ''})"
	class:image={project.imageUrl}
>
	<div class="flex-1">
		<h2 class="w-full pt-0 text-xl font-bold text-balance text-secondary">
			<a href={projectMainLink(project)} target="_blank" rel="noopener noreferrer">
				<span class="mr-[1ch] opacity-50">> </span>{name}
			</a>
		</h2>

		<p class="text-lg text-balance text-primary">{project.description}</p>

		<ul class="text-tertiary">
			{#if project.websiteUrl}
				<li>{@render link(project.websiteUrl, '->>', 'Website')}</li>
			{/if}
			<li>{@render link(project.sourceCodeUrl, '~>>', 'Source code')}</li>
			{#if project.documentationUrl}
				<li>{@render link(project.documentationUrl, '-|>', 'Documentation')}</li>
			{/if}
		</ul>
	</div>

	<div class="flex min-w-[17ch] flex-col items-end text-right">
		{#if project.language}
			<p class="text-primary-intense">{project.language}</p>
		{/if}
		<p class="text-primary-faint faint">Created: {formatMonthYear(project.creationDate)}</p>
		<p class="text-primary-faint faint">Updated: {formatMonthYear(project.lastUpdate)}</p>
		<ul class="grid w-fit grid-flow-col grid-rows-2 place-items-center justify-end gap-1">
			{#each project.topics as topic (topic)}
				<li class="w-fit"><Tag {topic} /></li>
			{/each}
		</ul>
	</div>
</li>

<style>
	li.image {
		--mix-level: 30%;
		background-image:
			linear-gradient(
				to right,
				color-mix(in oklch, var(--theme-color-neutral) var(--mix-level), transparent) 0 100%
			),
			var(--image-url);
		background-position: 50% 40%;
		background-size: cover;
		background-repeat: no-repeat;

		@media (prefers-color-scheme: light) {
			--mix-level: 80%;
		}
	}
</style>
