//! Convert Typst math syntax to LaTeX for `KaTeX` rendering
//!
//! Typst uses a cleaner math syntax that needs conversion for web display.
//! This module handles the most common patterns used in pragmastat.

use std::collections::HashMap;
use std::fmt::Write;

/// Convert Typst math content to LaTeX string
/// Stand-in for Typst's escaped solidus `\\/` while fractions are converted.
///
/// U+2044 FRACTION SLASH never appears in the manual's sources, so it cannot collide with real
/// content, and no fraction scanner recognises it, so a literal slash passes through untouched.
const LITERAL_SOLIDUS: &str = "\u{2044}";

pub fn typst_to_latex(
    typst_math: &str,
    definitions: &HashMap<String, String>,
    display: bool,
) -> String {
    let mut result = typst_math.to_string();

    // In Typst math, `/` builds a fraction and `\/` is an escaped, literal solidus. This
    // converter used to expand `\/` into \frac and leave `/` alone inside some contexts, so the
    // website disagreed with the PDF on the same source: manual/median/median.typ writes both
    // forms in one equation, and the two outputs swapped them.
    //
    // `\/` is therefore carried through as a marker that survives fraction conversion untouched
    // and becomes a plain slash at the end, which is what Typst prints.
    result = result.replace("\\/", LITERAL_SOLIDUS);

    // Handle Typst op() function before other processing
    result = convert_op(&result);

    // Handle Typst-specific constructs that have complex syntax
    result = convert_cases(&result);
    result = convert_attach(&result);

    // Handle Typst functions that need proper delimiter matching
    result = convert_bb(&result);
    result = convert_bold(&result);
    result = convert_binom(&result);
    result = convert_upright(&result);
    result = convert_floor_ceil_abs(&result);

    // Convert quoted text to \text{} before definitions to avoid conflicts
    result = convert_text_quotes(&result);

    // Apply custom definitions (longest first to avoid partial replacements)
    // Skip single-letter definitions that would match inside longer words
    // Important: Don't apply definitions inside \text{} blocks
    result = apply_definitions_outside_text(&result, definitions);

    // Convert Typst-specific syntax to LaTeX
    result = convert_syntax(&result, display);

    // Convert Typst line breaks and handle alignment
    result = convert_alignment(&result);

    // The thin spaces were placed before the word mappings ran; drop the ones that ended up
    // after a control word, which carries its own spacing.
    result = drop_thin_space_after_commands(&result);

    result
}

/// Apply definitions to the input, but skip content inside \text{} blocks
fn apply_definitions_outside_text(input: &str, definitions: &HashMap<String, String>) -> String {
    // Extract \text{...} blocks and replace with placeholders
    let mut result = input.to_string();
    let mut text_blocks: Vec<String> = Vec::new();

    // Find and replace all \text{...} blocks with placeholders
    loop {
        if let Some(start) = result.find("\\text{") {
            let after_text = &result[start + 6..];
            if let Some(end) = find_matching_brace(after_text) {
                let text_content = &result[start..=start + 6 + end];
                let placeholder = format!("\u{FFFE}{len}\u{FFFE}", len = text_blocks.len());
                text_blocks.push(text_content.to_string());
                result = format!(
                    "{}{}{}",
                    &result[..start],
                    placeholder,
                    &result[start + 6 + end + 1..]
                );
                continue;
            }
        }
        break;
    }

    // Apply definitions to the result (which now has placeholders instead of \text{} blocks)
    let mut sorted_defs: Vec<_> = definitions.iter().collect();
    sorted_defs.sort_by_key(|(name, _)| std::cmp::Reverse(name.len()));

    for (name, latex) in sorted_defs {
        // Skip single letters - they cause too many false matches
        if name.len() == 1 {
            continue;
        }
        // Match definition name at word boundary, NOT followed by more letters
        // Rust regex doesn't support lookahead, so use capturing group approach:
        // Match name followed by non-letter or end of string, preserve the following char
        // Pattern: \bName([^a-zA-Z]|$) -> replacement$1
        let pattern = format!(r"\b{}([^a-zA-Z]|$)", regex::escape(name));
        let replacement = format!("{latex}$1");
        if let Ok(re) = regex::Regex::new(&pattern) {
            result = re.replace_all(&result, replacement.as_str()).to_string();
        }
    }

    // Restore \text{} blocks from placeholders
    for (i, block) in text_blocks.iter().enumerate() {
        let placeholder = format!("\u{FFFE}{i}\u{FFFE}");
        result = result.replace(&placeholder, block);
    }

    result
}

/// Byte offset of the `}` closing a group whose `{` has already been consumed.
///
/// Bytes, not character positions, for the reason given on `find_matching_paren`: every caller
/// slices the string with the result, and the two agree only until the first multibyte character.
/// This converter produces one itself, since an escaped solidus becomes U+2044 before these run.
fn find_matching_brace(s: &str) -> Option<usize> {
    let mut depth = 1;
    for (i, c) in s.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

/// Convert Typst op("name") to LaTeX \operatorname{name}
// The walk is over byte offsets, not character positions. find_matching_paren returns a byte
// offset and the slices below are byte slices, so mixing the two panics on the first multibyte
// character in the expression and silently mis-slices before that. The manual has multibyte
// characters in its maths.
fn convert_op(input: &str) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < input.len() {
        if input[i..].starts_with("op(") {
            let start = i + 3;
            if let Some(end) = find_matching_paren(&input[start..]) {
                let inner = &input[start..start + end];
                // Remove quotes if present
                let name = inner.trim().trim_matches('"');
                let _ = write!(result, "\\operatorname{{{name}}}");
                i = start + end + 1;
                continue;
            }
        }
        let c = input[i..].chars().next().expect("i is a char boundary");
        result.push(c);
        i += c.len_utf8();
    }

    result
}

/// Convert Typst `bb()` (blackboard bold) to LaTeX `\mathbb{}`
/// Example: `bb(1)` -> `\mathbb{1}`
fn convert_bb(input: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        // Check for bb( pattern
        if i + 3 <= chars.len() && chars[i] == 'b' && chars[i + 1] == 'b' && chars[i + 2] == '(' {
            // Calculate byte offset for string slicing
            let byte_start: usize = chars[..i + 3].iter().map(|c| c.len_utf8()).sum();
            if let Some(end) = find_matching_paren(&input[byte_start..]) {
                let inner = &input[byte_start..byte_start + end];
                let _ = write!(result, "\\mathbb{{{inner}}}");
                let content_chars = inner.chars().count();
                i = i + 3 + content_chars + 1; // bb( + inner + )
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Convert Typst `bold()` to LaTeX `\mathbf{}`
/// Example: `bold(1)` -> `\mathbf{1}`
fn convert_bold(input: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        // Check for bold( pattern (but not bb which is blackboard bold)
        if i + 5 <= chars.len()
            && chars[i] == 'b'
            && chars[i + 1] == 'o'
            && chars[i + 2] == 'l'
            && chars[i + 3] == 'd'
            && chars[i + 4] == '('
        {
            // Calculate byte offset for string slicing
            let byte_start: usize = chars[..i + 5].iter().map(|c| c.len_utf8()).sum();
            if let Some(end) = find_matching_paren(&input[byte_start..]) {
                let inner = &input[byte_start..byte_start + end];
                let _ = write!(result, "\\mathbf{{{inner}}}");
                let content_chars = inner.chars().count();
                i = i + 5 + content_chars + 1; // bold( + inner + )
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Convert Typst `binom(n, k)` to LaTeX `\binom{n}{k}`
/// Example: `binom(n+m, n)` -> `\binom{n+m}{n}`
fn convert_binom(input: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        // Check for binom( pattern
        if i + 6 <= chars.len() {
            let slice: String = chars[i..i + 6].iter().collect();
            if slice == "binom(" {
                // Calculate byte offset for string slicing
                let byte_start: usize = chars[..i + 6].iter().map(|c| c.len_utf8()).sum();
                if let Some(end) = find_matching_paren(&input[byte_start..]) {
                    let inner = &input[byte_start..byte_start + end];
                    // Find the comma separator (not inside nested parens)
                    if let Some(comma_pos) = find_comma_in_args(inner) {
                        let first = inner[..comma_pos].trim();
                        let second = inner[comma_pos + 1..].trim();
                        let _ = write!(result, "\\binom{{{first}}}{{{second}}}");
                        // Skip past the closing paren
                        // Calculate how many chars we need to skip
                        let content_chars = inner.chars().count();
                        i = i + 6 + content_chars + 1; // binom( + inner + )
                        continue;
                    }
                }
            }
        }
        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Byte offset of the comma separating two arguments at the top level of a call.
///
/// Bytes, not character positions: see `find_matching_paren`.
fn find_comma_in_args(s: &str) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in s.char_indices() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            ',' if depth == 0 => return Some(i),
            _ => {}
        }
    }
    None
}

/// Convert Typst `upright()` to LaTeX `\mathrm{}`
/// Example: `upright("mean")` -> `\mathrm{mean}`
fn convert_upright(input: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        // Check for upright( pattern
        if i + 8 <= chars.len() {
            let slice: String = chars[i..i + 8].iter().collect();
            if slice == "upright(" {
                // Calculate byte offset for string slicing
                let byte_start: usize = chars[..i + 8].iter().map(|c| c.len_utf8()).sum();
                if let Some(end) = find_matching_paren(&input[byte_start..]) {
                    let inner = &input[byte_start..byte_start + end];
                    // Remove surrounding quotes if present
                    let content = inner.trim().trim_matches('"');
                    let _ = write!(result, "\\mathrm{{{content}}}");
                    // Skip past the closing paren
                    let content_chars = inner.chars().count();
                    i = i + 8 + content_chars + 1; // upright( + inner + )
                    continue;
                }
            }
        }
        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Convert Typst `floor()`/`ceil()`/`abs()` to LaTeX delimiters
/// Examples:
///   `floor(x/2)` -> `\lfloor x/2 \rfloor`
///   `ceil(x/2)`  -> `\lceil x/2 \rceil`
///   `abs(x-y)`   -> `\lvert x-y \rvert`
fn convert_floor_ceil_abs(input: &str) -> String {
    let mut result = input.to_string();

    // \left and \right so the delimiters take the height of what they enclose. Without them a
    // floor around a fraction renders as full-height content between half-height brackets, which
    // is the one typesetting error a reader notices immediately. They are inert when the content
    // is a single symbol, so there is no case where the plain form would be preferable.
    result = convert_delimiter_func(&result, "floor(", "\\left\\lfloor ", " \\right\\rfloor");
    result = convert_delimiter_func(&result, "ceil(", "\\left\\lceil ", " \\right\\rceil");
    // \lvert/\rvert rather than | so the delimiter cannot be read as a markdown table separator.
    result = convert_delimiter_func(&result, "abs(", "\\left\\lvert ", " \\right\\rvert");

    // Alphabet functions. Without these the wrapper name reached the page as literal text:
    // `cal(N)` rendered as the three letters "cal" followed by a parenthesised N.
    result = convert_delimiter_func(&result, "cal(", "\\mathcal{", "}");
    result = convert_delimiter_func(&result, "frak(", "\\mathfrak{", "}");
    result = convert_delimiter_func(&result, "upright(", "\\mathrm{", "}");

    result
}

/// Convert a function call to LaTeX delimiters
/// `func(content)` -> `left_delim content right_delim`
fn convert_delimiter_func(
    input: &str,
    func_name: &str,
    left_delim: &str,
    right_delim: &str,
) -> String {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();
    let func_chars: Vec<char> = func_name.chars().collect();
    let func_char_len = func_chars.len();

    while i < chars.len() {
        // Check for func( pattern
        if i + func_char_len <= chars.len() {
            let slice: String = chars[i..i + func_char_len].iter().collect();
            if slice == func_name {
                // Calculate byte offset for string slicing
                let byte_start: usize = chars[..i + func_char_len]
                    .iter()
                    .map(|c| c.len_utf8())
                    .sum();
                if let Some(end) = find_matching_paren(&input[byte_start..]) {
                    let inner = &input[byte_start..byte_start + end];
                    result.push_str(left_delim);
                    result.push_str(inner);
                    result.push_str(right_delim);
                    let content_chars = inner.chars().count();
                    i = i + func_char_len + content_chars + 1;
                    continue;
                }
            }
        }
        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Convert Typst `cases()` to LaTeX `\begin{cases}...\end{cases}`
fn convert_cases(input: &str) -> String {
    let mut result = input.to_string();

    // Find cases(...) and convert to LaTeX cases environment
    // This is a simplified conversion for common patterns
    if let Some(start_byte) = result.find("cases(") {
        let after_cases = &result[start_byte + 6..];
        if let Some(end) = find_matching_paren(after_cases) {
            let inner: String = after_cases[..end].to_string();

            // Branches are separated by top-level commas, which is how Typst reads them. Source
            // newlines are only formatting: splitting on those collapsed every one-line
            // `cases(...)` into a single row, which then ran off the side of the column.
            let latex_inner = split_top_level_commas(&inner)
                .into_iter()
                .map(|branch| branch.trim().to_string())
                .filter(|branch| !branch.is_empty())
                .collect::<Vec<_>>()
                .join(" \\\\ ");

            let latex_cases = format!("\\begin{{cases}} {latex_inner} \\end{{cases}}");

            let after_end = &after_cases[end + 1..];

            result = format!("{}{}{}", &result[..start_byte], latex_cases, after_end);
        }
    }

    result
}

/// Splits on commas that are not inside a bracket, brace or parenthesis.
fn split_top_level_commas(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;
    for c in input.chars() {
        match c {
            '(' | '[' | '{' => {
                depth += 1;
                current.push(c);
            }
            ')' | ']' | '}' => {
                depth -= 1;
                current.push(c);
            }
            ',' if depth == 0 => {
                parts.push(std::mem::take(&mut current));
            }
            _ => current.push(c),
        }
    }
    parts.push(current);
    parts
}

/// Convert Typst `attach(base, b: bottom)` to LaTeX `\underset{bottom}{base}`
fn convert_attach(input: &str) -> String {
    let mut result = input.to_string();

    // Find attach(...) patterns
    while let Some(start) = result.find("attach(") {
        let after_attach = &result[start + 7..];
        if let Some(end) = find_matching_paren(after_attach) {
            let inner = &after_attach[..end];

            // Parse attach(base, b: subscript)
            // Find first comma that's not escaped (not preceded by \)
            if let Some(comma_pos) = find_unescaped_comma(inner) {
                let base = inner[..comma_pos].trim();
                let rest = &inner[comma_pos + 1..];

                // Look for b: (bottom/subscript) modifier
                let subscript = if let Some(b_pos) = rest.find("b:") {
                    let after_b = rest[b_pos + 2..].trim();
                    // Take content until next unescaped comma or end
                    if let Some(next_comma) = find_unescaped_comma(after_b) {
                        after_b[..next_comma].trim()
                    } else {
                        after_b.trim_end_matches(')')
                    }
                } else {
                    ""
                };

                if !subscript.is_empty() {
                    // Convert \, (Typst thin space) to \, (LaTeX thin space)
                    let subscript_latex = subscript.replace("\\,", "\\;");
                    let latex = format!("\\underset{{{subscript_latex}}}{{{base}}}");
                    result = format!(
                        "{}{}{}",
                        &result[..start],
                        latex,
                        &result[start + 7 + end + 1..]
                    );
                    continue;
                }
            }
        }
        // If we couldn't parse it, break to avoid infinite loop
        break;
    }

    result
}

/// Byte offset of the first comma not preceded by a backslash.
///
/// Bytes, not character positions: see `find_matching_paren`.
fn find_unescaped_comma(s: &str) -> Option<usize> {
    let mut previous = None;
    for (i, c) in s.char_indices() {
        if c == ',' && previous != Some('\\') {
            return Some(i);
        }
        previous = Some(c);
    }
    None
}

/// Find matching closing parenthesis, accounting for nesting
/// Byte offset of the `)` closing a group whose `(` has already been consumed.
///
/// The offset is in bytes because every caller but one slices the string with it directly, and
/// they were doing that while this returned a character index. The two agree on ASCII, which is
/// why it went unnoticed: the first piece of prose to put a non-ASCII character inside a converted
/// group was a fraction slash, and it panicked on a char boundary rather than quietly
/// mis-slicing.
fn find_matching_paren(s: &str) -> Option<usize> {
    let mut depth = 1;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

/// Convert Typst "text" to LaTeX \text{text}
/// Removes a thin space that ended up directly after a LaTeX control word.
///
/// `convert_text_quotes` inserts the gap before the word mappings run, so at that point a Typst
/// operator such as `and` is still a bare word and looks like an atom; by the time it becomes
/// `\land` the gap is already in the string. Control words carry their own spacing and the extra
/// width shows: after `\begin{cases}` it indents the first branch relative to the others, which
/// reads as a misaligned column. There were 34 of these in the rendered manual.
///
/// `\text`, `\mathrm`, `\operatorname`, `\mathbf` and `\mathit` set an atom rather than spacing,
/// so a word following one of those still needs separating from it.
fn drop_thin_space_after_commands(input: &str) -> String {
    const ATOM_COMMANDS: [&str; 5] = ["text", "mathrm", "operatorname", "mathbf", "mathit"];
    let mut result = String::with_capacity(input.len());
    let mut rest = input;
    while let Some(at) = rest.find(r"\;") {
        let (before, after) = rest.split_at(at);
        result.push_str(before);
        let keep = match before.rfind('\\') {
            Some(start) => {
                let word: String = before[start + 1..]
                    .chars()
                    .take_while(char::is_ascii_alphabetic)
                    .collect();
                let tail = &before[start + 1 + word.len()..];
                // A control word either stands alone (\land) or takes a braced argument
                // (\begin{cases}, \text{if}). Both are commands; only the ones that SET an atom
                // earn a following gap.
                let is_command = !word.is_empty()
                    && (tail.is_empty() || (tail.starts_with('{') && tail.ends_with('}')));
                let sets_an_atom = ATOM_COMMANDS.contains(&word.as_str());
                !is_command || sets_an_atom
            }
            None => true,
        };
        if keep {
            result.push_str(r"\;");
        }
        rest = &after[2..];
    }
    result.push_str(rest);
    result
}

fn convert_text_quotes(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let mut in_quote = false;
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c != '"' {
            result.push(c);
            i += 1;
            continue;
        }
        if !in_quote {
            // Symmetric to the gap inserted after a closing quote: `n "is odd"` needs the same
            // thin space on this side, since the source space between them carries no width.
            // Mirror of the rule above: only an adjacent atom earns the gap. A script marker
            // binds to what follows it, so it is excluded even though it ends in a letter.
            let trimmed = result.trim_end();
            let needs_gap = trimmed
                .chars()
                .last()
                .is_some_and(|c| c.is_alphanumeric() || c == '}')
                && !trimmed.ends_with('_')
                && !trimmed.ends_with('^');
            if needs_gap {
                result.truncate(trimmed.len());
                result.push_str("\\;");
            }
            result.push_str("\\text{");
            in_quote = true;
            i += 1;
            continue;
        }
        result.push('}');
        in_quote = false;
        i += 1;

        // Typst separates a quoted word from what follows it. LaTeX does not, and a literal
        // space carries no width in math mode, so `"if" n "is odd"` rendered as a single run of
        // letters. A thin space restores the gap. Source spaces are dropped because they
        // contribute nothing on their own.
        let mut j = i;
        while j < chars.len() && chars[j].is_whitespace() {
            j += 1;
        }
        // A thin space belongs only between the word and an adjacent atom: a variable, a number
        // or another word. LaTeX already spaces relations, delimiters and separators, so adding
        // one there is stray width rather than a gap.
        let needs_gap = j < chars.len() && (chars[j].is_alphanumeric() || chars[j] == '"');
        if needs_gap {
            result.push_str("\\;");
            i = j;
        }
        // Otherwise the source whitespace is left alone rather than swallowed. A row of a display
        // equation ends with a space and a backslash, and convert_alignment recognizes a row break
        // by that exact sequence. Dropping the space left `\text{...}\` + newline, which KaTeX
        // reads as a control space rather than a row break, and four rows of SplitMix64 rendered
        // as one line.
    }

    // Close any unclosed text brace
    if in_quote {
        result.push('}');
    }

    result
}

/// Convert Typst `sqrt(...)` to LaTeX `\sqrt{...}`
fn convert_sqrt(input: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Check for sqrt( pattern
        if i + 5 <= chars.len() {
            let slice: String = chars[i..i + 5].iter().collect();
            if slice == "sqrt(" {
                result.push_str("\\sqrt{");
                i += 5;

                // Find matching closing paren and convert content
                let mut depth = 1;
                while i < chars.len() && depth > 0 {
                    let c = chars[i];
                    if c == '(' {
                        depth += 1;
                        result.push(c);
                    } else if c == ')' {
                        depth -= 1;
                        if depth == 0 {
                            result.push('}');
                        } else {
                            result.push(c);
                        }
                    } else {
                        result.push(c);
                    }
                    i += 1;
                }
                continue;
            }
        }

        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Convert Typst math syntax patterns to LaTeX equivalents
#[allow(clippy::too_many_lines)]
fn convert_syntax(input: &str, display: bool) -> String {
    let mut result = input.to_string();

    // sqrt needs special handling: sqrt(...) -> \sqrt{...}
    result = convert_sqrt(&result);

    // Function calls - convert function-style to LaTeX (these keep parens)
    // Note: floor(), ceil(), abs() are handled by convert_floor_ceil_abs() with proper delimiters
    let function_mappings = [
        ("sin(", "\\sin("),
        ("cos(", "\\cos("),
        ("tan(", "\\tan("),
        ("log(", "\\log("),
        ("ln(", "\\ln("),
        ("exp(", "\\exp("),
        ("lim(", "\\lim("),
        ("max(", "\\max("),
        ("min(", "\\min("),
        ("sup(", "\\sup("),
        ("inf(", "\\inf("),
        ("Pr(", "\\Pr("),
        ("Phi(", "\\Phi("),
    ];

    for (typst, latex) in function_mappings {
        result = result.replace(typst, latex);
    }

    // Comparison operators (must come before word mappings to handle multi-char operators)
    // These are literal replacements, not word-boundary
    // Order matters: longer patterns first to avoid partial matches
    let operator_replacements = [
        (">=", "\\geq"),
        ("<=", "\\leq"),
        ("<-", "\\leftarrow"),
        ("->", "\\to"),
        ("!=", "\\neq"),
        (">>", "\\gg"),
        ("<<", "\\ll"),
    ];

    for (typst, latex) in operator_replacements {
        result = result.replace(typst, latex);
    }

    // Greek letters - should convert even when followed by subscript/superscript markers
    // e.g., sigma_(n,m) -> \sigma_{n,m}, epsilon_k -> \epsilon_k
    let greek_letters = [
        ("epsilon", "\\epsilon"),
        ("Lambda", "\\Lambda"),
        ("lambda", "\\lambda"),
        ("Omega", "\\Omega"),
        ("omega", "\\omega"),
        ("Sigma", "\\Sigma"),
        ("sigma", "\\sigma"),
        ("Theta", "\\Theta"),
        ("theta", "\\theta"),
        ("Gamma", "\\Gamma"),
        ("gamma", "\\gamma"),
        ("Delta", "\\Delta"),
        ("delta", "\\delta"),
        ("kappa", "\\kappa"),
        ("alpha", "\\alpha"),
        ("beta", "\\beta"),
        ("zeta", "\\zeta"),
        ("iota", "\\iota"),
        // Note: Phi and Psi need special handling - see convert_greek_capitals below
        ("eta", "\\eta"),
        ("phi", "\\phi"),
        ("chi", "\\chi"),
        ("psi", "\\psi"),
        ("rho", "\\rho"),
        ("tau", "\\tau"),
        ("Xi", "\\Xi"),
        ("Pi", "\\Pi"),
        ("xi", "\\xi"),
        ("pi", "\\pi"),
        ("nu", "\\nu"),
        ("mu", "\\mu"),
    ];

    // Symbols and operators - should NOT convert when used as subscripts
    // e.g., x_min should stay as x_min, not x_\min
    let word_mappings = [
        // Multi-char symbols first
        ("arrow.r.double", "\\Rightarrow"),
        ("arrow.l.double", "\\Leftarrow"),
        ("arrow.lr.double", "\\Leftrightarrow"),
        ("infinity", "\\infty"),
        // Typst's short spelling of the same symbol. Without it the literal "oo" reached the page.
        ("oo", "\\infty"),
        ("arrow.r", "\\rightarrow"),
        ("arrow.l", "\\leftarrow"),
        ("forall", "\\forall"),
        ("exists", "\\exists"),
        ("approx", "\\approx"),
        ("dots.c", "\\cdots"),
        ("dots.v", "\\vdots"),
        ("dots.h", "\\ldots"),
        // Bare `dots` is Typst's default spelling and must come after the qualified ones, which
        // are longer matches. Without it the word reached the page set as a product of variables.
        ("dots", "\\dots"),
        ("times", "\\times"),
        ("tilde", "\\sim"),
        ("star", "\\star"),
        ("quad", "\\quad"),
        ("qquad", "\\qquad"),
        ("xor", "\\operatorname{xor}"),
        // Math operators without parentheses (e.g., "log n" not "log(n)")
        ("log", "\\log"),
        ("sin", "\\sin"),
        ("cos", "\\cos"),
        ("tan", "\\tan"),
        ("exp", "\\exp"),
        ("max", "\\max"),
        ("min", "\\min"),
        ("sup", "\\sup"),
        ("inf", "\\inf"),
        ("lim", "\\lim"),
        ("det", "\\det"),
        ("dim", "\\dim"),
        ("ker", "\\ker"),
        ("arg", "\\arg"),
        ("gcd", "\\gcd"),
        ("lcm", "\\operatorname{lcm}"),
        ("mod", "\\mod"),
        ("ln", "\\ln"),
        ("...", "\\ldots"),
        // neq, leq, geq are handled by operator_replacements (!=, <=, >=)
        ("in", "\\in"),
        // Large operators. These were matched by the literal prefixes " sum" and "(sum", so one
        // starting a math run converted nowhere and reached the page as three italic letters.
        ("sum", "\\sum"),
        ("prod", "\\prod"),
        ("integral", "\\int"),
        // Logical connectives, which otherwise set as a product of italic letters.
        ("and", "\\land"),
        ("or", "\\lor"),
        ("not", "\\lnot"),
        ("cup", "\\cup"),
        ("cap", "\\cap"),
        ("hat", "\\hat"),
        ("bar", "\\bar"),
        ("vec", "\\vec"),
        ("dot", "\\cdot"),
        // Note: lr(|...|) is handled by convert_lr function, not here
        // Don't add |) -> \right| here as it incorrectly matches |x|) patterns
        // Typst's spelled-out forms, which must precede the two-letter abbreviations below.
        ("plus.minus", "\\pm"),
        ("minus.plus", "\\mp"),
        ("pm", "\\pm"),
        ("mp", "\\mp"),
    ];

    // Protect \text{} and \mathrm{} blocks from word-boundary replacements
    // (e.g., approx -> \approx, min -> \min should not happen inside these blocks)
    // Extract them and replace with placeholders before applying word mappings
    let mut text_blocks_syntax: Vec<String> = Vec::new();
    let protected_commands = ["\\text{", "\\mathrm{"];
    loop {
        let mut found = false;
        for cmd in &protected_commands {
            if let Some(start) = result.find(cmd) {
                let cmd_len = cmd.len();
                let after_cmd = &result[start + cmd_len..];
                if let Some(end) = find_matching_brace(after_cmd) {
                    let block_content = &result[start..=start + cmd_len + end];
                    let placeholder =
                        format!("\u{FFFD}{len}\u{FFFD}", len = text_blocks_syntax.len());
                    text_blocks_syntax.push(block_content.to_string());
                    result = format!(
                        "{}{}{}",
                        &result[..start],
                        placeholder,
                        &result[start + cmd_len + end + 1..]
                    );
                    found = true;
                    break;
                }
            }
        }
        if !found {
            break;
        }
    }

    // Process Greek letters first - they should convert even when followed by _ or ^
    // e.g., sigma_(n,m) -> \sigma_{n,m}, epsilon_k -> \epsilon_k
    for (typst, latex) in greek_letters {
        let pattern = regex::escape(typst);
        if let Ok(re) = regex::Regex::new(&pattern) {
            let mut new_result = String::new();
            let mut last_end = 0;

            for m in re.find_iter(&result) {
                let bytes = result.as_bytes();

                // Check if preceded by backslash (already converted, e.g., \sigma)
                let preceded_by_backslash = m.start() > 0 && bytes[m.start() - 1] == b'\\';

                // Check if embedded in a larger word (preceded by letter)
                let preceded_by_letter =
                    m.start() > 0 && bytes[m.start() - 1].is_ascii_alphabetic();

                // Check if embedded in a larger word (followed by letter)
                let followed_by_letter =
                    m.end() < bytes.len() && bytes[m.end()].is_ascii_alphabetic();

                // Add text before this match
                new_result.push_str(&result[last_end..m.start()]);

                // Replace only if not preceded by backslash and not embedded in word
                if preceded_by_backslash || preceded_by_letter || followed_by_letter {
                    new_result.push_str(m.as_str());
                } else {
                    new_result.push_str(latex);
                }

                last_end = m.end();
            }

            // Add remaining text
            new_result.push_str(&result[last_end..]);
            result = new_result;
        }
    }

    // Process operators and symbols - these should NOT convert when used as subscripts
    // e.g., x_min should stay as x_min, not x_\min
    for (typst, latex) in word_mappings {
        if typst.contains('(') || typst.contains('|') || typst.contains('.') {
            result = result.replace(typst, latex);
        } else {
            // Word boundary on the left only. `_` counts as a word character, which is what keeps
            // `x_min` from turning into `x_\min`, but it also suppressed the right-hand boundary
            // for an operator carrying its own script: `sum_(i=0)` matched nothing and the symbol
            // reached the page as three italic letters. The right side is checked below instead,
            // where a following letter or digit rejects the match and a script marker does not.
            let pattern = format!(r"\b{}", regex::escape(typst));
            if let Ok(re) = regex::Regex::new(&pattern) {
                let mut new_result = String::new();
                let mut last_end = 0;

                for m in re.find_iter(&result) {
                    // Check if preceded by backslash
                    let preceded_by_backslash =
                        m.start() > 0 && result.as_bytes()[m.start() - 1] == b'\\';
                    // A following letter or digit means this is part of a longer identifier.
                    let inside_identifier = result[m.end()..]
                        .chars()
                        .next()
                        .is_some_and(|c| c.is_ascii_alphanumeric());

                    // Add text before this match
                    new_result.push_str(&result[last_end..m.start()]);

                    // Add replacement or original depending on backslash
                    if preceded_by_backslash || inside_identifier {
                        new_result.push_str(m.as_str());
                    } else {
                        new_result.push_str(latex);
                    }

                    last_end = m.end();
                }

                // Add remaining text
                new_result.push_str(&result[last_end..]);
                result = new_result;
            }
        }
    }

    // Restore \text{} blocks after word mappings
    for (i, block) in text_blocks_syntax.iter().enumerate() {
        let placeholder = format!("\u{FFFD}{i}\u{FFFD}");
        result = result.replace(&placeholder, block);
    }

    // Handle Phi and Psi that aren't followed by ( (function calls handled above)
    // Use negative lookbehind to avoid double-converting \Phi to \\Phi
    result = convert_greek_capitals(&result);

    // Handle subscripts BEFORE fractions so that p_(n,m)(c) becomes p_{n,m}(c)
    // and the function call detection in fraction conversion works correctly
    result = convert_subscripts(&result);

    // Handle superscripts BEFORE fractions so that a/(1-x)^2 keeps the exponent
    // as part of the denominator
    result = convert_superscripts(&result);

    // Handle fractions: a/b -> \frac{a}{b} (display mode only)
    // Inline mode keeps flat a/b notation
    if display {
        result = convert_fractions(&result);
    }

    // The escaped solidus has passed the fraction scanners untouched; restore it before any
    // later stage can see a multi-byte character it does not expect.
    result = result.replace(LITERAL_SOLIDUS, "/");

    // Braces the author wrote are set notation; braces this converter emitted are grouping. By
    // this point every emitted brace belongs to a LaTeX command, so what remains unattached is
    // the author's and needs escaping: `{2, 3, 4}` was reaching the page as a bare `2, 3, 4`.
    result = escape_set_braces(&result);

    // Convert Typst lr() for auto-sizing delimiters
    result = convert_lr(&result);

    // Typst sizes paired delimiters to their content automatically; LaTeX does not. Without this
    // an interval like [0, 1/4] renders as a full-height fraction between half-height brackets.
    if display {
        result = size_delimiters_to_content(&result);
    }

    // Escape % for LaTeX (comment character in LaTeX, literal in Typst)
    result = result.replace('%', "\\%");

    result
}

/// Escapes braces that are set notation rather than LaTeX grouping.
///
/// A grouping brace always follows a command (`\frac{`, `\text{`), a script marker (`_{`, `^{`),
/// another grouping brace, or the close of a previous argument (`\frac{a}{b}`). Anything else
/// opening a brace is the author writing a set, and LaTeX would silently swallow it.
fn escape_set_braces(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let mut grouping_depth: Vec<bool> = Vec::new();
    for (i, &c) in chars.iter().enumerate() {
        match c {
            '{' => {
                let prev = i.checked_sub(1).map(|k| chars[k]);
                let is_grouping = matches!(prev, Some('_' | '^' | '{' | '}'))
                    || prev.is_some_and(|p| p.is_ascii_alphanumeric() || p == '\\');
                grouping_depth.push(is_grouping);
                result.push_str(if is_grouping { "{" } else { "\\{" });
            }
            '}' => {
                let is_grouping = grouping_depth.pop().unwrap_or(true);
                result.push_str(if is_grouping { "}" } else { "\\}" });
            }
            _ => result.push(c),
        }
    }
    result
}

/// Wrap paired delimiters in `\left`/`\right` when what they enclose is tall.
///
/// Applied only where the content actually grows: a fraction, a radical, a binomial, or a large
/// operator. Wrapping everything would be harmless typographically but would churn every formula
/// in the manual for no visible gain, and `\left(` around a single symbol costs a little extra
/// space in some renderers.
fn size_delimiters_to_content(input: &str) -> String {
    const TALL: [&str; 5] = ["\\frac", "\\sqrt", "\\binom", "\\sum", "\\int"];
    let bytes = input.as_bytes();
    let mut result = String::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        // Decode the character rather than casting the byte. `bytes[i] as char` yields the wrong
        // scalar for anything non-ASCII, and the `len_utf8()` that follows then advances by the
        // wrong stride, so the character is replaced by a substitute and its continuation bytes
        // are consumed as if they were characters of their own. The manual has no non-ASCII inside
        // math today, which is the only reason this has not corrupted anything.
        let c = input[i..].chars().next().expect("i is a char boundary");

        // Subscripts and superscripts are set small, and a stretched delimiter there inflates the
        // script rather than fitting it. Copy those groups through untouched.
        let script_group = ((c == '_' || c == '^') && bytes.get(i + 1) == Some(&b'{'))
            .then(|| matching_delimiter(input, i + 1, '{', '}'))
            .flatten();
        if let Some(end) = script_group {
            result.push_str(&input[i..=end]);
            i = end + 1;
            continue;
        }

        let closing = match c {
            '(' => Some(')'),
            '[' => Some(']'),
            _ => None,
        };
        // A delimiter already carrying \left, or one that is part of a LaTeX command's argument
        // list, must be left alone.
        let already_sized = result.ends_with("\\left") || result.ends_with("\\right");
        if let (Some(close), false, Some(end)) = (
            closing,
            already_sized,
            closing.and_then(|close| matching_delimiter(input, i, c, close)),
        ) {
            let inner = &input[i + 1..end];
            if TALL.iter().any(|t| inner.contains(t)) {
                let _ = write!(
                    result,
                    "\\left{c}{}\\right{close}",
                    size_delimiters_to_content(inner)
                );
                i = end + 1;
                continue;
            }
        }
        result.push(c);
        i += c.len_utf8();
    }
    result
}

/// Byte offset of the delimiter closing the one at `open_at`, or None if unbalanced.
fn matching_delimiter(s: &str, open_at: usize, open: char, close: char) -> Option<usize> {
    let mut depth = 0usize;
    for (i, c) in s[open_at..].char_indices() {
        if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0 {
                return Some(open_at + i);
            }
        }
    }
    None
}

/// Convert capital Greek letters that might not be followed by (
/// This handles cases like standalone $Phi$ while avoiding double-conversion of \Phi
fn convert_greek_capitals(input: &str) -> String {
    let mut result = input.to_string();

    // Convert Phi and Psi only when not already preceded by backslash
    // Note: Rust's regex crate doesn't support lookbehind, so we use a capture group approach
    let greek_capitals = [("Phi", "\\Phi"), ("Psi", "\\Psi")];

    for (greek, latex) in greek_capitals {
        // Match word boundary + greek letter + word boundary
        // Then filter out matches preceded by backslash manually
        let pattern = format!(r"\b{greek}\b");
        if let Ok(re) = regex::Regex::new(&pattern) {
            let mut new_result = String::new();
            let mut last_end = 0;

            for m in re.find_iter(&result) {
                // Check if preceded by backslash
                let start = m.start();
                let preceded_by_backslash = start > 0 && result.as_bytes()[start - 1] == b'\\';

                // Add text before this match
                new_result.push_str(&result[last_end..start]);

                // Add replacement or original depending on backslash
                if preceded_by_backslash {
                    new_result.push_str(m.as_str());
                } else {
                    new_result.push_str(latex);
                }

                last_end = m.end();
            }

            // Add remaining text
            new_result.push_str(&result[last_end..]);
            result = new_result;
        }
    }

    result
}

/// Convert Typst fractions to LaTeX
/// Only `/` builds a fraction in Typst; `\/` is an escaped literal slash and is carried past
/// this stage as `LITERAL_SOLIDUS`.
fn convert_fractions(input: &str) -> String {
    // Only `/` builds a fraction. The escaped form is carried as LITERAL_SOLIDUS, which no
    // scanner here recognises, so it survives to the end and becomes a plain slash.
    //
    // The loop handles fractions nested inside another fraction's parts: the scanner takes the
    // whole exponent into the denominator on the first pass and converts what is inside it on
    // the next, which is what Typst prints for `x / (1 - U)^(1/alpha)`.
    let mut result = input.to_string();
    loop {
        let next = convert_regular_fractions(&result);
        if next == result {
            return result;
        }
        result = next;
    }
}

/// Convert regular / fractions (only in simple contexts)
fn convert_regular_fractions(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '/' {
            // Skip if inside subscript context
            if is_inside_subscript_context(&chars, i) {
                result.push(chars[i]);
                i += 1;
                continue;
            }

            // Find the numerator (content before /)
            if let Some((num_start, num_end)) = find_fraction_part_before(&chars, i) {
                // Find the denominator (content after /)
                if let Some((den_start, den_end)) = find_fraction_part_after(&chars, i + 1) {
                    // Calculate how many characters to remove from result
                    // This includes the numerator plus any whitespace between numerator and slash
                    let chars_to_remove = i - num_start;
                    for _ in 0..chars_to_remove {
                        result.pop();
                    }

                    // Get numerator and denominator content
                    let num: String = chars[num_start..num_end].iter().collect();
                    let den: String = chars[den_start..den_end].iter().collect();

                    // Strip single layer of parens if the entire expression is wrapped
                    let num = strip_outer_parens(&num);
                    let den = strip_outer_parens(&den);

                    let _ = write!(result, "\\frac{{{num}}}{{{den}}}");
                    i = den_end;
                    continue;
                }
            }
            // If we couldn't convert, output the slash as-is
            result.push('/');
            i += 1;
            continue;
        }

        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Check if position is inside a subscript/superscript context
/// Returns true if we're inside x_(...) or x^(...) where the paren isn't closed yet
fn is_inside_subscript_context(chars: &[char], pos: usize) -> bool {
    let mut i = pos;
    let mut paren_depth = 0;

    // Walk backwards to find if we're inside a subscript/superscript paren
    while i > 0 {
        i -= 1;
        match chars[i] {
            ')' => paren_depth += 1,
            '(' => {
                if paren_depth > 0 {
                    paren_depth -= 1;
                } else {
                    // Found an unmatched ( - check if it's preceded by _ or ^
                    if i > 0 && (chars[i - 1] == '_' || chars[i - 1] == '^') {
                        return true;
                    }
                    // Also check for double paren like _(( which is common for order statistics
                    if i > 1 && chars[i - 1] == '(' && (chars[i - 2] == '_' || chars[i - 2] == '^')
                    {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }

    false
}

/// Strip exactly one layer of outer parentheses if the entire string is wrapped
fn strip_outer_parens(s: &str) -> &str {
    let s = s.trim();
    if s.starts_with('(') && s.ends_with(')') {
        // Verify the parens are balanced and the outer ones match
        let inner = &s[1..s.len() - 1];
        let mut depth = 0;
        for c in inner.chars() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth < 0 {
                        // The outer ) doesn't match the outer (
                        return s;
                    }
                }
                _ => {}
            }
        }
        if depth == 0 {
            return inner;
        }
    }
    s
}

/// Find the fraction numerator (content before /)
/// Returns (start, end) indices of the numerator
// Single backward scanner over the character stream: splitting the bracket/function
// state across helpers would spread one state machine over several signatures.
#[allow(clippy::too_many_lines)]
fn find_fraction_part_before(chars: &[char], slash_pos: usize) -> Option<(usize, usize)> {
    if slash_pos == 0 {
        return None;
    }

    let mut start = slash_pos - 1;

    // Skip trailing whitespace
    while start > 0 && chars[start].is_whitespace() {
        start -= 1;
    }

    // Handle edge case: all whitespace before slash
    if chars[start].is_whitespace() {
        return None;
    }

    // end is one past the last meaningful character (after skipping whitespace)
    let end = start + 1;

    // If we hit a closing brace, find the matching open brace and continue backwards
    // to include the full expression (e.g., x_{min} where } ends a subscript group)
    if chars[start] == '}' {
        let mut brace_depth = 1;
        while start > 0 && brace_depth > 0 {
            start -= 1;
            match chars[start] {
                '}' => brace_depth += 1,
                '{' => brace_depth -= 1,
                _ => {}
            }
        }
        if brace_depth != 0 {
            return None;
        }
        // Continue backwards to include subscript/superscript marker and variable name
        // e.g., for x_{min}, after matching {min} we need to include x_
        while start > 0
            && (chars[start - 1].is_alphanumeric()
                || chars[start - 1] == '_'
                || chars[start - 1] == '\\'
                || chars[start - 1] == '^'
                || chars[start - 1] == '}')
        {
            start -= 1;
            // If we hit another closing brace, find its matching open
            if chars[start] == '}' {
                let mut bd = 1;
                while start > 0 && bd > 0 {
                    start -= 1;
                    match chars[start] {
                        '}' => bd += 1,
                        '{' => bd -= 1,
                        _ => {}
                    }
                }
            }
        }
        return Some((start, end));
    }

    // If we hit a closing paren, find the matching open
    if chars[start] == ')' {
        let mut depth = 1;
        while start > 0 && depth > 0 {
            start -= 1;
            match chars[start] {
                ')' => depth += 1,
                '(' => depth -= 1,
                _ => {}
            }
        }
        if depth != 0 {
            return None;
        }
        // Include function name before the paren (e.g., "f(x)", "p_{n,m}(c)", "Drift^2(x)")
        // This allows function calls to be fraction numerators
        // Note: Include '^' to handle superscripts like Drift^2(x) where ^2 is part of the term
        while start > 0
            && (chars[start - 1].is_alphanumeric()
                || chars[start - 1] == '_'
                || chars[start - 1] == '\\'
                || chars[start - 1] == '}'
                || chars[start - 1] == '^')
        {
            start -= 1;
            // If we hit a closing brace, find matching open (for subscripts like p_{n,m})
            if chars[start] == '}' {
                let mut brace_depth = 1;
                while start > 0 && brace_depth > 0 {
                    start -= 1;
                    match chars[start] {
                        '}' => brace_depth += 1,
                        '{' => brace_depth -= 1,
                        _ => {}
                    }
                }
                // Continue to include content before the brace (subscript marker, variable name)
                while start > 0
                    && (chars[start - 1].is_alphanumeric()
                        || chars[start - 1] == '_'
                        || chars[start - 1] == '\\'
                        || chars[start - 1] == '^')
                {
                    start -= 1;
                }
            }
        }
        return Some((start, end));
    }

    // If we hit a closing bracket, find the matching open
    if chars[start] == ']' {
        let mut depth = 1;
        while start > 0 && depth > 0 {
            start -= 1;
            match chars[start] {
                ']' => depth += 1,
                '[' => depth -= 1,
                _ => {}
            }
        }
        if depth != 0 {
            return None;
        }
        // Include function name before the bracket (e.g., "Var[...]", "Drift^2[...]")
        while start > 0
            && (chars[start - 1].is_alphanumeric()
                || chars[start - 1] == '_'
                || chars[start - 1] == '\\'
                || chars[start - 1] == '}'
                || chars[start - 1] == '^')
        {
            start -= 1;
            // If we hit a closing brace, find matching open (for \text{Var}[...])
            if chars[start] == '}' {
                let mut brace_depth = 1;
                while start > 0 && brace_depth > 0 {
                    start -= 1;
                    match chars[start] {
                        '}' => brace_depth += 1,
                        '{' => brace_depth -= 1,
                        _ => {}
                    }
                }
                // Continue to include the command before the brace
                while start > 0
                    && (chars[start - 1].is_alphabetic()
                        || chars[start - 1] == '\\'
                        || chars[start - 1] == '^')
                {
                    start -= 1;
                }
            }
        }
        return Some((start, end));
    }

    // Otherwise, collect alphanumeric and common math chars
    // Don't include { or } - those indicate LaTeX command boundaries
    // A decimal point counts when it sits between two digits, so that 4.8 stays one number.
    while start > 0
        && (chars[start - 1].is_alphanumeric()
            || chars[start - 1] == '_'
            || chars[start - 1] == '\\'
            || (chars[start - 1] == '.'
                && start >= 2
                && chars[start - 2].is_ascii_digit()
                && chars[start].is_ascii_digit()))
    {
        start -= 1;
    }

    if start < end {
        Some((start, end))
    } else {
        None
    }
}

/// Find the fraction denominator (content after /)
/// Returns (start, end) indices of the denominator
// Forward mirror of `find_fraction_part_before`; kept as one scanner for the same reason.
#[allow(clippy::too_many_lines)]
fn find_fraction_part_after(chars: &[char], start_pos: usize) -> Option<(usize, usize)> {
    if start_pos >= chars.len() {
        return None;
    }

    let mut start = start_pos;

    // Skip leading whitespace
    while start < chars.len() && chars[start].is_whitespace() {
        start += 1;
    }

    if start >= chars.len() {
        return None;
    }

    let mut end = start;

    // If we hit an opening paren, find the matching close
    if chars[start] == '(' {
        let mut depth = 1;
        end = start + 1;
        while end < chars.len() && depth > 0 {
            match chars[end] {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => {}
            }
            end += 1;
        }
        if depth != 0 {
            return None;
        }

        // Include trailing factorial operator(s)
        while end < chars.len() && chars[end] == '!' {
            end += 1;
        }

        // Include trailing superscript (e.g., (1-x)^{2} should be one term)
        // Superscripts are already converted to ^{...} by now
        if end < chars.len() && chars[end] == '^' {
            end += 1;
            if end < chars.len() && chars[end] == '{' {
                // Find matching close brace
                let mut brace_depth = 1;
                end += 1;
                while end < chars.len() && brace_depth > 0 {
                    match chars[end] {
                        '{' => brace_depth += 1,
                        '}' => brace_depth -= 1,
                        _ => {}
                    }
                    end += 1;
                }
            } else if end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '-') {
                // Simple superscript like ^2 or ^n or ^-1
                end += 1;
            }
        }

        return Some((start, end));
    }

    // Collect alphanumeric, backslash, and underscores. A decimal point counts when it sits
    // between two digits: without that, a denominator of 3.8 ends at the 3 and the .8 lands
    // outside the fraction, which is silently wrong rather than visibly broken.
    while end < chars.len()
        && (chars[end].is_alphanumeric()
            || chars[end] == '_'
            || chars[end] == '\\'
            || (chars[end] == '.'
                && end > start
                && chars[end - 1].is_ascii_digit()
                && end + 1 < chars.len()
                && chars[end + 1].is_ascii_digit()))
    {
        end += 1;
    }

    // Handle \lvert...\rvert as a single unit
    // After collecting alphanumeric, check if we have \lvert and find matching \rvert
    let collected: String = chars[start..end].iter().collect();
    if collected.ends_with("\\lvert") {
        // Find matching \rvert
        let remaining: String = chars[end..].iter().collect();
        if let Some(right_pos) = remaining.find("\\rvert") {
            end += right_pos + 6; // 6 = length of "\rvert"
        }
    }

    // If we hit an opening brace, include content up to matching close
    // This handles LaTeX commands like \operatorname{...}
    // Loop to handle multiple brace pairs (e.g., \binom{...}{...}, \frac{...}{...})
    while end < chars.len() && chars[end] == '{' {
        let mut depth = 1;
        end += 1;
        while end < chars.len() && depth > 0 {
            match chars[end] {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ => {}
            }
            end += 1;
        }
    }

    // If we hit an opening bracket, include content up to matching close
    // This handles function notation like Var[...], E[...]
    if end < chars.len() && chars[end] == '[' {
        let mut depth = 1;
        end += 1;
        while end < chars.len() && depth > 0 {
            match chars[end] {
                '[' => depth += 1,
                ']' => depth -= 1,
                _ => {}
            }
            end += 1;
        }
    }

    // Handle superscript after the base term (e.g., \operatorname{Drift}^2)
    // Track if we've seen a superscript, as it affects function call handling
    let mut had_superscript = false;
    if end < chars.len() && chars[end] == '^' {
        had_superscript = true;
        end += 1;
        if end < chars.len() && chars[end] == '{' {
            // Superscript with braces: ^{...}
            let mut brace_depth = 1;
            end += 1;
            while end < chars.len() && brace_depth > 0 {
                match chars[end] {
                    '{' => brace_depth += 1,
                    '}' => brace_depth -= 1,
                    _ => {}
                }
                end += 1;
            }
        } else if end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '-') {
            // Simple superscript like ^2 or ^n or ^-1
            end += 1;
        }
    }

    // Handle function call arguments after superscript (e.g., Drift^2(T_1, X))
    // If we had a superscript and see (, include the function arguments
    // If no superscript and see (, don't include it (it's a separate function call)
    if end < chars.len() && chars[end] == '(' {
        if had_superscript {
            // Include function arguments as part of the term
            let mut depth = 1;
            end += 1;
            while end < chars.len() && depth > 0 {
                match chars[end] {
                    '(' => depth += 1,
                    ')' => depth -= 1,
                    _ => {}
                }
                end += 1;
            }
        } else {
            // No superscript, so this is a separate function call - don't include
            return None;
        }
    }

    if end > start {
        Some((start, end))
    } else {
        None
    }
}

/// Convert Typst subscripts to LaTeX
fn convert_subscripts(input: &str) -> String {
    let mut result = input.to_string();

    // Handle x_(expr) -> x_{expr} with proper brace conversion
    // Find each _( and convert to _{ and change matching ) to }
    result = convert_paren_to_brace(&result, "_");

    // Handle _\text{...} -> _{\text{...}} (subscripts with text blocks)
    result = wrap_text_subscripts(&result, "_");

    // Wrap multi-character identifiers after _ in braces:
    // n_min -> n_{min} (Typst treats "min" as one subscript token, LaTeX does not)
    result = wrap_multichar_scripts(&result, "_");

    result
}

/// Convert Typst superscripts to LaTeX
fn convert_superscripts(input: &str) -> String {
    let mut result = input.to_string();

    // Handle x^(expr) -> x^{expr} with proper brace conversion
    result = convert_paren_to_brace(&result, "^");

    // Handle ^\text{...} -> ^{\text{...}} (superscripts with text blocks)
    result = wrap_text_subscripts(&result, "^");

    // Wrap multi-character identifiers after ^ in braces
    result = wrap_multichar_scripts(&result, "^");

    result
}

/// Wrap multi-character alphabetic identifiers after _ or ^ in braces.
/// In Typst math, `n_min` means n subscript "min", but in LaTeX it means n subscript "m" + "in".
/// This converts `_abc` to `_{abc}` (only for 2+ letter sequences not already braced).
fn wrap_multichar_scripts(input: &str, prefix: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = input.chars().collect();
    let prefix_chars: Vec<char> = prefix.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Check for prefix character
        if chars[i..].starts_with(&prefix_chars) {
            let after = i + prefix_chars.len();
            // Skip if already braced or parenthesized or followed by backslash (LaTeX command)
            if after < chars.len()
                && (chars[after] == '{' || chars[after] == '(' || chars[after] == '\\')
            {
                result.push(chars[i]);
                i += 1;
                continue;
            }
            // Count consecutive alphanumeric chars. Digits belong here as much as letters:
            // LaTeX takes one character after ^, so 10^308 set 10 cubed followed by 08, which
            // is a different number rather than a visibly broken one.
            let ident_start = after;
            let mut j = after;
            while j < chars.len() && chars[j].is_ascii_alphanumeric() {
                j += 1;
            }
            let ident_len = j - ident_start;
            if ident_len >= 2 {
                result.extend(prefix_chars.iter());
                result.push('{');
                result.extend(chars[ident_start..j].iter());
                result.push('}');
                i = j;
            } else {
                result.push(chars[i]);
                i += 1;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}

/// Convert prefix( to prefix{ and matching ) to }
/// Also handles nested parens like x_((1)) -> x_{(1)}
fn convert_paren_to_brace(input: &str, prefix: &str) -> String {
    let pattern = format!("{prefix}(");
    let mut result = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Check if we're at prefix(
        let remaining: String = chars[i..].iter().collect();
        if remaining.starts_with(&pattern) {
            result.push_str(prefix);
            result.push('{');
            i += pattern.len();

            // Find matching closing paren
            let mut depth = 1;
            while i < chars.len() && depth > 0 {
                let c = chars[i];
                if c == '(' {
                    depth += 1;
                    result.push(c);
                } else if c == ')' {
                    depth -= 1;
                    if depth == 0 {
                        result.push('}');
                    } else {
                        result.push(c);
                    }
                } else {
                    result.push(c);
                }
                i += 1;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}

/// Wrap \text{} blocks after subscript/superscript markers in braces
/// prefix\text{...} -> prefix{\text{...}}
fn wrap_text_subscripts(input: &str, prefix: &str) -> String {
    let pattern = format!("{prefix}\\text{{");
    let mut result = String::new();
    let mut remaining = input;

    while let Some(pos) = remaining.find(&pattern) {
        // Add content before the match
        result.push_str(&remaining[..pos]);

        // Find the closing brace of \text{...}
        let after_prefix = &remaining[pos + prefix.len()..];
        if let Some(text_start) = after_prefix.find("\\text{") {
            let after_text = &after_prefix[text_start + 6..];
            if let Some(brace_end) = find_matching_brace(after_text) {
                // Extract the full \text{...} and wrap in braces
                let text_content = &after_prefix[..=text_start + 6 + brace_end];
                result.push_str(prefix);
                result.push('{');
                result.push_str(text_content);
                result.push('}');
                remaining = &remaining[pos + prefix.len() + text_start + 6 + brace_end + 1..];
                continue;
            }
        }

        // Fallback: no proper match, just add the prefix
        result.push_str(prefix);
        remaining = &remaining[pos + prefix.len()..];
    }

    result.push_str(remaining);
    result
}

/// Convert Typst `lr()` to LaTeX `\left \right`
///
/// Typst `lr()` creates auto-sizing delimiters. For example:
/// - `lr(|x|)` -> `\left\lvert x\right\rvert`
/// - `lr((a+b))` -> `\left(a+b\right)`
// Byte offsets throughout, for the reason given on convert_op.
fn convert_lr(input: &str) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < input.len() {
        if input[i..].starts_with("lr(") {
            // Found lr(, now find the matching closing paren
            let start = i + 3; // After "lr("
            if let Some(end) = find_matching_paren(&input[start..]) {
                let inner = &input[start..start + end];

                // The inner content starts with a delimiter (e.g., "(", "|", "[")
                // and ends with the matching delimiter
                if let Some(first_char) = inner.chars().next() {
                    // Use \lvert/\rvert for | to avoid conflicts with markdown tables
                    let (left_delim, right_delim) = match first_char {
                        '(' => ("\\left(", "\\right)"),
                        '|' => ("\\left\\lvert ", " \\right\\rvert"),
                        '[' => ("\\left[", "\\right]"),
                        '{' => ("\\left\\{", "\\right\\}"),
                        _ => ("", ""),
                    };

                    if !left_delim.is_empty() {
                        // Remove the outer delimiters from inner content
                        let inner_content = &inner[1..inner.len() - 1];
                        result.push_str(left_delim);
                        result.push_str(inner_content);
                        result.push_str(right_delim);
                        i = start + end + 1; // Skip past the closing )
                        continue;
                    }
                }

                // Fallback: just include the inner content without lr()
                result.push_str(inner);
                i = start + end + 1;
                continue;
            }
        }

        let c = input[i..].chars().next().expect("i is a char boundary");
        result.push(c);
        i += c.len_utf8();
    }

    result
}

/// Convert Typst line breaks and alignment to LaTeX
///
/// In Typst:
/// - `\` at end of line is a line break
/// - `&` is used for alignment
///
/// In LaTeX:
/// - `\\` is a line break
/// - `&` for alignment requires an environment like `aligned`
fn convert_alignment(input: &str) -> String {
    // Check if input contains alignment markers
    let has_alignment = input.contains('&');
    let has_line_breaks = input.contains(" \\\n") || input.ends_with(" \\");

    if !has_alignment && !has_line_breaks {
        return input.to_string();
    }

    let mut result = input.to_string();

    // Convert Typst line breaks (single \) to LaTeX line breaks (\\)
    // Typst uses " \" at end of line, LaTeX uses "\\"
    // Be careful not to double-convert already escaped backslashes
    result = result.replace(" \\\n", " \\\\\n");
    if result.ends_with(" \\") {
        result = result[..result.len() - 1].to_string() + "\\\\";
    }

    // If there's alignment, wrap in aligned environment
    if has_alignment {
        result = format!("\\begin{{aligned}}\n{}\n\\end{{aligned}}", result.trim());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_simple_fraction() {
        let result = convert_fractions("a/b");
        assert_eq!(result, "\\frac{a}{b}");
    }

    #[test]
    fn convert_subscript() {
        // Single character subscripts are NOT wrapped in braces
        // KaTeX handles x_i correctly without braces, and braces cause MDX issues
        let result = convert_subscripts("x_i");
        assert_eq!(result, "x_i");
    }

    #[test]
    fn convert_superscript() {
        // Single character superscripts are NOT wrapped in braces
        let result = convert_superscripts("x^2");
        assert_eq!(result, "x^2");
    }

    #[test]
    fn convert_text_in_quotes() {
        // The thin space is deliberate: a literal space carries no width in math mode, so
        // without it the words and the variable render as one run of letters.
        let result = convert_text_quotes(r#""if" n "is odd""#);
        assert_eq!(result, "\\text{if}\\;n\\;\\text{is odd}");
    }

    #[test]
    fn convert_with_definitions() {
        let mut defs = HashMap::new();
        defs.insert("Center".to_string(), "\\operatorname{Center}".to_string());

        let result = typst_to_latex("Center(x)", &defs, true);
        assert!(result.contains("\\operatorname{Center}"));
    }

    #[test]
    fn convert_comparison_operators() {
        let defs = HashMap::new();
        let result = typst_to_latex("1 <= i <= n", &defs, true);
        // Should produce single backslash: \leq
        assert_eq!(result, "1 \\leq i \\leq n");
    }

    #[test]
    fn convert_attach_with_comparison() {
        let defs = HashMap::new();
        let result = typst_to_latex("attach(Median, b: 1 <= i <= n)", &defs, true);
        // Should produce \underset{1 \leq i \leq n}{Median}
        assert!(result.contains("\\underset{1 \\leq i \\leq n}{Median}"));
    }

    #[test]
    fn convert_explicit_fraction_in_subscript() {
        let defs = HashMap::new();
        // Typst: x_(((n+1)\/2)) should become x_{(\frac{n+1}{2})}
        let result = typst_to_latex("x_(((n+1)/2))", &defs, true);
        assert_eq!(result, "x_{(\\frac{n+1}{2})}");
    }

    #[test]
    fn convert_complex_expression_with_fractions() {
        let defs = HashMap::new();
        // Typst: (x_((n\/2)) + x_((n\/2+1))) / 2
        let result = typst_to_latex("(x_((n\\/2)) + x_((n\\/2+1))) / 2", &defs, true);
        // Should convert the \/ inside subscripts to \frac, and the outer / to \frac too
        // Expected: \frac{(x_{(\frac{n}{2})} + x_{(\frac{n}{2}+1)})}{2}
        // Or simpler: (x_{(\frac{n}{2})} + x_{(\frac{n}{2}+1)}) / 2
        eprintln!("Result: {result}");
        // For now, just check it contains \frac and no ⁄ markers
        assert!(
            result.contains("\\frac"),
            "Result should contain \\frac: {result}"
        );
        assert!(
            !result.contains('\u{2044}'),
            "Result should not contain fraction slash marker: {result}"
        );
    }

    #[test]
    fn convert_cases_with_text() {
        let defs = HashMap::new();
        // Typst cases with text quotes
        let input = r#"cases(
  x & "if" n "is odd",
  y & "if" n "is even"
)"#;
        let result = typst_to_latex(input, &defs, true);
        eprintln!("Cases result: {result}");
        assert!(
            result.contains("\\begin{cases}"),
            "Should contain \\begin{{cases}}: {result}"
        );
        assert!(
            result.contains("\\end{cases}"),
            "Should contain \\end{{cases}}: {result}"
        );
        assert!(
            result.contains("\\text{is even}"),
            "Should contain \\text{{is even}}: {result}"
        );
        // Make sure \end{cases} is NOT inside the text
        assert!(
            !result.contains("\\text{is \\end{cases}"),
            "\\end{{cases}} should not be inside \\text{{}}: {result}"
        );
    }

    #[test]
    fn convert_median_cases_formula() {
        let defs = HashMap::new();
        // Full Median formula with cases and fractions
        let input = r#"Median(vx) = cases(
  x_(((n+1)\/2)) & "if" n "is odd",
  (x_((n\/2)) + x_((n\/2+1))) / 2 & "if" n "is even"
)"#;
        let result = typst_to_latex(input, &defs, true);
        eprintln!("Median result: {result}");
        // Check structure is correct
        assert!(
            result.contains("\\begin{cases}"),
            "Should contain \\begin{{cases}}: {result}"
        );
        assert!(
            result.contains("\\end{cases}"),
            "Should contain \\end{{cases}}: {result}"
        );
        assert!(
            result.contains("\\text{is even}"),
            "Should contain \\text{{is even}}: {result}"
        );
        assert!(
            !result.contains("\\text{is \\end{cases}"),
            "\\end{{cases}} should not be inside \\text{{}}: {result}"
        );
    }

    #[test]
    fn convert_simple_outer_fraction() {
        // Test outer fraction: (a + b) / 2 should become \frac{a + b}{2}
        let input = "(a + b) / 2";
        let result = convert_regular_fractions(input);
        eprintln!("Simple fraction result: {result}");
        assert_eq!(result, "\\frac{a + b}{2}");
    }

    #[test]
    fn convert_mathbf_fraction() {
        // Test: \mathbf{x} / \mathbf{y} should not be converted (too complex)
        // Or if converted: \frac{\mathbf{x}}{\mathbf{y}}
        let input = "\\mathbf{x} / \\mathbf{y}";
        let result = convert_regular_fractions(input);
        eprintln!("Mathbf fraction result: {result}");
        // Should NOT produce \mathbf{\frac{...
        assert!(
            !result.contains("\\mathbf{\\frac"),
            "Should not put \\frac inside \\mathbf"
        );
    }

    #[test]
    fn convert_explicit_mathbf_fraction() {
        let defs = HashMap::new();
        // Using explicit fraction marker (from \/)
        let input = "\\mathbf{x} \u{2044} \\mathbf{y}";
        let result = typst_to_latex(input, &defs, true);
        eprintln!("Explicit mathbf fraction result: {result}");
        // Should NOT produce \mathbf{\frac{...
        assert!(
            !result.contains("\\mathbf{\\frac"),
            "Should not put \\frac inside \\mathbf: {result}"
        );
    }

    #[test]
    fn definitions_not_applied_inside_text() {
        let mut defs = HashMap::new();
        defs.insert(
            "Dominance".to_string(),
            "\\operatorname{Dominance}".to_string(),
        );

        // "Dominance" in quotes should become \text{Dominance}, NOT \text{\operatorname{Dominance}}
        let input = r#""Dominance""#;
        let result = typst_to_latex(input, &defs, true);
        assert_eq!(
            result, "\\text{Dominance}",
            "Definitions should not be applied inside \\text{{}}"
        );

        // But unquoted Dominance should get the definition applied
        let input2 = "Dominance(x, y)";
        let result2 = typst_to_latex(input2, &defs, true);
        assert!(
            result2.contains("\\operatorname{Dominance}"),
            "Definitions should be applied outside \\text{{}}"
        );
    }

    #[test]
    fn convert_blackboard_bold() {
        let defs = HashMap::new();
        let result = typst_to_latex("bb(1)", &defs, true);
        assert_eq!(result, "\\mathbb{1}");
    }

    #[test]
    fn convert_blackboard_bold_in_sum() {
        let defs = HashMap::new();
        let result = typst_to_latex("sum bb(1)(x > y)", &defs, true);
        assert!(
            result.contains("\\mathbb{1}"),
            "Should convert bb(1): {result}"
        );
    }

    #[test]
    fn convert_binomial() {
        let defs = HashMap::new();
        let result = typst_to_latex("binom(n, k)", &defs, true);
        assert_eq!(result, "\\binom{n}{k}");
    }

    #[test]
    fn convert_binomial_complex() {
        let defs = HashMap::new();
        let result = typst_to_latex("binom(n+m, n)", &defs, true);
        assert_eq!(result, "\\binom{n+m}{n}");
    }

    #[test]
    fn convert_floor() {
        let defs = HashMap::new();
        let result = typst_to_latex("floor(x)", &defs, true);
        assert_eq!(result, "\\left\\lfloor x \\right\\rfloor");
    }

    #[test]
    fn convert_floor_complex() {
        let defs = HashMap::new();
        let result = typst_to_latex("floor((N+1)/2)", &defs, true);
        assert!(result.contains("\\lfloor"), "Should have lfloor: {result}");
        assert!(result.contains("\\rfloor"), "Should have rfloor: {result}");
    }

    #[test]
    fn convert_ceil() {
        let defs = HashMap::new();
        let result = typst_to_latex("ceil(x)", &defs, true);
        assert_eq!(result, "\\left\\lceil x \\right\\rceil");
    }

    #[test]
    fn convert_abs() {
        let defs = HashMap::new();
        let result = typst_to_latex("abs(x)", &defs, true);
        assert_eq!(result, "\\left\\lvert x \\right\\rvert");
    }

    #[test]
    fn convert_abs_complex() {
        let defs = HashMap::new();
        let result = typst_to_latex("abs(x_i - x_j)", &defs, true);
        assert!(result.contains("\\lvert"), "Should have lvert: {result}");
        assert!(result.contains("\\rvert"), "Should have rvert: {result}");
    }

    #[test]
    fn convert_abs_in_fraction_denominator() {
        // Test that abs() in fraction denominator stays intact
        // This was a bug where \lvert...\rvert got split by fraction conversion
        let defs = HashMap::new();
        let result = typst_to_latex("a / abs(b)", &defs, true);
        eprintln!("Result: {result}");
        assert!(
            result.contains("\\lvert") && result.contains("\\rvert"),
            "abs should be intact in denominator: {result}"
        );
    }

    #[test]
    fn convert_pr_function() {
        let defs = HashMap::new();
        let result = typst_to_latex("Pr(X > 0)", &defs, true);
        assert!(result.contains("\\Pr("), "Should have \\Pr(: {result}");
    }

    #[test]
    fn convert_phi_function() {
        let defs = HashMap::new();
        let result = typst_to_latex("Phi(z)", &defs, true);
        assert!(result.contains("\\Phi("), "Should have \\Phi(: {result}");
    }

    #[test]
    fn convert_phi_standalone() {
        let defs = HashMap::new();
        // Standalone Phi without parentheses should also be converted
        let result = typst_to_latex("where Phi denotes", &defs, true);
        assert!(result.contains("\\Phi"), "Should have \\Phi: {result}");
        assert!(
            !result.contains("\\\\Phi"),
            "Should not have double backslash: {result}"
        );
    }

    #[test]
    fn convert_phi_no_double_convert() {
        let defs = HashMap::new();
        // Phi( is converted first, then standalone Phi shouldn't double-convert the \Phi
        let result = typst_to_latex("Phi(z) and Phi", &defs, true);
        eprintln!("Result: {result}");
        assert!(
            result.contains("\\Phi("),
            "Should have \\Phi( function: {result}"
        );
        assert!(
            !result.contains("\\\\Phi"),
            "Should not have double backslash: {result}"
        );
    }

    #[test]
    fn convert_fraction_with_brackets() {
        let defs = HashMap::new();
        // Test fraction with bracket notation like Var[...] / Var[...]
        let result = typst_to_latex("\"Var\"[X] / \"Var\"[Y]", &defs, true);
        eprintln!("Bracket fraction result: {result}");
        assert!(result.contains("\\frac"), "Should have \\frac: {result}");
        assert!(
            !result.contains("\\frac{]}"),
            "Should not have \\frac{{]}}: {result}"
        );
    }

    #[test]
    fn convert_upright() {
        let defs = HashMap::new();
        let result = typst_to_latex("upright(\"mean\")", &defs, true);
        assert_eq!(result, "\\mathrm{mean}");
    }

    #[test]
    fn convert_upright_no_quotes() {
        let defs = HashMap::new();
        let result = typst_to_latex("upright(stdDev)", &defs, true);
        assert_eq!(result, "\\mathrm{stdDev}");
    }

    #[test]
    fn convert_subscript_with_text() {
        let defs = HashMap::new();
        // k_"left" -> first converts to k_\text{left}, then should wrap in braces
        let result = typst_to_latex("k_\"left\"", &defs, true);
        assert_eq!(result, "k_{\\text{left}}");
    }

    #[test]
    fn convert_fraction_with_binom() {
        let defs = HashMap::new();
        // Test the problematic case: 1\/binom(12, 6) should become \frac{1}{\binom{12}{6}}
        let result = typst_to_latex("1/binom(12, 6)", &defs, true);
        eprintln!("Result: {result}");
        assert!(
            result.contains("\\binom{12}{6}"),
            "Should convert binom: {result}"
        );
        assert!(result.contains("\\frac"), "Should have frac: {result}");
    }

    #[test]
    fn convert_definition_with_subscript() {
        let mut defs = HashMap::new();
        defs.insert("Drift".to_string(), "\\operatorname{Drift}".to_string());

        // Drift_"baseline" should have Drift converted to \operatorname{Drift}
        // even though _ is a word character in regex
        let result = typst_to_latex("Drift_\"baseline\"(T, X)", &defs, true);
        eprintln!("Result: {result}");
        assert!(
            result.contains("\\operatorname{Drift}"),
            "Drift should be converted to \\operatorname{{Drift}}: {result}"
        );
    }

    #[test]
    fn convert_definition_with_superscript() {
        let mut defs = HashMap::new();
        defs.insert("Drift".to_string(), "\\operatorname{Drift}".to_string());

        // Drift^2 should have Drift converted to \operatorname{Drift}
        let result = typst_to_latex("Drift^2", &defs, true);
        eprintln!("Result: {result}");
        assert!(
            result.contains("\\operatorname{Drift}"),
            "Drift should be converted to \\operatorname{{Drift}}: {result}"
        );
    }

    #[test]
    fn convert_pmean_definition() {
        let mut defs = HashMap::new();
        defs.insert("pmean".to_string(), "\\mathrm{mean}".to_string());

        let result = typst_to_latex("pmean", &defs, true);
        assert_eq!(result, "\\mathrm{mean}");
    }

    #[test]
    fn convert_pstddev_definition() {
        let mut defs = HashMap::new();
        defs.insert("pstddev".to_string(), "\\mathrm{stdDev}".to_string());

        let result = typst_to_latex("pstddev", &defs, true);
        assert_eq!(result, "\\mathrm{stdDev}");
    }

    #[test]
    fn convert_distribution_with_parameters() {
        let mut defs = HashMap::new();
        defs.insert(
            "Additive".to_string(),
            "\\underline{\\operatorname{Additive}}".to_string(),
        );
        defs.insert("pmean".to_string(), "\\mathrm{mean}".to_string());
        defs.insert("pstddev".to_string(), "\\mathrm{stdDev}".to_string());

        // Test Additive(pmean, pstddev) conversion
        let result = typst_to_latex("Additive(pmean, pstddev)", &defs, true);
        eprintln!("Result: {result}");
        assert!(
            result.contains("\\underline{\\operatorname{Additive}}"),
            "Additive should be converted: {result}"
        );
        assert!(
            result.contains("\\mathrm{mean}"),
            "pmean should be converted: {result}"
        );
        assert!(
            result.contains("\\mathrm{stdDev}"),
            "pstddev should be converted: {result}"
        );
    }

    #[test]
    fn convert_pstddev_with_superscript() {
        let mut defs = HashMap::new();
        defs.insert("pstddev".to_string(), "\\mathrm{stdDev}".to_string());

        // pstddev^2 should convert pstddev correctly
        let result = typst_to_latex("pstddev^2", &defs, true);
        eprintln!("Result: {result}");
        assert!(
            result.contains("\\mathrm{stdDev}"),
            "pstddev should be converted: {result}"
        );
    }

    #[test]
    fn convert_cmad_definition() {
        let mut defs = HashMap::new();
        defs.insert("cmad".to_string(), "c_{\\mathrm{mad}}".to_string());

        let result = typst_to_latex("cmad", &defs, true);
        assert_eq!(result, "c_{\\mathrm{mad}}");
    }

    #[test]
    fn convert_cspr_definition() {
        let mut defs = HashMap::new();
        defs.insert("cspr".to_string(), "c_{\\mathrm{spr}}".to_string());

        let result = typst_to_latex("cspr", &defs, true);
        assert_eq!(result, "c_{\\mathrm{spr}}");
    }

    #[test]
    fn convert_approxdist_definition() {
        let mut defs = HashMap::new();
        // Use \text{approx} to avoid word_mappings converting approx to \approx
        defs.insert("approxdist".to_string(), "\\sim\\text{approx}".to_string());

        let result = typst_to_latex("X approxdist Y", &defs, true);
        eprintln!("Result: {result}");
        assert!(
            result.contains("\\sim\\text{approx}"),
            "approxdist should be converted: {result}"
        );
    }

    #[test]
    fn convert_all_distribution_parameters() {
        let mut defs = HashMap::new();
        defs.insert("pmean".to_string(), "\\mathrm{mean}".to_string());
        defs.insert("pstddev".to_string(), "\\mathrm{stdDev}".to_string());
        defs.insert("plogmean".to_string(), "\\mathrm{logMean}".to_string());
        defs.insert("plogstddev".to_string(), "\\mathrm{logStdDev}".to_string());
        defs.insert("pmin".to_string(), "\\mathrm{min}".to_string());
        defs.insert("pmax".to_string(), "\\mathrm{max}".to_string());
        defs.insert("pshape".to_string(), "\\mathrm{shape}".to_string());
        defs.insert("prate".to_string(), "\\mathrm{rate}".to_string());

        // Test each parameter
        assert_eq!(typst_to_latex("pmean", &defs, true), "\\mathrm{mean}");
        assert_eq!(typst_to_latex("pstddev", &defs, true), "\\mathrm{stdDev}");
        assert_eq!(typst_to_latex("plogmean", &defs, true), "\\mathrm{logMean}");
        assert_eq!(
            typst_to_latex("plogstddev", &defs, true),
            "\\mathrm{logStdDev}"
        );
        assert_eq!(typst_to_latex("pmin", &defs, true), "\\mathrm{min}");
        assert_eq!(typst_to_latex("pmax", &defs, true), "\\mathrm{max}");
        assert_eq!(typst_to_latex("pshape", &defs, true), "\\mathrm{shape}");
        assert_eq!(typst_to_latex("prate", &defs, true), "\\mathrm{rate}");
    }

    #[test]
    fn convert_expression_with_pstddev_division() {
        let mut defs = HashMap::new();
        defs.insert("pstddev".to_string(), "\\mathrm{stdDev}".to_string());

        // Test pstddev/sqrt(n) pattern - note that fraction conversion doesn't work
        // when the numerator is a LaTeX command result (the converter sees \mathrm{...}
        // and doesn't recognize it as a valid numerator for fractions)
        let result = typst_to_latex("pstddev/sqrt(n)", &defs, true);
        eprintln!("Result: {result}");
        assert!(
            result.contains("\\mathrm{stdDev}"),
            "pstddev should be converted: {result}"
        );
        assert!(
            result.contains("\\sqrt{n}"),
            "sqrt should be converted: {result}"
        );
    }

    #[test]
    fn convert_pstddev_in_complex_formula() {
        let mut defs = HashMap::new();
        defs.insert("pstddev".to_string(), "\\mathrm{stdDev}".to_string());

        // From the notes chapter: sqrt(2) dot pstddev
        let result = typst_to_latex("sqrt(2) dot pstddev", &defs, true);
        eprintln!("Result: {result}");
        assert!(
            result.contains("\\mathrm{stdDev}"),
            "pstddev should be converted: {result}"
        );
        assert!(
            result.contains("\\sqrt{2}"),
            "sqrt should be converted: {result}"
        );
        assert!(
            result.contains("\\cdot"),
            "dot should be converted: {result}"
        );
    }

    #[test]
    fn convert_complex_additive_expression() {
        let mut defs = HashMap::new();
        defs.insert(
            "Additive".to_string(),
            "\\underline{\\operatorname{Additive}}".to_string(),
        );
        defs.insert("pmean".to_string(), "\\mathrm{mean}".to_string());
        defs.insert("pstddev".to_string(), "\\mathrm{stdDev}".to_string());

        // From notes: Additive(0, sqrt(2) dot pstddev)
        let result = typst_to_latex("Additive(0, sqrt(2) dot pstddev)", &defs, true);
        eprintln!("Result: {result}");
        assert!(
            result.contains("\\underline{\\operatorname{Additive}}"),
            "Additive should be converted: {result}"
        );
        assert!(
            result.contains("\\mathrm{stdDev}"),
            "pstddev should be converted: {result}"
        );
        assert!(
            result.contains("\\sqrt{2}"),
            "sqrt should be converted: {result}"
        );
        assert!(
            result.contains("\\cdot"),
            "dot should be converted: {result}"
        );
    }

    #[test]
    fn convert_pmean_not_inside_text() {
        let mut defs = HashMap::new();
        defs.insert("pmean".to_string(), "\\mathrm{mean}".to_string());

        // pmean in quotes should NOT be converted (it becomes \text{pmean})
        let result = typst_to_latex("\"pmean\"", &defs, true);
        assert_eq!(
            result, "\\text{pmean}",
            "pmean inside quotes should not be converted: {result}"
        );

        // But pmean outside quotes should be converted
        let result2 = typst_to_latex("pmean", &defs, true);
        assert_eq!(result2, "\\mathrm{mean}");
    }

    #[test]
    fn convert_assignment_arrow() {
        let defs = HashMap::new();
        let result = typst_to_latex("x <- x + 1", &defs, true);
        assert_eq!(result, "x \\leftarrow x + 1");
    }

    #[test]
    fn convert_xor_operator() {
        let defs = HashMap::new();
        let result = typst_to_latex("x xor y", &defs, true);
        assert_eq!(result, "x \\operatorname{xor} y");
    }

    #[test]
    fn convert_log_operator() {
        let defs = HashMap::new();
        // Standalone log should become \log
        let result = typst_to_latex("O(n log n)", &defs, true);
        assert_eq!(result, "O(n \\log n)");

        // log with parentheses should also work
        let result2 = typst_to_latex("log(x)", &defs, true);
        assert_eq!(result2, "\\log(x)");
    }

    #[test]
    fn convert_math_operators() {
        let defs = HashMap::new();
        // Test various math operators
        assert_eq!(typst_to_latex("sin x", &defs, true), "\\sin x");
        assert_eq!(typst_to_latex("cos x", &defs, true), "\\cos x");
        assert_eq!(typst_to_latex("max(a, b)", &defs, true), "\\max(a, b)");
        assert_eq!(typst_to_latex("min(a, b)", &defs, true), "\\min(a, b)");
        assert_eq!(typst_to_latex("ln x", &defs, true), "\\ln x");
        assert_eq!(typst_to_latex("exp x", &defs, true), "\\exp x");
    }

    #[test]
    fn convert_quad_spacing() {
        let defs = HashMap::new();
        let result = typst_to_latex("a quad b", &defs, true);
        assert_eq!(result, "a \\quad b");
    }

    #[test]
    fn convert_right_shift() {
        let defs = HashMap::new();
        let result = typst_to_latex("x >> 30", &defs, true);
        assert_eq!(result, "x \\gg 30");
    }

    #[test]
    fn convert_left_shift() {
        let defs = HashMap::new();
        let result = typst_to_latex("x << 3", &defs, true);
        assert_eq!(result, "x \\ll 3");
    }

    #[test]
    fn convert_splitmix64_formula() {
        let defs = HashMap::new();
        // Test the actual formula from the randomization chapter
        let result = typst_to_latex(
            "x <- (x xor (x >> 30)) times \"0xbf58476d1ce4e5b9\"",
            &defs,
            true,
        );
        eprintln!("Result: {result}");
        assert!(
            result.contains("\\leftarrow"),
            "Should have leftarrow: {result}"
        );
        assert!(
            result.contains("\\operatorname{xor}"),
            "Should have xor operator: {result}"
        );
        assert!(result.contains("\\gg"), "Should have >> as \\gg: {result}");
        assert!(result.contains("\\times"), "Should have times: {result}");
    }

    #[test]
    fn convert_fnv1a_hash_formula() {
        let defs = HashMap::new();
        // Test with quad spacing
        let result = typst_to_latex(
            "\"hash\" <- \"0xcbf29ce484222325\" quad \"(offset basis)\"",
            &defs,
            true,
        );
        eprintln!("Result: {result}");
        assert!(
            result.contains("\\leftarrow"),
            "Should have leftarrow: {result}"
        );
        assert!(result.contains("\\quad"), "Should have quad: {result}");
    }

    #[test]
    fn convert_function_call_with_subscript_as_numerator() {
        // Test that p_{n,m}(c) / x creates a proper fraction with p_{n,m}(c) as numerator
        // This was a bug where (c) alone became the numerator
        let defs = HashMap::new();
        let result = typst_to_latex("p_(n,m)(c) / binom(n+m, n)", &defs, true);
        eprintln!("Result: {result}");
        // The result should have p_{n,m}(c) as the numerator
        assert!(
            result.contains("\\frac{p_{n,m}(c)}{"),
            "Should have p_{{n,m}}(c) as fraction numerator: {result}"
        );
        assert!(
            result.contains("\\binom{n+m}{n}"),
            "Should have binom as denominator: {result}"
        );
    }

    #[test]
    fn convert_fraction_with_superscript_in_denominator() {
        // Test that (1-U)^{2} stays together as denominator
        let defs = HashMap::new();
        let result = typst_to_latex("x_min / (1 - U)^(2)", &defs, true);
        eprintln!("Result: {result}");
        // The entire (1 - U)^{2} should be in the denominator
        assert!(
            result.contains("\\frac{x_{min}}{(1 - U)^{2}}"),
            "Superscript should be part of denominator: {result}"
        );
    }

    #[test]
    fn convert_fraction_with_nested_fraction_exponent() {
        // Test x_min / (1 - U)^(1/alpha) - the exponent has a fraction inside
        let defs = HashMap::new();
        let result = typst_to_latex("x_min / (1 - U)^(1/alpha)", &defs, true);
        eprintln!("Result: {result}");
        // The denominator should include the entire (1-U)^{...} expression
        // Note: alpha gets converted to \alpha by Greek letter conversion
        assert!(
            result.contains("\\frac{x_{min}}{(1 - U)^{\\frac{1}{\\alpha}}}"),
            "Exponent with fraction should be part of denominator: {result}"
        );
    }

    #[test]
    fn convert_factorial_in_denominator() {
        // Test that (n+m)! has the factorial as part of the term
        let defs = HashMap::new();
        let result = typst_to_latex("(n! dot m!) / (n+m)!", &defs, true);
        eprintln!("Result: {result}");
        // The factorial should be inside the fraction, not outside
        assert!(
            result.contains("\\frac{"),
            "Should create a fraction: {result}"
        );
        // The denominator should be (n+m)! not just (n+m)
        assert!(
            result.contains("{(n+m)!}") || result.contains("/(n+m)!"),
            "Factorial should be part of denominator: {result}"
        );
        // Make sure ! is not dangling outside
        assert!(
            !result.ends_with("}!"),
            "Factorial should not be outside the fraction: {result}"
        );
    }

    #[test]
    fn convert_explicit_fraction_factorial() {
        // Test explicit fraction with factorial
        let defs = HashMap::new();
        let result = typst_to_latex("(n! dot m!) / (n+m)!", &defs, true);
        eprintln!("Result: {result}");
        // Should be \frac{n! \cdot m!}{(n+m)!}
        assert!(
            result.contains("\\frac{n! \\cdot m!}{(n+m)!}"),
            "Factorial should be inside denominator: {result}"
        );
    }

    #[test]
    fn convert_fraction_with_superscript_function_call() {
        // Test Drift^2(T_2, X) / Drift^2(T_1, X) pattern
        // The ^2 superscript should be included as part of the numerator/denominator
        let mut defs = HashMap::new();
        defs.insert("Drift".to_string(), "\\operatorname{Drift}".to_string());

        let result = typst_to_latex("Drift^2(T_2, X) / Drift^2(T_1, X)", &defs, true);
        eprintln!("Result: {result}");

        // Should create a proper fraction with superscripts intact
        assert!(
            result.contains(
                "\\frac{\\operatorname{Drift}^2(T_2, X)}{\\operatorname{Drift}^2(T_1, X)}"
            ),
            "Superscript function calls should be proper fraction parts: {result}"
        );

        // Should NOT have the broken pattern where ^2 is split
        assert!(
            !result.contains("^\\frac"),
            "Should not have superscript followed by frac: {result}"
        );
    }

    #[test]
    fn convert_sample_size_formula() {
        // Test the actual formula from efficiency-drift.typ
        let mut defs = HashMap::new();
        defs.insert("Drift".to_string(), "\\operatorname{Drift}".to_string());

        let result = typst_to_latex(
            "n_\"new\" = n_\"original\" dot Drift^2(T_2, X) / Drift^2(T_1, X)",
            &defs,
            true,
        );
        eprintln!("Result: {result}");

        // Should have proper text subscripts
        assert!(
            result.contains("n_{\\text{new}}"),
            "Should have n_{{\\text{{new}}}}: {result}"
        );
        assert!(
            result.contains("n_{\\text{original}}"),
            "Should have n_{{\\text{{original}}}}: {result}"
        );

        // Should have proper fraction
        assert!(
            result.contains(
                "\\frac{\\operatorname{Drift}^2(T_2, X)}{\\operatorname{Drift}^2(T_1, X)}"
            ),
            "Should have proper fraction with Drift^2: {result}"
        );
    }

    #[test]
    fn convert_greek_with_subscript_parens() {
        // Greek letters followed by subscript in parentheses: sigma_(n,m)
        let defs = HashMap::new();
        let result = typst_to_latex("sigma_(n,m)(d)", &defs, true);
        assert_eq!(
            result, "\\sigma_{n,m}(d)",
            "sigma with subscript parens should convert: {result}"
        );
    }

    #[test]
    fn convert_greek_with_simple_subscript() {
        // Greek letters followed by simple subscript: epsilon_k
        let defs = HashMap::new();
        let result = typst_to_latex("epsilon_k", &defs, true);
        assert_eq!(
            result, "\\epsilon_k",
            "epsilon with subscript should convert: {result}"
        );
    }

    #[test]
    fn convert_greek_with_superscript() {
        // Greek letters followed by superscript: sigma^2
        let defs = HashMap::new();
        let result = typst_to_latex("sigma^2", &defs, true);
        assert_eq!(
            result, "\\sigma^2",
            "sigma with superscript should convert: {result}"
        );
    }

    #[test]
    fn convert_pairwise_margin_formula() {
        // Test the actual formula from manual/pairwise-margin/pairwise-margin-algorithm.typ
        let defs = HashMap::new();
        let result = typst_to_latex("sigma_(n,m)(d) = sum_(k|d) epsilon_k dot k", &defs, true);
        eprintln!("Result: {result}");
        assert!(
            result.contains("\\sigma_{n,m}(d)"),
            "sigma with subscript should convert: {result}"
        );
        assert!(
            result.contains("\\sum_{k|d}"),
            "sum with subscript condition should convert: {result}"
        );
        assert!(
            result.contains("\\epsilon_k"),
            "epsilon with subscript should convert: {result}"
        );
    }

    #[test]
    fn greek_not_converted_inside_word() {
        // Greek letter names embedded in larger words should NOT be converted
        let defs = HashMap::new();

        // "thesigma" should stay as-is (sigma is embedded)
        let result = typst_to_latex("thesigma", &defs, true);
        assert_eq!(
            result, "thesigma",
            "Embedded sigma should not convert: {result}"
        );

        // "sigmaX" should stay as-is (sigma followed by letter)
        let result = typst_to_latex("sigmaX", &defs, true);
        assert_eq!(
            result, "sigmaX",
            "sigma followed by letter should not convert: {result}"
        );

        // But "sigma X" should convert (space separator)
        let result = typst_to_latex("sigma X", &defs, true);
        assert_eq!(
            result, "\\sigma X",
            "sigma with space should convert: {result}"
        );
    }

    #[test]
    fn greek_standalone_converts() {
        // Standalone Greek letters should convert
        let defs = HashMap::new();
        assert_eq!(typst_to_latex("sigma", &defs, true), "\\sigma");
        assert_eq!(typst_to_latex("epsilon", &defs, true), "\\epsilon");
        assert_eq!(typst_to_latex("alpha", &defs, true), "\\alpha");
        assert_eq!(typst_to_latex("beta", &defs, true), "\\beta");
    }

    #[test]
    fn greek_with_operators_converts() {
        // Greek letters adjacent to operators should convert
        let defs = HashMap::new();
        assert_eq!(
            typst_to_latex("sigma + tau", &defs, true),
            "\\sigma + \\tau"
        );
        assert_eq!(typst_to_latex("(sigma)", &defs, true), "(\\sigma)");
        assert_eq!(typst_to_latex("sigma,tau", &defs, true), "\\sigma,\\tau");
    }

    #[test]
    fn convert_chained_explicit_fractions() {
        // From additive.typ: (sqrt(2) dot cmad dot pstddev\/sqrt(n))\/(z_(0.75) dot pstddev)
        // The first \/ expands to \frac{B}{\sqrt{n}}, making result longer
        // than the original chars span. The second \/ must still work correctly.
        let mut defs = HashMap::new();
        defs.insert("cmad".to_string(), "c_{\\mathrm{mad}}".to_string());
        defs.insert("pstddev".to_string(), "\\mathrm{stdDev}".to_string());
        let input =
            "(sqrt(2) dot z_(0.75) dot cmad dot pstddev\\/sqrt(n))\\/(z_(0.75) dot pstddev)";
        let result = typst_to_latex(input, &defs, true);
        eprintln!("chained explicit fractions: {result}");
        assert!(
            !result.contains("\\sqrt{2\\frac"),
            "sqrt brace must close before frac: {result}"
        );
    }

    #[test]
    fn inline_simple_fraction_stays_flat() {
        let defs = HashMap::new();
        let result = typst_to_latex("a/b", &defs, false);
        assert_eq!(result, "a/b");
    }

    /// A decimal number is one term on either side of a fraction slash.
    ///
    /// The scanners collected alphanumerics and stopped at the point, so `(2t - 4.8) \/ 3.8`
    /// produced a denominator of 3 with a stray `.8` left outside the fraction. That renders as
    /// something a reader can misread as a different expression rather than as visible breakage.
    #[test]
    fn decimal_numbers_survive_a_fraction() {
        let defs = HashMap::new();
        let result = typst_to_latex("u = (2 t - 4.8) / 3.8", &defs, true);
        assert!(
            result.contains("{3.8}"),
            "denominator lost its decimal part: {result}"
        );
        assert!(
            !result.contains(".8 "),
            "a decimal fragment escaped the fraction: {result}"
        );
        let flipped = typst_to_latex("3.8 / (2 t - 4.8)", &defs, true);
        assert!(
            flipped.contains("{3.8}"),
            "numerator lost its decimal part: {flipped}"
        );
    }

    /// Set membership is an operator, not the English word.
    #[test]
    fn set_membership_converts() {
        let defs = HashMap::new();
        let result = typst_to_latex("s = t^2 in [0, 1 / 4]", &defs, true);
        assert!(result.contains("\\in"), "in was not converted: {result}");
    }

    /// Paired delimiters take the height of what they enclose.
    ///
    /// Typst sizes them automatically, LaTeX does not, so an interval holding a fraction rendered
    /// as full-height content between half-height brackets.
    #[test]
    fn delimiters_grow_with_tall_content() {
        let defs = HashMap::new();
        let interval = typst_to_latex("s = t^2 in [0, 1 / 4]", &defs, true);
        assert!(
            interval.contains("\\left[") && interval.contains("\\right]"),
            "interval brackets did not grow: {interval}"
        );

        // Nothing tall inside: leave it alone rather than churn every formula in the manual.
        let plain = typst_to_latex("f(x) = (a + b)", &defs, true);
        assert!(
            !plain.contains("\\left("),
            "a plain group was needlessly stretched: {plain}"
        );

        // Scripts are set small, where a stretched delimiter inflates rather than fits.
        let script = typst_to_latex("x_((n+1) / 2)", &defs, true);
        assert!(
            !script.contains("\\left("),
            "a subscript group was stretched: {script}"
        );

        // Inline mode keeps fractions flat, so nothing needs stretching there either.
        let inline = typst_to_latex("s in [0, 1 / 4]", &defs, false);
        assert!(
            !inline.contains("\\left["),
            "inline math was stretched: {inline}"
        );
    }

    /// Braces the author wrote are set notation and must survive to the page.
    ///
    /// LaTeX treats a bare brace as grouping and drops it, so `{2, 3, 4}` rendered as `2, 3, 4`
    /// and every set in the manual silently lost its delimiters.
    #[test]
    fn set_braces_are_escaped_but_grouping_braces_are_not() {
        let defs = HashMap::new();

        let set = typst_to_latex("n in {2, 3, 4}", &defs, true);
        assert!(
            set.contains("\\{2, 3, 4\\}"),
            "set braces were swallowed: {set}"
        );

        // Braces the converter emits itself belong to commands and must stay bare.
        for (input, want) in [
            ("(a + b) / 2", "\\frac{a + b}{2}"),
            ("x_min", "x_{min}"),
            ("binom(n, k)", "\\binom{n}{k}"),
            ("\"if\" n", "\\text{if}"),
        ] {
            let result = typst_to_latex(input, &defs, true);
            assert!(
                result.contains(want),
                "a grouping brace was escaped in {input:?}: {result}"
            );
            assert!(
                !result.contains("\\\\{"),
                "a grouping brace was escaped in {input:?}: {result}"
            );
        }
    }

    /// A multi-character exponent or subscript is one group.
    ///
    /// LaTeX takes a single character after `^`, so `10^308` set ten cubed followed by 08. That is
    /// a different number rather than visibly broken output, which is why it survived review.
    #[test]
    fn multi_character_scripts_are_grouped() {
        let defs = HashMap::new();
        for (input, want) in [
            ("10^308", "10^{308}"),
            ("2^64", "2^{64}"),
            ("x_min", "x_{min}"),
            ("8.475 dot 10^307", "10^{307}"),
        ] {
            let result = typst_to_latex(input, &defs, true);
            assert!(
                result.contains(want),
                "expected {want} in the conversion of {input:?}: {result}"
            );
        }
        // A single character needs no braces and should not gain any.
        let single = typst_to_latex("x^2", &defs, true);
        assert_eq!(
            single, "x^2",
            "a one-character exponent was needlessly braced"
        );
    }

    /// A quoted word keeps its separation from what follows.
    ///
    /// Typst renders `"if" n "is odd"` with gaps; LaTeX closes `\text{if}` and starts the next
    /// token immediately, so the page showed a single run of letters.
    #[test]
    fn quoted_words_stay_separated() {
        let defs = HashMap::new();
        let result = typst_to_latex("\"if\" n \"is odd\"", &defs, true);
        assert!(
            result.contains("\\text{if}\\;n"),
            "the quoted word ran into the variable: {result}"
        );
        // Punctuation should sit tight against the word rather than after a space.
        let punctuated = typst_to_latex("\"for\", x", &defs, true);
        assert!(
            !punctuated.contains("\\text{for}\\;,"),
            "a space was inserted before punctuation: {punctuated}"
        );
    }

    /// Typst's two division forms mean opposite things, and the conversion must preserve that.
    ///
    /// `/` is the fraction operator and `\/` is an escaped literal solidus. This converter used to
    /// expand `\/` into a fraction and leave `/` flat in some contexts, so the website contradicted
    /// the PDF on the same source. Verified against Typst itself: `$a \/ b$` prints `a/b` and
    /// `$a / b$` prints a built-up fraction.
    #[test]
    fn the_two_division_forms_keep_their_meanings() {
        let defs = HashMap::new();

        let built = typst_to_latex("a / b", &defs, true);
        assert!(
            built.contains("\\frac{a}{b}"),
            "the fraction operator must build a fraction in display: {built}"
        );

        let literal = typst_to_latex("a \\/ b", &defs, true);
        assert!(
            !literal.contains("\\frac"),
            "the escaped solidus must stay flat in display: {literal}"
        );
        assert!(
            !literal.contains('\u{2044}'),
            "the internal marker leaked into the output: {literal}"
        );
        assert!(
            literal.contains('/'),
            "the escaped solidus must survive as a slash: {literal}"
        );

        // Inline keeps everything flat, so the two agree there.
        for input in ["a / b", "a \\/ b"] {
            let inline = typst_to_latex(input, &defs, false);
            assert!(
                !inline.contains("\\frac") && !inline.contains('\u{2044}'),
                "inline math must stay flat for {input:?}: {inline}"
            );
        }

        // The shape from manual/median/median.typ, which is where the disagreement showed.
        let mixed = typst_to_latex("x_(((n+1)\\/2)) + (a + b) / 2", &defs, true);
        assert!(
            mixed.contains("x_{((n+1)/2)}"),
            "the escaped form must stay flat inside a subscript: {mixed}"
        );
        assert!(
            mixed.contains("\\frac{a + b}{2}"),
            "the operator form must build a fraction beside it: {mixed}"
        );
    }

    /// Typst's spelled-out symbol names convert, not only their abbreviations.
    #[test]
    fn spelled_out_symbol_names_convert() {
        let defs = HashMap::new();
        for (input, want) in [("x plus.minus y", "\\pm"), ("x minus.plus y", "\\mp")] {
            let result = typst_to_latex(input, &defs, true);
            assert!(
                result.contains(want),
                "expected {want} in the conversion of {input:?}: {result}"
            );
            assert!(
                !result.contains("plus.minus") && !result.contains("minus.plus"),
                "the spelled-out name leaked through in {input:?}: {result}"
            );
        }
    }

    /// Ellipses are a symbol in every spelling Typst accepts.
    #[test]
    fn ellipsis_converts_in_every_spelling() {
        let defs = HashMap::new();
        for (input, want) in [
            ("1 \\/ 2, 1 \\/ 6, dots", "\\dots"),
            ("a_1, dots.h, a_n", "\\ldots"),
            ("a_1 dots.c a_n", "\\cdots"),
            ("x_1, ..., x_n", "\\ldots"),
        ] {
            let result = typst_to_latex(input, &defs, true);
            assert!(
                result.contains(want),
                "expected {want} in the conversion of {input:?}: {result}"
            );
        }
    }

    /// Both spellings of infinity reach the page as a symbol.
    #[test]
    fn infinity_converts_in_either_spelling() {
        let defs = HashMap::new();
        for input in ["x in (-oo, +oo)", "x in (-infinity, +infinity)"] {
            let result = typst_to_latex(input, &defs, true);
            assert!(
                result.contains("\\infty"),
                "infinity survived as text in {input:?}: {result}"
            );
            assert!(
                !result.contains("oo"),
                "the short spelling leaked through in {input:?}: {result}"
            );
        }
    }

    #[test]
    fn inline_explicit_fraction_stays_flat() {
        let defs = HashMap::new();
        let result = typst_to_latex("a\\/b", &defs, false);
        assert_eq!(result, "a/b");
    }

    /// A row of a display equation that ends with a quoted string still ends with a row break.
    ///
    /// The thin-space rule after a closing quote used to swallow every following space, including
    /// the one that separates the row from its trailing backslash. `convert_alignment` recognizes a
    /// break by that exact sequence, so the break was never doubled and `KaTeX` read the lone
    /// backslash as a control space: the four rows of `SplitMix64` rendered as one long line.
    /// A quoted word after a LaTeX control word gets no thin space.
    ///
    /// The gap is inserted before the word mappings run, when a Typst operator is still a bare
    /// word and looks like an atom, so the decision has to be revisited once the mapping has
    /// happened. Control words carry their own spacing; `\begin{cases}` in particular indents
    /// the first branch relative to the others when a gap follows it.
    /// Non-ASCII inside a display equation survives conversion.
    ///
    /// Three finders returned character positions while their callers sliced by bytes, and the
    /// sized-delimiter pass decoded characters by casting a byte. Both defects are invisible on
    /// ASCII and silent on anything else: the first mis-slices, the second substitutes a
    /// replacement character and eats the continuation bytes.
    #[test]
    fn non_ascii_inside_math_survives() {
        let defs = HashMap::new();
        for input in [
            // The multibyte character has to sit INSIDE the argument list, before the separator:
            // that is where a character position and a byte offset disagree.
            "binom(1 \\/ 2, k)",
            "binom(n \u{2248} m, k \\/ 2)",
            "attach(1 \\/ 2, b: \"\u{03B1}\")",
            "alpha = binom(n, k) dot 1 \\/ 2",
            "lr((1 \\/ 2 + \u{2248} + sqrt(x)))",
            "\"\u{2248}\" quad x = 1 \\/ 2",
        ] {
            for display in [true, false] {
                let result = typst_to_latex(input, &defs, display);
                assert!(
                    !result.is_empty(),
                    "conversion produced nothing for {input:?}"
                );
                assert!(
                    !result.contains('\u{FFFD}'),
                    "a character was replaced during conversion of {input:?}: {result}"
                );
                assert!(
                    !result.contains('\u{2044}'),
                    "the fraction marker survived conversion in {input:?}: {result}"
                );
            }
        }
    }

    #[test]
    fn a_control_word_does_not_earn_a_thin_space() {
        let defs = HashMap::new();
        for (input, forbidden) in [
            ("metric = Spread and \"seed\" != \"null\"", r"\land\;"),
            ("x quad \"where\" y", r"\quad\;"),
            ("a times \"b\"", r"\times\;"),
        ] {
            for display in [true, false] {
                let result = typst_to_latex(input, &defs, display);
                assert!(
                    !result.contains(forbidden),
                    "stray thin space after a control word in {input:?}: {result}"
                );
            }
        }

        // The exception: text-setting commands produce an atom, and a word after one still needs
        // separating from it.
        let kept = typst_to_latex("\"if\" \"then\"", &defs, true);
        assert!(
            kept.contains(r"\;"),
            "the gap between two set words was dropped: {kept}"
        );
    }

    #[test]
    fn a_row_ending_in_a_quoted_string_keeps_its_break() {
        let defs = HashMap::new();
        let input = "x &<- x + \"0x9e3779b97f4a7c15\" \\\nz &<- z + 1";
        let result = typst_to_latex(input, &defs, true);
        assert!(result.contains("\\\\"), "the row break was lost: {result}");
        assert!(
            !result.contains("}\\\n"),
            "a lone backslash survived, which KaTeX reads as a control space: {result}"
        );
    }

    /// op(...) and lr(...) survive a multibyte character earlier in the expression.
    ///
    /// Both walked character positions and then sliced the input by those positions as if they
    /// were byte offsets. Before the first multibyte character the two agree, so the defect is
    /// invisible; after one, the slice either lands mid-character and panics or cuts in the wrong
    /// place. The fraction marker this converter uses internally is U+2044, three bytes wide.
    #[test]
    fn op_and_lr_survive_a_multibyte_character() {
        let defs = HashMap::new();
        for input in [
            "alpha \\/ beta + op(\"erfc\")(t)",
            "1 \\/ 2 + lr(|x - y|)",
            "sum_(i=1)^n 1 \\/ n dot lr((x_i - macron(x)))",
        ] {
            for display in [true, false] {
                let result = typst_to_latex(input, &defs, display);
                assert!(
                    !result.is_empty(),
                    "conversion produced nothing for {input:?}"
                );
                assert!(
                    !result.contains('\u{2044}'),
                    "the fraction marker survived conversion in {input:?}: {result}"
                );
            }
        }
    }

    /// A group containing a multi-byte character must not be mis-sliced.
    ///
    /// `find_matching_paren` returned a character index while its callers sliced with it as a byte
    /// offset. The two agree on ASCII, so this went unnoticed until a section put an explicit
    /// fraction inside a converted group: `\/` becomes U+2044 before these callers run, so the
    /// group then holds a three-byte character and the slice lands mid-character.
    #[test]
    fn multibyte_inside_a_group_does_not_panic() {
        let defs = HashMap::new();
        // The first is the display equation that actually panicked, from the ExpFunction section:
        // a delimiter-taking function whose argument holds two explicit fractions, so the second
        // marker sits past the point the mis-scaled index lands on.
        for input in [
            "k = floor(y \\/ ln 2 + 1 \\/ 2), quad r = y - k ln 2, quad e^y = 2^k dot e^r",
            "e^y = (p dot 2^\"half\") dot 2^(k - \"half\"), quad \"half\" = \"trunc\"(k \\/ 2)",
            "abs(r) <= (ln 2) \\/ 2 approx 0.347",
            "AdditiveCumulative(z) = cases((1 + \"erf\"(t)) \\/ 2 & \"for\" z >= 0, \"erfc\"(t) \\/ 2 & \"for\" z < 0,)",
        ] {
            for display in [true, false] {
                let result = typst_to_latex(input, &defs, display);
                assert!(
                    !result.is_empty(),
                    "conversion produced nothing for {input:?}"
                );
                assert!(
                    !result.contains('\u{2044}'),
                    "the fraction marker survived conversion in {input:?}: {result}"
                );
            }
        }
    }

    #[test]
    fn inline_complex_fraction_stays_flat() {
        let defs = HashMap::new();
        let result = typst_to_latex("(a + b) / 2", &defs, false);
        assert!(
            !result.contains("\\frac"),
            "Inline should not produce \\frac: {result}"
        );
        assert!(
            result.contains('/'),
            "Inline should keep flat slash: {result}"
        );
    }

    #[test]
    fn display_fraction_still_uses_frac() {
        let defs = HashMap::new();
        let result = typst_to_latex("a/b", &defs, true);
        assert_eq!(result, "\\frac{a}{b}");
    }
}
