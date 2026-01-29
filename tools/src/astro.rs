use crate::definitions::Definitions;
use crate::hayagriva::{References, short_citation};
use crate::math_conv::typst_to_latex;
use crate::typst_parser::{TypstDocument, TypstEvent};
use crate::xref::XRefMap;
use std::fmt::Write;

/// Generate the index/landing page with abstract and chapter links
pub fn generate_index_page<C>(abstract_content: &str, chapters: &[C]) -> String
where
    C: ChapterInfoProvider<ChapterInfoRef>,
{
    let mut output = String::new();

    // Frontmatter
    output.push_str("---\n");
    output.push_str("title: \"Pragmastat Manual\"\n");
    output.push_str("description: \"Pragmatic Statistical Toolkit - Technical Manual\"\n");
    output.push_str("---\n\n");

    // Title and abstract
    output.push_str("# Pragmastat: Pragmatic Statistical Toolkit\n\n");
    output.push_str(abstract_content.trim());
    output.push_str("\n\n");

    // Chapter links
    output.push_str("## Chapters\n\n");
    for chapter in chapters {
        let info = chapter.chapter_info();
        let _ = writeln!(output, "- [{}](./{})", info.title, info.slug);
    }
    output.push('\n');

    output
}

/// Trait for accessing chapter info (allows different struct types)
pub trait ChapterInfoProvider<T> {
    fn chapter_info(&self) -> T;
}

/// Reference to chapter info fields
pub struct ChapterInfoRef {
    pub slug: &'static str,
    pub title: &'static str,
}

/// Convert parsed Typst document to Astro MDX format
pub fn convert_typst_to_mdx(
    document: &TypstDocument,
    definitions: &Definitions,
    references: &References,
    xref_map: &XRefMap,
    title: &str,
    order: u8,
) -> String {
    let mut output = String::new();

    // Frontmatter with sidebar ordering
    output.push_str("---\n");
    let _ = writeln!(output, "title: \"{title}\"");
    let _ = writeln!(output, "sidebar:\n  order: {order}");
    output.push_str("---\n\n");

    // Skip the first H1 heading (chapter title) since it's in frontmatter
    let mut skip_first_h1 = true;
    for event in &document.events {
        if skip_first_h1 && matches!(event, TypstEvent::Heading { level: 1, .. }) {
            skip_first_h1 = false;
            continue;
        }
        convert_typst_event_to_mdx(event, definitions, references, xref_map, &mut output);
    }

    output
}

/// Convert a single Typst event to MDX format
#[allow(clippy::too_many_lines)]
fn convert_typst_event_to_mdx(
    event: &TypstEvent,
    definitions: &Definitions,
    references: &References,
    xref_map: &XRefMap,
    output: &mut String,
) {
    match event {
        TypstEvent::Heading { level, text, .. } => {
            let after_frontmatter = output.ends_with("---\n\n");
            if !output.is_empty() && !output.ends_with("\n\n") && !after_frontmatter {
                if output.ends_with('\n') {
                    output.push('\n');
                } else {
                    output.push_str("\n\n");
                }
            }
            let prefix = "#".repeat(*level as usize);
            let _ = write!(output, "{prefix} {text}\n\n");
        }
        TypstEvent::Text(text) => {
            output.push_str(text);
        }
        TypstEvent::CodeBlock { lang, code } => {
            let code = code.trim();
            if lang.is_empty() {
                let _ = write!(output, "```\n{code}\n```\n\n");
            } else {
                let _ = write!(output, "```{lang}\n{code}\n```\n\n");
            }
        }
        TypstEvent::Math { display, content } => {
            // Convert Typst math to LaTeX for KaTeX
            let latex = typst_to_latex(content, definitions);
            if *display {
                // Ensure display math starts on its own line for proper MDX parsing
                if !output.is_empty() && !output.ends_with('\n') {
                    output.push('\n');
                }
                let _ = write!(output, "\n$$\n{latex}\n$$\n\n");
            } else {
                let _ = write!(output, "${latex}$");
            }
        }
        TypstEvent::Citation(key) => {
            if let Some(reference) = references.get(key) {
                let short = short_citation(reference);
                let _ = write!(
                    output,
                    r#"<span class="citation" data-key="{key}">{short}</span>"#
                );
            } else {
                output.push('[');
                output.push_str(key);
                output.push(']');
            }
        }
        TypstEvent::ParagraphBreak => {
            output.push_str("\n\n");
        }
        TypstEvent::ListItem { depth, content } => {
            // Ensure list items start on a new line (trim trailing whitespace that isn't newline)
            while output.ends_with(' ') {
                output.pop();
            }
            if !output.is_empty() && !output.ends_with('\n') {
                output.push('\n');
            }
            let indent = "  ".repeat((*depth as usize).saturating_sub(1));
            let _ = write!(output, "{indent}- ");

            let has_nested_lists = content
                .iter()
                .any(|e| matches!(e, TypstEvent::ListItem { .. }));

            let mut before_first_nested = true;
            for item in content {
                if before_first_nested && matches!(item, TypstEvent::ListItem { .. }) {
                    output.push('\n');
                    before_first_nested = false;
                }
                convert_typst_event_to_mdx(item, definitions, references, xref_map, output);
            }

            if !has_nested_lists {
                output.push('\n');
            }
        }
        TypstEvent::Image { alt, src } => {
            let abs_src = if src.starts_with("img/") {
                format!("/{src}")
            } else {
                src.clone()
            };
            // Strip _light or _dark suffix - the rehype-themed-images plugin will add them back
            let base_src = abs_src
                .replace("_light.png", ".png")
                .replace("_dark.png", ".png")
                .replace("_light.svg", ".svg")
                .replace("_dark.svg", ".svg");
            let _ = write!(output, "![{alt}]({base_src})\n\n");
        }
        TypstEvent::Strong(content) => {
            output.push_str("**");
            for item in content {
                convert_typst_event_to_mdx(item, definitions, references, xref_map, output);
            }
            output.push_str("**");
        }
        TypstEvent::Emphasis(content) => {
            output.push('*');
            for item in content {
                convert_typst_event_to_mdx(item, definitions, references, xref_map, output);
            }
            output.push('*');
        }
        TypstEvent::Table { headers, rows } => {
            output.push('|');
            for cell in headers {
                output.push(' ');
                for item in cell {
                    convert_typst_event_to_mdx(item, definitions, references, xref_map, output);
                }
                output.push_str(" |");
            }
            output.push('\n');

            output.push('|');
            for _ in headers {
                output.push_str("---|");
            }
            output.push('\n');

            for row in rows {
                output.push('|');
                for cell in row {
                    output.push(' ');
                    for item in cell {
                        convert_typst_event_to_mdx(item, definitions, references, xref_map, output);
                    }
                    output.push_str(" |");
                }
                output.push('\n');
            }
            output.push('\n');
        }
        TypstEvent::ThematicBreak => {
            output.push_str("---\n\n");
        }
        TypstEvent::Linebreak => {
            // Use HTML <br> for forced line break (more reliable than trailing spaces in markdown)
            output.push_str("<br/>\n");
        }
        TypstEvent::Link { text, dest } => {
            // Handle internal links (cross-references)
            if let Some(label) = dest.strip_prefix("internal:") {
                if let Some(url) = xref_map.resolve(label) {
                    let _ = write!(output, "[{text}]({url})");
                } else {
                    eprintln!("Warning: unresolved xref: {label}");
                    output.push_str(text);
                }
            } else {
                let _ = write!(output, "[{text}]({dest})");
            }
        }
    }
}

/// Generate `KaTeX` macros configuration from definitions
pub fn generate_katex_config(definitions: &Definitions) -> String {
    let mut macros: Vec<String> = Vec::new();

    let mut sorted_defs: Vec<_> = definitions.iter().collect();
    sorted_defs.sort_by_key(|(k, _)| *k);

    for (name, latex) in sorted_defs {
        // Skip self-referential macros (native KaTeX commands)
        // e.g., "Phi" -> "\Phi" would create "\Phi": "\Phi" which fails
        if *latex == format!("\\{name}") {
            continue;
        }

        // Escape backslashes and quotes for JSON
        let escaped_latex = latex.replace('\\', "\\\\").replace('"', "\\\"");
        macros.push(format!("  \"\\\\{name}\": \"{escaped_latex}\""));
    }

    format!("{{\n{}\n}}", macros.join(",\n"))
}

/// Colophon information for generating the colophon page
pub struct ColophonInfo<'a> {
    pub author: &'a str,
    pub email: &'a str,
    pub doi: &'a str,
    pub github_url: &'a str,
}

/// Generate colophon page MDX content
pub fn generate_colophon_page(info: &ColophonInfo, order: u8) -> String {
    let mut output = String::new();

    // Frontmatter
    output.push_str("---\n");
    output.push_str("title: \"Colophon\"\n");
    let _ = write!(output, "sidebar:\n  order: {order}\n");
    output.push_str("---\n\n");

    // Author info (use <br /> for line breaks within the block)
    let _ = writeln!(output, "{}<br />", info.author);
    let _ = writeln!(output, "[{}](mailto:{})<br />", info.email, info.email);
    let _ = writeln!(
        output,
        "[DOI: {}](https://doi.org/{})\n",
        info.doi, info.doi
    );

    // Copyright
    output.push_str("**Copyright © 2025–2026 Andrey Akinshin**\n\n");

    // Manual license
    output.push_str("This manual is licensed under the **Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International License** ([CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/)). ");
    output.push_str("You are free to share and adapt this material for non-commercial purposes, provided you give appropriate credit, indicate if changes were made, and distribute your contributions under the same license.\n\n");

    // Code license
    output.push_str("The accompanying source code and software implementations are licensed under the **MIT License**. ");
    output.push_str("You are free to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the software, subject to the conditions stated in the license. ");
    output
        .push_str("For complete license terms, see the LICENSE file in the source repository.\n\n");

    // Disclaimer
    output.push_str("While the information in this manual is believed to be accurate at the date of publication, the author makes no warranty, express or implied, with respect to the material contained herein. ");
    output.push_str("The author shall not be liable for any errors, omissions, or damages arising from the use of this information.\n\n");

    // Source
    let github_short = info.github_url.trim_start_matches("https://");
    let _ = writeln!(
        output,
        "Source code and implementations are available at [{}]({}).\n",
        github_short, info.github_url
    );

    // Typesetting
    output.push_str("Typeset with [Typst](https://typst.app). Text refined with LLM assistance.\n");

    output
}

/// Generate bibliography page MDX content
/// Only includes references that are actually cited in the manual
/// Format matches the citation tooltip style
pub fn generate_bibliography_page(
    references: &References,
    used_citations: &std::collections::HashSet<String>,
    order: u8,
) -> String {
    let mut output = String::new();

    // Frontmatter
    output.push_str("---\n");
    output.push_str("title: \"Bibliography\"\n");
    let _ = write!(output, "sidebar:\n  order: {order}\n");
    output.push_str("---\n\n");

    // Filter to only used references and sort by author last name, then year
    let mut sorted_refs: Vec<_> = references
        .values()
        .filter(|r| used_citations.contains(&r.key))
        .collect();
    sorted_refs.sort_by(|a, b| {
        let a_sort = a.authors.first().map_or(a.key.as_str(), String::as_str);
        let b_sort = b.authors.first().map_or(b.key.as_str(), String::as_str);
        a_sort.cmp(b_sort).then_with(|| a.year.cmp(&b.year))
    });

    output.push_str("<div class=\"bibliography\">\n");

    for reference in sorted_refs {
        output.push_str("<div class=\"bib-entry\">\n");

        // Title (plain text, links are in DOI section)
        let _ = writeln!(
            output,
            "  <div class=\"bib-title\">{}</div>",
            reference.title
        );

        // Authors and year
        if reference.authors.is_empty() {
            let _ = writeln!(
                output,
                "  <div class=\"bib-authors\">({})</div>",
                reference.year
            );
        } else {
            let _ = writeln!(
                output,
                "  <div class=\"bib-authors\">{} ({})</div>",
                reference.authors.join(", "),
                reference.year
            );
        }

        // Venue
        if let Some(venue) = &reference.venue {
            let _ = writeln!(output, "  <div class=\"bib-venue\">{venue}</div>");
        }

        // DOI link
        if let Some(doi) = &reference.doi {
            let _ = writeln!(
                output,
                "  <div class=\"bib-doi\"><a href=\"https://doi.org/{doi}\" target=\"_blank\">DOI: {doi}</a></div>"
            );
        }

        output.push_str("</div>\n");
    }

    output.push_str("</div>\n");
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn generate_katex_macros() {
        let mut defs = HashMap::new();
        defs.insert("Center".to_string(), r"\operatorname{Center}".to_string());
        defs.insert("x".to_string(), r"\mathbf{x}".to_string());

        let config = generate_katex_config(&defs);
        assert!(config.contains(r#""\\Center": "\\operatorname{Center}""#));
        assert!(config.contains(r#""\\x": "\\mathbf{x}""#));
    }

    #[test]
    fn code_block_no_leading_blank_line() {
        use crate::typst_parser::TypstEvent;

        let event = TypstEvent::CodeBlock {
            lang: "bash".to_string(),
            code: "\necho hello\n".to_string(),
        };

        let defs = HashMap::new();
        let refs = crate::hayagriva::References::new();
        let xref = XRefMap::new();
        let mut output = String::new();
        convert_typst_event_to_mdx(&event, &defs, &refs, &xref, &mut output);

        // Code block should not have blank lines after opening or before closing fence
        assert_eq!(output, "```bash\necho hello\n```\n\n");
    }

    #[test]
    fn list_items_no_leading_space() {
        use crate::typst_parser::TypstEvent;

        // Simulate a space before list item (which happens between list items)
        let events = vec![
            TypstEvent::Text("Items:".to_string()),
            TypstEvent::Text(" ".to_string()), // Space that would come from newline
            TypstEvent::ListItem {
                depth: 1,
                content: vec![TypstEvent::Text("First".to_string())],
            },
            TypstEvent::Text(" ".to_string()), // Space between items
            TypstEvent::ListItem {
                depth: 1,
                content: vec![TypstEvent::Text("Second".to_string())],
            },
        ];

        let defs = HashMap::new();
        let refs = crate::hayagriva::References::new();
        let xref = XRefMap::new();
        let mut output = String::new();
        for event in &events {
            convert_typst_event_to_mdx(event, &defs, &refs, &xref, &mut output);
        }

        // List items should not have leading spaces before the dash
        assert!(
            !output.contains(" -"),
            "List items should not have leading spaces. Got: {output:?}"
        );
        // Each list item should start at beginning of line
        assert!(
            output.contains("\n- First") || output.starts_with("- First"),
            "First item should be at start of line. Got: {output:?}"
        );
        assert!(
            output.contains("\n- Second"),
            "Second item should be at start of line. Got: {output:?}"
        );
    }

    #[test]
    fn internal_link_resolved() {
        use crate::typst_parser::TypstEvent;

        let event = TypstEvent::Link {
            text: "Algorithms".to_string(),
            dest: "internal:ch-algorithms".to_string(),
        };

        let defs = HashMap::new();
        let refs = crate::hayagriva::References::new();
        let xref = XRefMap::new();
        let mut output = String::new();
        convert_typst_event_to_mdx(&event, &defs, &refs, &xref, &mut output);

        assert_eq!(output, "[Algorithms](/algorithms)");
    }

    #[test]
    fn external_link_unchanged() {
        use crate::typst_parser::TypstEvent;

        let event = TypstEvent::Link {
            text: "Example".to_string(),
            dest: "https://example.com".to_string(),
        };

        let defs = HashMap::new();
        let refs = crate::hayagriva::References::new();
        let xref = XRefMap::new();
        let mut output = String::new();
        convert_typst_event_to_mdx(&event, &defs, &refs, &xref, &mut output);

        assert_eq!(output, "[Example](https://example.com)");
    }

    #[test]
    fn skip_self_referential_katex_macros() {
        let mut defs = HashMap::new();
        defs.insert("Phi".to_string(), r"\Phi".to_string());
        defs.insert("Pr".to_string(), r"\Pr".to_string());
        defs.insert("Center".to_string(), r"\operatorname{Center}".to_string());

        let config = generate_katex_config(&defs);
        // Self-referential macros should be skipped
        assert!(!config.contains(r#""\\Phi""#));
        assert!(!config.contains(r#""\\Pr""#));
        // Custom macros should still be included
        assert!(config.contains(r#""\\Center": "\\operatorname{Center}""#));
    }
}
