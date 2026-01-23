export interface Chapter {
  slug: string;
  label: string;
}

export const chapters: Chapter[] = [
  { slug: 'introduction', label: 'Introduction' },
  { slug: 'estimators', label: 'Estimators' },
  { slug: 'distributions', label: 'Distributions' },
  { slug: 'properties', label: 'Properties' },
  { slug: 'methodology', label: 'Methodology' },
  { slug: 'algorithms', label: 'Algorithms' },
  { slug: 'studies', label: 'Studies' },
  { slug: 'implementations', label: 'Implementations' },
  { slug: 'tests', label: 'Tests' },
  { slug: 'artifacts', label: 'Artifacts' },
  { slug: 'bibliography', label: 'Bibliography' },
  { slug: 'colophon', label: 'Colophon' },
];

export function getPrevNextChapters(currentSlug: string): {
  prev: Chapter | null;
  next: Chapter | null;
} {
  const index = chapters.findIndex((c) => c.slug === currentSlug);
  if (index === -1) {
    return { prev: null, next: null };
  }
  return {
    prev: index > 0 ? chapters[index - 1] : null,
    next: index < chapters.length - 1 ? chapters[index + 1] : null,
  };
}
