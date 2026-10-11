import { mkdir, readFile, readdir, writeFile } from 'node:fs/promises';
import { extname, join } from 'node:path';
import { loadEnvFile } from 'node:process';
import subsetFont from 'subset-font';

try {
	loadEnvFile('.env');
} catch {
	// no .env file; rely on the ambient environment
}

/** Iosevka weights referenced by the site's styles. */
const WEIGHTS = [300, 400, 600, 700, 900];

/** The ligature size tradeoff is recorded in docs/decisions/0001-font-loading.md. */
const KEEP_LIGATURES = true;

const SOURCE_DIR = join('node_modules', '@fontsource', 'iosevka', 'files');
const OUTPUT_DIR = join('src', 'fonts');

const TEXT_EXTENSIONS = new Set([
	'.css',
	'.html',
	'.js',
	'.json',
	'.md',
	'.svelte',
	'.svg',
	'.toml',
	'.ts',
	'.txt',
	'.xml',
	'.yaml',
	'.yml',
]);

/** Typographic characters that markdown entities can render without appearing literally in source. */
const EXTRA_CHARACTERS =
	'\u00a0\u00b0\u00b1\u00b5\u00d7\u2010\u2011\u2012\u2013\u2014\u2015\u2018\u2019\u201c\u201d\u2022\u2026\u2190\u2192\u2194\u2260\u2264\u2265';

async function* textFiles(dir: string): AsyncGenerator<string> {
	for (const entry of await readdir(dir, { withFileTypes: true })) {
		const path = join(dir, entry.name);
		if (entry.isDirectory()) yield* textFiles(path);
		else if (TEXT_EXTENSIONS.has(extname(entry.name))) yield path;
	}
}

async function collectCharacters(): Promise<string> {
	const characters = new Set<string>();
	for (let code = 0x20; code <= 0x7e; code += 1) characters.add(String.fromCodePoint(code));
	for (const character of EXTRA_CHARACTERS) characters.add(character);

	const roots = [
		'src',
		'projects.toml',
		process.env.ODILF_BLOG_PATH,
		process.env.ODILF_MEDIA_LOG_PATH,
	].filter((root): root is string => Boolean(root));

	for (const root of roots) {
		if (TEXT_EXTENSIONS.has(extname(root))) {
			for (const character of await readFile(root, 'utf8')) characters.add(character);
			continue;
		}

		for await (const file of textFiles(root)) {
			for (const character of await readFile(file, 'utf8')) characters.add(character);
		}
	}

	return [...characters].join('');
}

const characters = await collectCharacters();
await mkdir(OUTPUT_DIR, { recursive: true });

let total = 0;
for (const weight of WEIGHTS) {
	const input = await readFile(join(SOURCE_DIR, `iosevka-latin-${weight}-normal.woff2`));
	const output = await subsetFont(input, characters, {
		targetFormat: 'woff2',
		keepFeatures: KEEP_LIGATURES ? undefined : [],
	});
	await writeFile(join(OUTPUT_DIR, `iosevka-${weight}-normal.woff2`), output);

	total += output.length;
	console.log(
		`iosevka-${weight}-normal.woff2: ${(input.length / 1024).toFixed(0)} KiB -> ${(output.length / 1024).toFixed(1)} KiB`,
	);
}

console.log(`Subset ${characters.length} codepoints into ${(total / 1024).toFixed(1)} KiB`);
