import { defineConfig } from 'astro/config';
import mdx from '@astrojs/mdx';
import remarkMath from 'remark-math';
import rehypeKatex from 'rehype-katex';
import rehypeThemedImages from './src/plugins/rehype-themed-images.js';
import fs from 'fs';

// Load KaTeX macros from generated file
let katexMacros = {};
try {
  const macrosPath = new URL('./katex-macros.json', import.meta.url);
  katexMacros = JSON.parse(fs.readFileSync(macrosPath, 'utf-8'));
} catch {
  console.warn('katex-macros.json not found, using empty macros');
}

export default defineConfig({
  integrations: [mdx()],
  markdown: {
    remarkPlugins: [remarkMath],
    rehypePlugins: [
      [rehypeKatex, {
        macros: katexMacros,
        strict: false,
        // trust: false is the default - only trust macros we explicitly define
      }],
      rehypeThemedImages,
    ],
    shikiConfig: {
      themes: {
        light: 'kanagawa-lotus',
        dark: 'nord',
      },
    },
  },
});
