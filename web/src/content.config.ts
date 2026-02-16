import { defineCollection, z } from 'astro:content';
import { glob } from 'astro/loaders';

const manual = defineCollection({
  loader: glob({ pattern: '**/*.mdx', base: './src/content/manual' }),
  schema: z.object({
    title: z.string(),
    description: z.string().optional(),
    sidebar: z.object({
      order: z.number(),
      group: z.string().optional(),
    }).optional(),
  }),
});

export const collections = { manual };
