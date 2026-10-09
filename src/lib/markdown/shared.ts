import type { Element, Root as HastRoot } from 'hast';
import type { Transformer } from 'unified';
import { visit } from 'unist-util-visit';

export function rehypeFigure(): Transformer<HastRoot, HastRoot> {
	return (tree) => {
		visit(tree, 'element', (node, index, parent) => {
			if (node.tagName !== 'img' || index === undefined || parent === undefined) return;
			if (parent.type === 'element' && parent.tagName === 'figure') return;

			const title = node.properties.title;
			if (typeof title !== 'string' || title.length === 0) return;

			parent.children[index] = {
				type: 'element',
				tagName: 'figure',
				properties: {},
				children: [
					node as Element,
					{
						type: 'element',
						tagName: 'figcaption',
						properties: {},
						children: [{ type: 'text', value: title }],
					},
				],
			};
		});
	};
}
