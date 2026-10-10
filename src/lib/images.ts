import type { Picture } from '@sveltejs/enhanced-img';

const fullModules = import.meta.glob<Picture>('/assets/pics/*.{jpg,jpeg,png,webp}', {
	eager: true,
	query: { enhanced: true, w: '1080;1440;1920;2560' },
	import: 'default',
});

const thumbModules = import.meta.glob<Picture>('/assets/pics/*.{jpg,jpeg,png,webp}', {
	eager: true,
	query: { enhanced: true, w: '400;800' },
	import: 'default',
});

function index(modules: Record<string, Picture>): Map<string, Picture> {
	const entries = new Map<string, Picture>();
	for (const [path, picture] of Object.entries(modules)) {
		const name = path.slice(path.lastIndexOf('/') + 1);
		entries.set(name.slice(0, name.lastIndexOf('.')), picture);
	}
	return entries;
}

const full = index(fullModules);
const thumbs = index(thumbModules);

export function fullImage(id: string): Picture | undefined {
	return full.get(id);
}

export function thumbImage(id: string): Picture | undefined {
	return thumbs.get(id);
}
