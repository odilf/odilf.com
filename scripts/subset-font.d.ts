declare module 'subset-font' {
	export interface SubsetFontOptions {
		targetFormat?: 'sfnt' | 'woff' | 'woff2';
		preserveNameIds?: number[];
		keepFeatures?: string[];
		keepAllGlyphs?: boolean;
		noLayoutClosure?: boolean;
		variationAxes?: Record<string, { min?: number; max?: number; default?: number }>;
	}

	export default function subsetFont(
		buffer: Buffer,
		text: string | null,
		options?: SubsetFontOptions,
	): Promise<Buffer>;
}
