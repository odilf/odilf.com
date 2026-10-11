<script lang="ts">
	import Back from '#lib/components/Back.svelte';
</script>

<Back />

<div class="mx-auto max-w-line-length px-4">
	<h1>about</h1>

	<div class="prose">
		<p>hi, I'm Odysseas, I like making computers do stuff tastefully, usually using Rust.</p>
		<p>
			I have a master's from the Carlos III University of Madrid, a bachelor from TU Delft in the
			Netherlands, and IB + National spanish highschool.
		</p>
		<p>I also like to make and play music; mainly with the bass, the keyboard and the computer</p>
		<p>You can download my CV here:</p>
		<ul>
			<li>
				<a href="https://github.com/odilf/cv/releases/latest/download/cv-english.pdf">
					CV (English)
				</a>
			</li>
			<li>
				<a href="https://github.com/odilf/cv/releases/latest/download/cv-spanish.pdf">
					CV (Spanish)
				</a>
			</li>
		</ul>

		<object
			class="pt-8"
			data="https://github.com/odilf/cv/releases/latest/download/cv-english.pdf"
			type="application/pdf"
			title="CV (English)"
			width="100%"
			style="aspect-ratio: 0.67;"
		>
			Your browser can't render this PDF, please use the links above.
		</object>
	</div>
</div>
