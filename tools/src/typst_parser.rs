use crate::typst_eval::{EvalContext, TypstValue, parse_definitions};
use anyhow::Result;
use std::path::Path;
use typst_syntax::{SyntaxKind, SyntaxNode, ast, ast::AstNode, parse};

/// Parsed Typst document content
#[derive(Debug, Clone)]
pub struct TypstDocument {
    pub events: Vec<TypstEvent>,
}

impl TypstDocument {
    /// Extract all citation keys used in this document
    pub fn extract_citations(&self) -> std::collections::HashSet<String> {
        let mut citations = std::collections::HashSet::new();
        for event in &self.events {
            Self::collect_citations_from_event(event, &mut citations);
        }
        citations
    }

    fn collect_citations_from_event(
        event: &TypstEvent,
        citations: &mut std::collections::HashSet<String>,
    ) {
        match event {
            TypstEvent::Citation(key) => {
                citations.insert(key.clone());
            }
            TypstEvent::ListItem { content, .. }
            | TypstEvent::Strong(content)
            | TypstEvent::Emphasis(content) => {
                for e in content {
                    Self::collect_citations_from_event(e, citations);
                }
            }
            TypstEvent::Table { headers, rows } => {
                for cell in headers {
                    for e in cell {
                        Self::collect_citations_from_event(e, citations);
                    }
                }
                for row in rows {
                    for cell in row {
                        for e in cell {
                            Self::collect_citations_from_event(e, citations);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

/// Document events for processing (similar to `DocEvent` but for Typst source)
#[derive(Debug, Clone)]
pub enum TypstEvent {
    Text(String),
    Heading {
        level: u8,
        text: String,
    },
    CodeBlock {
        lang: String,
        code: String,
    },
    Math {
        display: bool,
        content: String,
    },
    Citation(String),
    ParagraphBreak,
    ListItem {
        depth: u8,
        content: Vec<TypstEvent>,
    },
    Image {
        alt: String,
        src: String,
    },
    Link {
        text: String,
        dest: String,
    },
    Strong(Vec<TypstEvent>),
    Emphasis(Vec<TypstEvent>),
    Table {
        headers: Vec<Vec<TypstEvent>>,
        rows: Vec<Vec<Vec<TypstEvent>>>,
    },
    ThematicBreak,
}

/// Parse a Typst document, resolving #include directives and evaluating variables
pub fn parse_typst_document(path: &Path, base_path: &Path) -> Result<TypstDocument> {
    let content = std::fs::read_to_string(path)?;
    // Resolve includes relative to the file's directory, not base_path
    let file_dir = path.parent().unwrap_or(Path::new("."));
    let resolved = resolve_includes(&content, file_dir)?;

    // Load definitions and preprocess to expand variables
    let definitions_path = base_path.join("manual/definitions.typ");
    let ctx = if definitions_path.exists() {
        parse_definitions(&definitions_path)?
    } else {
        EvalContext::new(base_path)
    };

    let preprocessed = preprocess_typst(&resolved, &ctx, base_path)?;
    let events = parse_typst_content(&preprocessed);
    Ok(TypstDocument { events })
}

/// Preprocess Typst content to expand variable references and dynamic function calls
#[allow(clippy::too_many_lines)]
fn preprocess_typst(content: &str, ctx: &EvalContext, base_path: &Path) -> Result<String> {
    let mut result = String::new();
    let mut local_ctx = ctx.clone();
    let chars: Vec<char> = content.chars().collect();
    let mut char_idx = 0;

    // Helper to get substring from char indices
    let chars_to_string =
        |chars: &[char], start: usize, end: usize| -> String { chars[start..end].iter().collect() };

    // Helper to check if remaining chars start with pattern
    let starts_with = |chars: &[char], idx: usize, pattern: &str| -> bool {
        let pat_chars: Vec<char> = pattern.chars().collect();
        if idx + pat_chars.len() > chars.len() {
            return false;
        }
        chars[idx..idx + pat_chars.len()] == pat_chars[..]
    };

    while char_idx < chars.len() {
        // Check for #import (skip - definitions already loaded)
        if starts_with(&chars, char_idx, "#import") {
            // Skip to end of line
            while char_idx < chars.len() && chars[char_idx] != '\n' {
                char_idx += 1;
            }
            if char_idx < chars.len() {
                char_idx += 1;
            }
            continue;
        }

        // Check for #let
        if starts_with(&chars, char_idx, "#let") {
            char_idx += 4;

            // Skip whitespace
            while char_idx < chars.len()
                && chars[char_idx].is_whitespace()
                && chars[char_idx] != '\n'
            {
                char_idx += 1;
            }

            // Read variable name
            let name_start = char_idx;
            while char_idx < chars.len()
                && (chars[char_idx].is_alphanumeric()
                    || chars[char_idx] == '-'
                    || chars[char_idx] == '_')
            {
                char_idx += 1;
            }
            let name = chars_to_string(&chars, name_start, char_idx).replace('-', "_");

            // Skip whitespace and =
            while char_idx < chars.len()
                && (chars[char_idx].is_whitespace() || chars[char_idx] == '=')
                && chars[char_idx] != '\n'
            {
                char_idx += 1;
            }

            // Read value (simple case: just copy the reference)
            let value_start = char_idx;
            while char_idx < chars.len() && chars[char_idx] != '\n' {
                char_idx += 1;
            }
            let value_str = chars_to_string(&chars, value_start, char_idx);
            let value_str = value_str.trim();

            // Evaluate the value
            if let Some(resolved) = local_ctx.resolve(value_str) {
                local_ctx.set(&name, resolved.clone());
            } else {
                // Store as string if not a resolvable reference
                local_ctx.set(&name, TypstValue::String(value_str.to_string()));
            }

            // Skip to next line
            if char_idx < chars.len() && chars[char_idx] == '\n' {
                char_idx += 1;
            }

            // Don't output the #let line
            continue;
        }

        // Check for #raw(...) function call
        if starts_with(&chars, char_idx, "#raw") {
            let start = char_idx;
            char_idx += 4;

            // Skip whitespace
            while char_idx < chars.len()
                && chars[char_idx].is_whitespace()
                && chars[char_idx] != '\n'
            {
                char_idx += 1;
            }

            if char_idx < chars.len()
                && chars[char_idx] == '('
                && let Some((code_block, new_idx)) =
                    parse_raw_call_chars(&chars, char_idx, &local_ctx, base_path)?
            {
                result.push_str(&code_block);
                char_idx = new_idx;
                continue;
            }

            // Couldn't parse, output as-is
            result.push_str(&chars_to_string(&chars, start, char_idx));
            continue;
        }

        // Check for #link(...) function call
        if starts_with(&chars, char_idx, "#link") {
            let start = char_idx;
            char_idx += 5;

            // Skip whitespace
            while char_idx < chars.len()
                && chars[char_idx].is_whitespace()
                && chars[char_idx] != '\n'
            {
                char_idx += 1;
            }

            if char_idx < chars.len()
                && chars[char_idx] == '('
                && let Some((link_md, new_idx)) =
                    parse_link_call_chars(&chars, char_idx, &local_ctx)?
            {
                result.push_str(&link_md);
                char_idx = new_idx;
                continue;
            }

            // Couldn't parse, output as-is
            result.push_str(&chars_to_string(&chars, start, char_idx));
            continue;
        }

        // Check for #variable or #variable.field reference (not followed by ()
        if chars[char_idx] == '#'
            && char_idx + 1 < chars.len()
            && chars[char_idx + 1].is_alphabetic()
        {
            let start = char_idx;
            char_idx += 1;

            // Read identifier path (name.field.field...)
            let ident_start = char_idx;
            while char_idx < chars.len()
                && (chars[char_idx].is_alphanumeric()
                    || chars[char_idx] == '_'
                    || chars[char_idx] == '-'
                    || chars[char_idx] == '.')
            {
                char_idx += 1;
            }
            let ident = chars_to_string(&chars, ident_start, char_idx).replace('-', "_");

            // Check if followed by ( - if so, it's a function call, handle elsewhere
            if char_idx < chars.len() && chars[char_idx] == '(' {
                // Output as-is (will be handled by AST parser or other preprocessor steps)
                result.push_str(&chars_to_string(&chars, start, char_idx));
                continue;
            }

            // Resolve variable reference
            if let Some(value) = local_ctx.resolve(&ident) {
                let s = value.as_string();
                if !s.is_empty() {
                    result.push_str(s);
                }
            }
            // Skip this reference (don't output if not resolved)
            continue;
        }

        // Regular character - copy to output
        result.push(chars[char_idx]);
        char_idx += 1;
    }

    Ok(result)
}

/// Parse a #raw(...) function call using character arrays (UTF-8 safe)
#[allow(clippy::too_many_lines)]
fn parse_raw_call_chars(
    chars: &[char],
    paren_start: usize,
    ctx: &EvalContext,
    base_path: &Path,
) -> Result<Option<(String, usize)>> {
    let mut i = paren_start + 1; // Skip opening (

    // Helper to get substring from char slice
    let chars_to_string =
        |start: usize, end: usize| -> String { chars[start..end].iter().collect() };

    // Helper to check if chars start with pattern at index
    let starts_with_at = |idx: usize, pattern: &str| -> bool {
        let pat_chars: Vec<char> = pattern.chars().collect();
        if idx + pat_chars.len() > chars.len() {
            return false;
        }
        chars[idx..idx + pat_chars.len()] == pat_chars[..]
    };

    // Skip whitespace
    while i < chars.len() && chars[i].is_whitespace() {
        i += 1;
    }

    // Parse first argument (content)
    let code_content: String;

    if i < chars.len() && chars[i] == '"' {
        // String literal with potential concatenation
        let (s, new_i) = parse_string_with_concat_chars(chars, i, ctx)?;
        code_content = s;
        i = new_i;
    } else if starts_with_at(i, "read") {
        // read(...) function
        i += 4;
        // Skip whitespace
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= chars.len() || chars[i] != '(' {
            return Ok(None);
        }
        i += 1;

        // Parse path argument
        let (path_str, new_i) = parse_string_with_concat_chars(chars, i, ctx)?;
        i = new_i;

        // Skip to closing paren of read()
        while i < chars.len() && chars[i] != ')' {
            i += 1;
        }
        if i < chars.len() {
            i += 1; // Skip )
        }

        // Read the file
        // First check if it's already an absolute path that exists
        let path_obj = std::path::Path::new(&path_str);
        let file_path = if path_obj.is_absolute() && path_obj.exists() {
            path_obj.to_path_buf()
        } else if let Some(stripped) = path_str.strip_prefix('/') {
            // Typst convention: leading / means relative to project root
            base_path.join(stripped)
        } else {
            base_path.join(&path_str)
        };

        code_content = std::fs::read_to_string(&file_path)
            .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", file_path.display(), e))?;
    } else {
        // Variable reference
        let ident_start = i;
        while i < chars.len()
            && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-' || chars[i] == '.')
        {
            i += 1;
        }
        let ident = chars_to_string(ident_start, i).replace('-', "_");
        code_content = ctx.resolve_string(&ident);
    }

    // Parse named arguments (lang:, block:)
    let mut lang = String::new();
    let mut block = false;

    while i < chars.len() && chars[i] != ')' {
        // Skip whitespace and commas
        while i < chars.len() && (chars[i].is_whitespace() || chars[i] == ',') {
            i += 1;
        }

        if i >= chars.len() || chars[i] == ')' {
            break;
        }

        // Read argument name
        let arg_start = i;
        while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
            i += 1;
        }
        let arg_name = chars_to_string(arg_start, i);

        // Skip : and whitespace
        while i < chars.len() && (chars[i] == ':' || chars[i].is_whitespace()) {
            i += 1;
        }

        // Parse argument value
        if arg_name == "lang" {
            if i < chars.len() && chars[i] == '"' {
                let (s, new_i) = parse_simple_string_chars(chars, i)?;
                lang = s;
                i = new_i;
            } else {
                // Variable reference for lang
                let ident_start = i;
                while i < chars.len()
                    && (chars[i].is_alphanumeric()
                        || chars[i] == '_'
                        || chars[i] == '-'
                        || chars[i] == '.')
                {
                    i += 1;
                }
                let ident = chars_to_string(ident_start, i).replace('-', "_");
                lang = ctx.resolve_string(&ident);
            }
        } else if arg_name == "block" {
            // Expect "true" or "false"
            let val_start = i;
            while i < chars.len() && chars[i].is_alphabetic() {
                i += 1;
            }
            block = chars_to_string(val_start, i) == "true";
        }
    }

    // Skip closing paren
    if i < chars.len() && chars[i] == ')' {
        i += 1;
    }

    // Generate code block
    let code_content_trimmed = code_content.trim_end();
    let code_block = if block {
        format!("```{lang}\n{code_content_trimmed}\n```\n")
    } else {
        format!("`{code_content}`")
    };

    Ok(Some((code_block, i)))
}

/// Parse a #link(...) function call using character arrays (UTF-8 safe)
fn parse_link_call_chars(
    chars: &[char],
    paren_start: usize,
    ctx: &EvalContext,
) -> Result<Option<(String, usize)>> {
    let mut i = paren_start + 1; // Skip opening (

    // Helper to get substring from char slice
    let chars_to_string =
        |start: usize, end: usize| -> String { chars[start..end].iter().collect() };

    // Skip whitespace
    while i < chars.len() && chars[i].is_whitespace() {
        i += 1;
    }

    // Parse URL (first argument)
    let url: String;
    if i < chars.len() && chars[i] == '"' {
        let (s, new_i) = parse_string_with_concat_chars(chars, i, ctx)?;
        url = s;
        i = new_i;
    } else {
        // Expression: variable + "string" + ...
        let (s, new_i) = parse_concat_expr_chars(chars, i, ctx)?;
        url = s;
        i = new_i;
    }

    // Skip to closing paren or content block
    while i < chars.len() && chars[i].is_whitespace() {
        i += 1;
    }

    // Check for content block [text]
    let mut link_text = String::new();
    if i < chars.len() && chars[i] == ')' {
        i += 1;

        // Check for content block after )
        while i < chars.len() && chars[i].is_whitespace() && chars[i] != '\n' {
            i += 1;
        }

        if i < chars.len() && chars[i] == '[' {
            i += 1;
            let text_start = i;
            while i < chars.len() && chars[i] != ']' {
                i += 1;
            }
            link_text = chars_to_string(text_start, i);
            if i < chars.len() {
                i += 1; // Skip ]
            }
        }
    } else {
        // Skip to closing paren
        while i < chars.len() && chars[i] != ')' {
            i += 1;
        }
        if i < chars.len() {
            i += 1;
        }
    }

    // Reconstruct #link() call with resolved URL for Typst parser to handle
    if link_text.is_empty() || link_text == url {
        // No custom text - output bare URL, Typst will auto-link it
        Ok(Some((url, i)))
    } else {
        // Custom text - reconstruct #link() call so parser creates proper Link event
        let link_output = format!("#link(\"{url}\")[{link_text}]");
        Ok(Some((link_output, i)))
    }
}

/// Parse a concatenation expression using character arrays: var + "string" + var ...
fn parse_concat_expr_chars(
    chars: &[char],
    mut i: usize,
    ctx: &EvalContext,
) -> Result<(String, usize)> {
    let mut result = String::new();

    // Helper to get substring from char slice
    let chars_to_string =
        |start: usize, end: usize| -> String { chars[start..end].iter().collect() };

    loop {
        // Skip whitespace
        while i < chars.len() && chars[i].is_whitespace() && chars[i] != '\n' {
            i += 1;
        }

        if i >= chars.len() {
            break;
        }

        // String literal
        if chars[i] == '"' {
            let (s, new_i) = parse_simple_string_chars(chars, i)?;
            result.push_str(&s);
            i = new_i;
        }
        // Variable reference
        else if chars[i].is_alphabetic() || chars[i] == '_' {
            let ident_start = i;
            while i < chars.len()
                && (chars[i].is_alphanumeric()
                    || chars[i] == '_'
                    || chars[i] == '-'
                    || chars[i] == '.')
            {
                i += 1;
            }
            let ident = chars_to_string(ident_start, i).replace('-', "_");
            result.push_str(&ctx.resolve_string(&ident));
        } else {
            break;
        }

        // Skip whitespace
        while i < chars.len() && chars[i].is_whitespace() && chars[i] != '\n' {
            i += 1;
        }

        // Check for +
        if i < chars.len() && chars[i] == '+' {
            i += 1;
        } else {
            break;
        }
    }

    Ok((result, i))
}

/// Parse a string literal with potential concatenation using character arrays
fn parse_string_with_concat_chars(
    chars: &[char],
    i: usize,
    ctx: &EvalContext,
) -> Result<(String, usize)> {
    parse_concat_expr_chars(chars, i, ctx)
}

/// Parse a simple string literal using character arrays (just the string, no concatenation)
#[allow(clippy::unnecessary_wraps)]
fn parse_simple_string_chars(chars: &[char], mut i: usize) -> Result<(String, usize)> {
    if i >= chars.len() || chars[i] != '"' {
        return Ok((String::new(), i));
    }
    i += 1;

    let mut result = String::new();
    while i < chars.len() && chars[i] != '"' {
        if chars[i] == '\\' && i + 1 < chars.len() {
            i += 1;
            match chars[i] {
                'n' => result.push('\n'),
                't' => result.push('\t'),
                '"' => result.push('"'),
                '\\' => result.push('\\'),
                _ => {
                    result.push('\\');
                    result.push(chars[i]);
                }
            }
        } else {
            result.push(chars[i]);
        }
        i += 1;
    }

    if i < chars.len() && chars[i] == '"' {
        i += 1;
    }

    Ok((result, i))
}

/// Resolve #include "path.typ" and #source-include directives recursively
#[allow(clippy::too_many_lines)]
fn resolve_includes(content: &str, current_dir: &Path) -> Result<String> {
    let mut result = String::new();
    let mut remaining = content;

    loop {
        // Find the next directive
        let typst_include = remaining.find("#include");
        let source_include = remaining.find("#source-include");

        // Determine which comes first
        let (start, directive_type) = match (typst_include, source_include) {
            (Some(t), Some(s)) if t < s => (t, "typst"),
            (Some(t), None) => (t, "typst"),
            (None | Some(_), Some(s)) => (s, "source"),
            (None, None) => break,
        };

        // Add content before the directive
        result.push_str(&remaining[..start]);

        // Check if this directive is commented out (preceded by // on the same line)
        let line_start = remaining[..start].rfind('\n').map_or(0, |i| i + 1);
        let before_directive = &remaining[line_start..start];
        if before_directive.trim_start().starts_with("//") {
            // This is a comment, skip it
            let directive = if directive_type == "typst" {
                "#include"
            } else {
                "#source-include"
            };
            result.push_str(directive);
            remaining = &remaining[start + directive.len()..];
            continue;
        }

        if directive_type == "typst" {
            // Handle #include "path.typ"
            let after_include = &remaining[start + 8..];
            let after_include = after_include.trim_start();

            // Find the quoted path
            if let Some(quote_start) = after_include.find('"') {
                let after_quote = &after_include[quote_start + 1..];
                if let Some(quote_end) = after_quote.find('"') {
                    let include_path = &after_quote[..quote_end];
                    let full_path = current_dir.join(include_path.trim());

                    // Read and recursively resolve the included file
                    let included_content = std::fs::read_to_string(&full_path).map_err(|e| {
                        anyhow::anyhow!("Failed to include {}: {}", full_path.display(), e)
                    })?;

                    // Resolve nested includes relative to the included file's directory
                    let include_dir = full_path.parent().unwrap_or(Path::new("."));
                    let resolved_include = resolve_includes(&included_content, include_dir)?;
                    result.push_str(&resolved_include);
                    result.push('\n');

                    // Move past the include directive
                    remaining = &remaining[start + 8 + quote_start + 1 + quote_end + 1..];
                    continue;
                }
            }

            // No valid include found, just add the #include text
            result.push_str("#include");
            remaining = &remaining[start + 8..];
        } else {
            // Handle #source-include("path", "lang") function call syntax
            let after_directive = &remaining[start + 15..]; // Skip "#source-include"
            let after_directive = after_directive.trim_start();

            // Expect opening parenthesis for function call
            if !after_directive.starts_with('(') {
                result.push_str("#source-include");
                remaining = &remaining[start + 15..];
                continue;
            }

            let after_paren = &after_directive[1..]; // Skip '('

            if let Some(quote_start) = after_paren.find('"') {
                let after_quote = &after_paren[quote_start + 1..];
                if let Some(quote_end) = after_quote.find('"') {
                    let include_path = &after_quote[..quote_end];

                    // Get the language (second string argument after comma)
                    let after_path = &after_quote[quote_end + 1..];
                    // Find the second quoted string (the language)
                    let lang = if let Some(lang_quote_start) = after_path.find('"') {
                        let after_lang_quote = &after_path[lang_quote_start + 1..];
                        if let Some(lang_quote_end) = after_lang_quote.find('"') {
                            &after_lang_quote[..lang_quote_end]
                        } else {
                            ""
                        }
                    } else {
                        ""
                    };

                    // Find the file relative to project root
                    let include_path_trimmed = include_path.trim();
                    let full_path = find_project_root(current_dir).map_or_else(
                        || current_dir.join(include_path_trimmed),
                        |root| root.join(include_path_trimmed),
                    );

                    if full_path.exists() {
                        let source_content = std::fs::read_to_string(&full_path).map_err(|e| {
                            anyhow::anyhow!(
                                "Failed to include source {}: {}",
                                full_path.display(),
                                e
                            )
                        })?;

                        // Generate a Typst raw block with the source code
                        result.push_str("```");
                        result.push_str(lang);
                        result.push('\n');
                        result.push_str(&source_content);
                        if !source_content.ends_with('\n') {
                            result.push('\n');
                        }
                        result.push_str("```\n");
                    } else {
                        anyhow::bail!(
                            "Failed to include source '{}': file not found at {}",
                            include_path,
                            full_path.display()
                        );
                    }

                    // Move past the directive (find closing paren and newline)
                    let directive_str = &remaining[start..];
                    let close_paren_pos = directive_str.find(')').unwrap_or(directive_str.len());
                    let after_paren = &directive_str[close_paren_pos..];
                    // Include the newline if present, otherwise just move past the closing paren
                    let directive_end = if let Some(nl_pos) = after_paren.find('\n') {
                        start + close_paren_pos + nl_pos + 1
                    } else {
                        start + close_paren_pos + 1
                    };
                    remaining = &remaining[directive_end.min(remaining.len())..];
                    continue;
                }
            }

            // No valid source-include found, just add the directive text
            result.push_str("#source-include");
            remaining = &remaining[start + 15..];
        }
    }

    result.push_str(remaining);
    Ok(result)
}

/// Find project root by looking for CITATION.cff
fn find_project_root(start: &Path) -> Option<std::path::PathBuf> {
    let mut path = start;
    loop {
        if path.join("CITATION.cff").exists() {
            return Some(path.to_path_buf());
        }
        match path.parent() {
            Some(parent) => path = parent,
            None => return None,
        }
    }
}

/// Parse Typst content into document events
fn parse_typst_content(content: &str) -> Vec<TypstEvent> {
    let root = parse(content);
    let mut events = Vec::new();

    parse_node(&root, &mut events, 0);

    events
}

/// Recursively parse a syntax node
#[allow(clippy::too_many_lines)]
fn parse_node(node: &SyntaxNode, events: &mut Vec<TypstEvent>, list_depth: u8) {
    match node.kind() {
        SyntaxKind::Heading => {
            if let Some(heading) = node.cast::<ast::Heading>() {
                #[allow(clippy::cast_possible_truncation)]
                let level = heading.depth().get() as u8;
                let text = extract_text_content(heading.body().to_untyped());
                events.push(TypstEvent::Heading { level, text });
            }
        }
        SyntaxKind::Text => {
            let text = node.text().to_string();
            if !text.is_empty() {
                events.push(TypstEvent::Text(text));
            }
        }
        SyntaxKind::Space => {
            let text = node.text().to_string();
            // Convert multiple newlines to paragraph break
            if text.contains("\n\n") {
                events.push(TypstEvent::ParagraphBreak);
            } else if text.contains('\n') {
                events.push(TypstEvent::Text(" ".to_string()));
            } else {
                events.push(TypstEvent::Text(text));
            }
        }
        SyntaxKind::Parbreak => {
            events.push(TypstEvent::ParagraphBreak);
        }
        SyntaxKind::Escape => {
            // Handle escape sequences like \# -> #, \* -> *, etc.
            let escaped = node.text();
            if let Some(ch) = escaped.strip_prefix('\\') {
                events.push(TypstEvent::Text(ch.to_string()));
            } else {
                events.push(TypstEvent::Text(escaped.to_string()));
            }
        }
        SyntaxKind::Raw => {
            if let Some(raw) = node.cast::<ast::Raw>() {
                // to_untyped().text() only returns the node's direct text, not its children
                // We need to collect text from the RawDelim and other children
                let mut code = String::new();
                let mut in_content = false;

                for child in node.children() {
                    match child.kind() {
                        SyntaxKind::RawLang => {
                            // Skip language identifier
                        }
                        SyntaxKind::RawDelim => {
                            // Toggle content collection state
                            in_content = !in_content;
                        }
                        SyntaxKind::RawTrimmed | SyntaxKind::Text => {
                            if in_content {
                                code.push_str(child.text());
                            }
                        }
                        _ => {
                            // Collect any other text content
                            if in_content {
                                code.push_str(child.text());
                            }
                        }
                    }
                }

                let lang = raw
                    .lang()
                    .map(|l| l.to_untyped().text().to_string())
                    .unwrap_or_default();

                if raw.block() {
                    events.push(TypstEvent::CodeBlock { lang, code });
                } else {
                    // Inline code
                    events.push(TypstEvent::Text(format!("`{code}`")));
                }
            }
        }
        SyntaxKind::Equation => {
            if let Some(eq) = node.cast::<ast::Equation>() {
                let display = eq.block();
                let content = extract_math_content(eq.body().to_untyped());
                events.push(TypstEvent::Math { display, content });
            }
        }
        SyntaxKind::Strong => {
            let mut content = Vec::new();
            for child in node.children() {
                if child.kind() != SyntaxKind::Star {
                    parse_node(child, &mut content, list_depth);
                }
            }
            events.push(TypstEvent::Strong(content));
        }
        SyntaxKind::Emph => {
            let mut content = Vec::new();
            for child in node.children() {
                if child.kind() != SyntaxKind::Underscore {
                    parse_node(child, &mut content, list_depth);
                }
            }
            events.push(TypstEvent::Emphasis(content));
        }
        SyntaxKind::ListItem | SyntaxKind::EnumItem => {
            let mut content = Vec::new();
            for child in node.children() {
                // Skip the marker (-, +, or number)
                if child.kind() != SyntaxKind::Minus
                    && child.kind() != SyntaxKind::Plus
                    && !matches!(child.kind(), SyntaxKind::Int)
                    && child.kind() != SyntaxKind::Dot
                {
                    parse_node(child, &mut content, list_depth + 1);
                }
            }
            events.push(TypstEvent::ListItem {
                depth: list_depth + 1,
                content,
            });
        }
        SyntaxKind::Ref => {
            // Typst @key reference (citation)
            if let Some(reference) = node.cast::<ast::Ref>() {
                let key = reference.target().to_string();
                events.push(TypstEvent::Citation(key));
            }
        }
        SyntaxKind::Link => {
            if let Some(link) = node.cast::<ast::Link>() {
                let dest = link.get().as_str().to_string();
                events.push(TypstEvent::Link {
                    text: dest.clone(),
                    dest,
                });
            }
        }
        SyntaxKind::FuncCall => {
            // Handle special function calls like #image, #link, #line
            if let Some(call) = node.cast::<ast::FuncCall>() {
                // Get callee name from the expression
                let callee_text = match call.callee() {
                    ast::Expr::Ident(ident) => ident.to_untyped().text().to_string(),
                    _ => String::new(),
                };

                match callee_text.as_str() {
                    "image" => {
                        // Extract image path from arguments
                        if let Some(ast::Arg::Pos(ast::Expr::Str(s))) = call.args().items().next() {
                            let src = s.to_untyped().text().trim_matches('"').to_string();
                            events.push(TypstEvent::Image {
                                alt: String::new(),
                                src,
                            });
                        }
                    }
                    "link" => {
                        // Extract link destination and text
                        let mut dest = String::new();
                        let mut text = String::new();

                        for arg in call.args().items() {
                            if let ast::Arg::Pos(ast::Expr::Str(s)) = arg {
                                dest = s.to_untyped().text().trim_matches('"').to_string();
                            }
                        }

                        // Check for content body [text]
                        for child in call.args().to_untyped().children() {
                            if child.kind() == SyntaxKind::ContentBlock {
                                text = extract_text_content(child);
                                break;
                            }
                        }

                        if text.is_empty() {
                            text.clone_from(&dest);
                        }

                        events.push(TypstEvent::Link { text, dest });
                    }
                    "line" => {
                        events.push(TypstEvent::ThematicBreak);
                    }
                    "table" => {
                        let (headers, rows) = parse_table_call(call);
                        events.push(TypstEvent::Table { headers, rows });
                    }
                    _ => {
                        // Other function calls - recurse into children
                        for child in node.children() {
                            parse_node(child, events, list_depth);
                        }
                    }
                }
            }
        }
        _ => {
            // Recurse into children
            for child in node.children() {
                parse_node(child, events, list_depth);
            }
        }
    }
}

/// Extract plain text content from a node
fn extract_text_content(node: &SyntaxNode) -> String {
    let mut text = String::new();
    extract_text_recursive(node, &mut text);
    text.trim().to_string()
}

fn extract_text_recursive(node: &SyntaxNode, text: &mut String) {
    match node.kind() {
        SyntaxKind::Text => {
            text.push_str(node.text());
        }
        SyntaxKind::Space => {
            let s = node.text();
            if s.contains('\n') {
                text.push(' ');
            } else {
                text.push_str(s);
            }
        }
        SyntaxKind::Escape => {
            // Handle escape sequences like \# -> #, \* -> *, etc.
            let escaped = node.text();
            if let Some(ch) = escaped.strip_prefix('\\') {
                text.push_str(ch);
            } else {
                text.push_str(escaped);
            }
        }
        _ => {
            for child in node.children() {
                extract_text_recursive(child, text);
            }
        }
    }
}

/// Extract math content from equation body (preserves Typst math syntax)
///
/// Recursively collects all text from the node and its children to preserve
/// the original Typst math syntax exactly
fn extract_math_content(node: &SyntaxNode) -> String {
    let mut content = String::new();
    collect_all_text(node, &mut content);
    content.trim().to_string()
}

/// Recursively collect all text from a node and its children
fn collect_all_text(node: &SyntaxNode, content: &mut String) {
    let text = node.text();
    if !text.is_empty() && node.children().next().is_none() {
        // Leaf node with text
        content.push_str(text);
    } else {
        // Non-leaf node - recurse into children
        for child in node.children() {
            collect_all_text(child, content);
        }
    }
}

/// Parse a `#table()` function call into headers and rows
fn parse_table_call(call: ast::FuncCall) -> (Vec<Vec<TypstEvent>>, Vec<Vec<Vec<TypstEvent>>>) {
    let mut headers: Vec<Vec<TypstEvent>> = Vec::new();
    let mut rows = Vec::new();
    let mut current_row: Vec<Vec<TypstEvent>> = Vec::new();
    let mut columns = 0usize;
    let mut is_first_row = true;

    for arg in call.args().items() {
        match arg {
            ast::Arg::Named(named) if named.name().to_untyped().text() == "columns" => {
                // Extract column count
                if let ast::Expr::Int(i) = named.expr() {
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    let col_count = i.get() as usize;
                    columns = col_count;
                }
            }
            ast::Arg::Pos(ast::Expr::Content(content)) => {
                // Parse cell content
                let mut cell_events = Vec::new();
                parse_node(content.body().to_untyped(), &mut cell_events, 0);
                current_row.push(cell_events);

                // Check if row is complete
                if columns > 0 && current_row.len() >= columns {
                    if is_first_row {
                        headers = std::mem::take(&mut current_row);
                        is_first_row = false;
                    } else {
                        rows.push(std::mem::take(&mut current_row));
                    }
                }
            }
            _ => {}
        }
    }

    // Handle remaining cells
    if !current_row.is_empty() {
        if is_first_row {
            headers = current_row;
        } else {
            rows.push(current_row);
        }
    }

    (headers, rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typst_eval::TypstValue;
    use std::collections::HashMap;

    #[test]
    fn preprocess_let_binding() {
        // Test that #let bindings work in preprocessing
        let content = r#"#let lang = languages.py

Demo: #lang.title

#raw("pip install test==" + version, lang: "bash", block: true)
"#;

        // Set up context with languages and version
        let mut ctx = EvalContext::new(Path::new("."));
        ctx.set("version", TypstValue::String("1.0.0".to_string()));

        let mut py = HashMap::new();
        py.insert(
            "title".to_string(),
            TypstValue::String("Python".to_string()),
        );
        py.insert(
            "demo".to_string(),
            TypstValue::String("/py/demo.py".to_string()),
        );
        py.insert("code".to_string(), TypstValue::String("python".to_string()));

        let mut languages = HashMap::new();
        languages.insert("py".to_string(), TypstValue::Dictionary(py));
        ctx.set("languages", TypstValue::Dictionary(languages));

        let result = preprocess_typst(content, &ctx, Path::new(".")).unwrap();

        // Check that #let was consumed (not in output)
        assert!(!result.contains("#let"), "Output should not contain #let");

        // Check that lang.title was resolved
        assert!(
            result.contains("Python"),
            "Output should contain 'Python' from lang.title, got: {result}"
        );

        // Check that #raw was expanded to code block
        assert!(
            result.contains("```bash"),
            "Output should contain bash code block, got: {result}"
        );
        assert!(
            result.contains("pip install test==1.0.0"),
            "Output should contain version-expanded command, got: {result}"
        );
    }

    #[test]
    fn preprocess_let_from_nested_dict() {
        // Test the exact pattern from implementations: #let lang = languages.py
        let content = r"#let lang = languages.py

The demo path is: #lang.demo
";

        // Set up context mimicking definitions.typ
        let mut ctx = EvalContext::new(Path::new("."));

        // Build nested structure: languages.py.demo
        let mut py = HashMap::new();
        py.insert(
            "title".to_string(),
            TypstValue::String("Python".to_string()),
        );
        py.insert(
            "demo".to_string(),
            TypstValue::String("/py/examples/demo.py".to_string()),
        );
        py.insert("code".to_string(), TypstValue::String("python".to_string()));

        let mut languages = HashMap::new();
        languages.insert("py".to_string(), TypstValue::Dictionary(py));
        ctx.set("languages", TypstValue::Dictionary(languages));

        // Verify the context is set up correctly
        assert_eq!(ctx.resolve_string("languages.py.title"), "Python");
        assert_eq!(
            ctx.resolve_string("languages.py.demo"),
            "/py/examples/demo.py"
        );

        let result = preprocess_typst(content, &ctx, Path::new(".")).unwrap();

        // Check that lang.demo was resolved
        assert!(
            result.contains("/py/examples/demo.py"),
            "Output should contain the demo path. Got: {result}"
        );
    }

    #[test]
    fn preprocess_raw_with_read() {
        // Create a temp file
        let temp_dir = std::env::temp_dir().join("typst_preprocess_test");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let demo_file = temp_dir.join("demo.py");
        std::fs::write(&demo_file, "print('hello')").unwrap();

        let content = r"#let lang = languages.py

#raw(read(lang.demo), lang: lang.code, block: true)
";

        // Set up context
        let mut ctx = EvalContext::new(&temp_dir);

        let mut py = HashMap::new();
        py.insert(
            "demo".to_string(),
            TypstValue::String(demo_file.to_str().unwrap().to_string()),
        );
        py.insert("code".to_string(), TypstValue::String("python".to_string()));

        let mut languages = HashMap::new();
        languages.insert("py".to_string(), TypstValue::Dictionary(py));
        ctx.set("languages", TypstValue::Dictionary(languages));

        let result = preprocess_typst(content, &ctx, &temp_dir).unwrap();

        // Check that code was included
        assert!(
            result.contains("print('hello')"),
            "Output should contain file content, got: {result}"
        );
        assert!(
            result.contains("```python"),
            "Output should have python code block, got: {result}"
        );

        // Cleanup
        std::fs::remove_file(&demo_file).ok();
        std::fs::remove_dir(&temp_dir).ok();
    }

    #[test]
    fn preprocess_with_real_definitions() {
        // Test with actual definitions.typ file if available
        let def_path = std::path::Path::new("../manual/definitions.typ");
        let base_path = std::path::Path::new("..");

        if !def_path.exists() {
            return; // Skip if definitions file not available
        }

        // Load real definitions
        let ctx = crate::typst_eval::parse_definitions(def_path).expect("Should parse definitions");

        // Verify languages.py is available
        let demo_path = ctx.resolve_string("languages.py.demo");
        assert!(
            !demo_path.is_empty(),
            "languages.py.demo should not be empty, got: '{demo_path}'"
        );

        // Test preprocessing content similar to implementations
        let content = r#"#import "/manual/definitions.typ": *

#let lang = languages.py

== #lang.title

Source code: #link(github-tree + "/py")

#lang.package
"#;

        let result = preprocess_typst(content, &ctx, base_path).unwrap();

        // Should have resolved the title
        assert!(
            result.contains("Python"),
            "Should contain 'Python' from lang.title. Got:\n{result}"
        );

        // Should have resolved the link
        assert!(
            result.contains("github.com"),
            "Should contain github link. Got:\n{result}"
        );
    }

    #[test]
    fn parse_heading_with_escape() {
        // Test that escaped characters like \# are preserved in heading text
        let content = "== C\\#\n\nSome text.";
        let events = parse_typst_content(content);
        let heading = events.iter().find_map(|e| {
            if let TypstEvent::Heading { level, text } = e {
                Some((level, text.clone()))
            } else {
                None
            }
        });
        assert!(heading.is_some(), "Should find a heading");
        let (level, text) = heading.unwrap();
        assert_eq!(*level, 2, "Heading level should be 2");
        assert_eq!(
            text, "C#",
            "Heading text should be 'C#' with hash preserved"
        );
    }

    #[test]
    fn preprocess_link_with_custom_text() {
        // Test that #link("url")[text] preserves link structure for parser
        let content = r#"Download: #link("https://example.com/file.pdf")[file.pdf]"#;
        let ctx = EvalContext::new(Path::new("."));
        let result = preprocess_typst(content, &ctx, Path::new(".")).unwrap();

        // Should reconstruct #link() call with resolved URL
        assert!(
            result.contains(r#"#link("https://example.com/file.pdf")[file.pdf]"#),
            "Should contain reconstructed #link() call. Got: {result}"
        );
        // Should NOT have the text and URL separated
        assert!(
            !result.contains("file.pdf https://"),
            "Should not have text and URL separated. Got: {result}"
        );
    }

    #[test]
    fn parse_escape_in_list_item() {
        // Test that escaped characters in list items are preserved
        let content = "- C\\#: item";
        let events = parse_typst_content(content);

        // Find the list item
        let list_item = events.iter().find_map(|e| {
            if let TypstEvent::ListItem { content, .. } = e {
                Some(content)
            } else {
                None
            }
        });
        assert!(list_item.is_some(), "Should find a list item");

        // Extract text from list item content
        let content = list_item.unwrap();
        let text: String = content
            .iter()
            .filter_map(|e| {
                if let TypstEvent::Text(t) = e {
                    Some(t.as_str())
                } else {
                    None
                }
            })
            .collect();

        assert!(
            text.contains("C#"),
            "List item should contain 'C#' with hash preserved. Got: {text}"
        );
    }

    #[test]
    fn parse_heading() {
        let content = "= Test Heading\n\nSome text.";
        let events = parse_typst_content(content);
        assert!(events.iter().any(
            |e| matches!(e, TypstEvent::Heading { level: 1, text } if text == "Test Heading")
        ));
    }

    #[test]
    fn parse_math() {
        let content = "The value $x + y$ is important.";
        let events = parse_typst_content(content);
        assert!(
            events
                .iter()
                .any(|e| matches!(e, TypstEvent::Math { display: false, .. }))
        );
    }

    #[test]
    fn parse_display_math() {
        let content = "$ x + y = z $";
        let events = parse_typst_content(content);
        assert!(
            events
                .iter()
                .any(|e| matches!(e, TypstEvent::Math { display: true, .. }))
        );
    }

    #[test]
    fn parse_citation() {
        let content = "See @hodges1963 for details.";
        let events = parse_typst_content(content);
        assert!(
            events
                .iter()
                .any(|e| matches!(e, TypstEvent::Citation(key) if key == "hodges1963"))
        );
    }

    #[test]
    fn parse_list() {
        let content = "- Item one\n- Item two";
        let events = parse_typst_content(content);
        assert!(
            events
                .iter()
                .any(|e| matches!(e, TypstEvent::ListItem { .. }))
        );
    }

    #[test]
    fn parse_code_block() {
        let content = "```bash\necho hello\n```";
        let events = parse_typst_content(content);

        let code_block = events.iter().find_map(|e| {
            if let TypstEvent::CodeBlock { lang, code } = e {
                Some((lang.clone(), code.clone()))
            } else {
                None
            }
        });
        assert!(code_block.is_some(), "Should find a code block");
        let (lang, code) = code_block.unwrap();
        assert_eq!(lang, "bash", "Language should be 'bash'");
        assert!(
            code.contains("echo hello"),
            "Code should contain 'echo hello', got: {code}"
        );
        assert!(!code.contains("```"), "Code should not contain backticks");
    }

    #[test]
    fn resolve_source_include() {
        use std::io::Write;

        // Create temp directory with source file
        let temp_dir = std::env::temp_dir().join("typst_parser_test");
        std::fs::create_dir_all(&temp_dir).unwrap();

        let source_file = temp_dir.join("test.cs");
        let mut f = std::fs::File::create(&source_file).unwrap();
        writeln!(f, "public class Test {{ }}").unwrap();

        // Create content with #source-include using function call syntax
        let content = format!(
            r#"Some text before.

#source-include("{}", "cs")

Some text after."#,
            source_file.display()
        );

        // Resolve includes
        let resolved = resolve_includes(&content, &temp_dir).unwrap();

        // Verify the output contains proper code block
        assert!(
            resolved.contains("```cs\n"),
            "Should contain code block with 'cs' language, got:\n{resolved}"
        );
        assert!(
            resolved.contains("public class Test"),
            "Should contain the source code"
        );
        assert!(
            !resolved.contains(r#", "cs")"#),
            "Should not contain raw function call syntax in output, got:\n{resolved}"
        );

        // Cleanup
        std::fs::remove_file(&source_file).ok();
        std::fs::remove_dir(&temp_dir).ok();
    }

    #[test]
    fn source_include_produces_valid_code_block() {
        // Test that resolved source-include produces parseable code block
        let content = "```cs\npublic class Test { }\n```";
        let events = parse_typst_content(content);

        let code_block = events.iter().find_map(|e| {
            if let TypstEvent::CodeBlock { lang, code } = e {
                Some((lang.clone(), code.clone()))
            } else {
                None
            }
        });

        assert!(code_block.is_some(), "Should find code block");
        let (lang, code) = code_block.unwrap();
        assert_eq!(lang, "cs", "Language should be 'cs'");
        assert!(
            code.contains("public class Test"),
            "Code should contain source, got: {code}"
        );
    }
}
