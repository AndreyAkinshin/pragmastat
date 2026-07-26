/**
 * Strip the woff and truetype fallbacks from KaTeX's @font-face rules.
 *
 * katex.min.css lists three formats per face:
 *
 *   src: url(fonts/X.woff2) format("woff2"),
 *        url(fonts/X.woff)  format("woff"),
 *        url(fonts/X.ttf)   format("truetype")
 *
 * Vite resolves every url() it finds, so all three land in the bundle: 19 woff2
 * plus 20 woff plus 20 ttf, around 1MB, of which only the woff2 files are ever
 * requested by a browser that supports woff2 — which every current one does.
 * Removing the fallback urls before Vite parses the stylesheet keeps those files
 * out of the build entirely.
 *
 * Runs with enforce: 'pre' so it sees the raw stylesheet, ahead of Vite's own
 * asset resolution.
 */
export default function katexWoff2Only() {
  return {
    name: 'katex-woff2-only',
    enforce: 'pre',
    transform(code, id) {
      if (!id.includes('katex') || !id.includes('.css')) return null;

      const stripped = code
        .replace(/,\s*url\([^)]+\.woff\)\s*format\("woff"\)/g, '')
        .replace(/,\s*url\([^)]+\.ttf\)\s*format\("truetype"\)/g, '');

      if (stripped === code) return null;
      return { code: stripped, map: null };
    },
  };
}
