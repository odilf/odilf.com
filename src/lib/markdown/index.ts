export { parseBlogMetadata, renderBlogMarkdown, type RenderedMarkdown } from './blog.js';
export { parseMediaMetadata, renderMediaReview } from './media.js';

import type { Element, Root as HastRoot } from 'hast';
import type { Transformer } from 'unified';
import { visit } from 'unist-util-visit';
import matter from 'gray-matter';
import * as yaml from 'js-yaml';

export function frontmatter(content: string) {
  return matter(content, {
    engines: {
      yaml: (s) => yaml.load(s, { schema: yaml.CORE_SCHEMA }) as object,
      // If we wrote back, this would be needed. But we never write back.
      // stringify: (o) => yaml.dump(o),
    },
  })
}

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
