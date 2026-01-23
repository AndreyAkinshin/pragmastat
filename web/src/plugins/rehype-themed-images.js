/**
 * Rehype plugin to transform image references for theme switching.
 *
 * Transforms:
 *   <img src="/img/foo.png">
 * Into:
 *   <picture class="themed-image">
 *     <source srcset="/img/foo_light.png" media="[data-theme='light']">
 *     <source srcset="/img/foo_dark.png" media="[data-theme='dark']">
 *     <img src="/img/foo_dark.png" alt="...">
 *   </picture>
 *
 * Since media queries can't detect data-theme attributes, we use CSS classes instead.
 */

import { visit } from 'unist-util-visit';

export default function rehypeThemedImages() {
  return (tree) => {
    visit(tree, 'element', (node, index, parent) => {
      if (node.tagName !== 'img') return;

      const src = node.properties?.src;
      if (!src || typeof src !== 'string') return;
      if (!src.startsWith('/img/')) return;

      // Extract base name and extension
      const match = src.match(/^(\/img\/[^.]+)(\.[^.]+)$/);
      if (!match) return;

      const [, basePath, ext] = match;
      const lightSrc = `${basePath}_light${ext}`;
      const darkSrc = `${basePath}_dark${ext}`;

      // Replace <img> with a span containing both themed images
      const themedContainer = {
        type: 'element',
        tagName: 'span',
        properties: { className: ['themed-image'] },
        children: [
          {
            type: 'element',
            tagName: 'img',
            properties: {
              ...node.properties,
              src: lightSrc,
              className: ['theme-light'],
            },
            children: [],
          },
          {
            type: 'element',
            tagName: 'img',
            properties: {
              ...node.properties,
              src: darkSrc,
              className: ['theme-dark'],
            },
            children: [],
          },
        ],
      };

      // Replace the original node
      parent.children[index] = themedContainer;
    });
  };
}
