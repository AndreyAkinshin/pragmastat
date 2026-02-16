//! Convert Typst math syntax to LaTeX for `KaTeX` rendering
//!
//! Typst uses a cleaner math syntax that needs conversion for web display.
//! This module handles the most common patterns used in pragmastat.

use std::collections::HashMap;
use std::fmt::Write;

/// Convert Typst math content to LaTeX string
pub fn typst_to_latex(typst_math: &str, definitions: &HashMap<String, String>) -> String {
    let mut result = typst_math.to_string();

    // Convert Typst \/ (explicit fraction) to a marker that won't be confused with regular /
    // Use Unicode fraction slash (U+2044) as temporary marker
    result = result.replace("\\/", "\u{2044}");

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
    result = convert_syntax(&result);

    // Convert Typst line breaks and handle alignment
    result = convert_alignment(&result);

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
    sorted_defs.sort_by(|(a, _), (b, _)| b.len().cmp(&a.len()));

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

/// Find matching closing brace, accounting for nesting
fn find_matching_brace(s: &str) -> Option<usize> {
    let mut depth = 1;
    for (i, c) in s.chars().enumerate() {
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
fn convert_op(input: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        // Check for op( pattern
        if i + 3 < chars.len() && chars[i] == 'o' && chars[i + 1] == 'p' && chars[i + 2] == '(' {
            // Found op(, now look for the content
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
        result.push(chars[i]);
        i += 1;
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

/// Find comma separator in function arguments, respecting nesting
fn find_comma_in_args(s: &str) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in s.chars().enumerate() {
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

    // Process floor() calls
    result = convert_delimiter_func(&result, "floor(", "\\lfloor ", " \\rfloor");

    // Process ceil() calls
    result = convert_delimiter_func(&result, "ceil(", "\\lceil ", " \\rceil");

    // Process abs() calls (use \lvert/\rvert to avoid | conflicting with markdown tables)
    result = convert_delimiter_func(&result, "abs(", "\\lvert ", " \\rvert");

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
        if let Some(end_char) = find_matching_paren(after_cases) {
            // Convert character index to byte index for proper string slicing
            // find_matching_paren returns character position, not byte position
            let chars: Vec<char> = after_cases.chars().collect();
            let inner: String = chars[..end_char].iter().collect();

            // Convert inner content:
            // - & stays as &
            // - , at end of line becomes \\
            let latex_inner = inner
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(|line| line.trim_end_matches(','))
                .collect::<Vec<_>>()
                .join(" \\\\ ");

            let latex_cases = format!("\\begin{{cases}} {latex_inner} \\end{{cases}}");

            // Calculate byte offset for the content after the closing paren
            let after_end: String = chars[end_char + 1..].iter().collect();

            result = format!("{}{}{}", &result[..start_byte], latex_cases, after_end);
        }
    }

    result
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

/// Find the first comma that's not escaped (not preceded by \)
fn find_unescaped_comma(s: &str) -> Option<usize> {
    let chars: Vec<char> = s.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if c == ',' {
            // Check if preceded by backslash
            if i == 0 || chars[i - 1] != '\\' {
                return Some(i);
            }
        }
    }
    None
}

/// Find matching closing parenthesis, accounting for nesting
fn find_matching_paren(s: &str) -> Option<usize> {
    let mut depth = 1;
    for (i, c) in s.chars().enumerate() {
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
fn convert_text_quotes(input: &str) -> String {
    let mut result = String::new();
    let chars = input.chars().peekable();
    let mut in_quote = false;

    for c in chars {
        if c == '"' {
            if in_quote {
                result.push('}');
                in_quote = false;
            } else {
                result.push_str("\\text{");
                in_quote = true;
            }
        } else {
            result.push(c);
        }
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
fn convert_syntax(input: &str) -> String {
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

    // Special operators that need \prefix form
    let operator_mappings = [
        (" sum", " \\sum"),
        (" prod", " \\prod"),
        ("(sum", "(\\sum"),
        ("(prod", "(\\prod"),
    ];

    for (typst, latex) in operator_mappings {
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
        ("arrow.r", "\\rightarrow"),
        ("arrow.l", "\\leftarrow"),
        ("forall", "\\forall"),
        ("exists", "\\exists"),
        ("approx", "\\approx"),
        ("dots.c", "\\cdots"),
        ("dots.v", "\\vdots"),
        ("dots.h", "\\ldots"),
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
        ("cup", "\\cup"),
        ("cap", "\\cap"),
        ("hat", "\\hat"),
        ("bar", "\\bar"),
        ("vec", "\\vec"),
        ("dot", "\\cdot"),
        // Note: lr(|...|) is handled by convert_lr function, not here
        // Don't add |) -> \right| here as it incorrectly matches |x|) patterns
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
                let preceded_by_backslash =
                    m.start() > 0 && bytes[m.start() - 1] == b'\\';

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
            // Use word boundary matching - treats _ as word character so x_min won't convert
            let pattern = format!(r"\b{}\b", regex::escape(typst));
            if let Ok(re) = regex::Regex::new(&pattern) {
                let mut new_result = String::new();
                let mut last_end = 0;

                for m in re.find_iter(&result) {
                    // Check if preceded by backslash
                    let preceded_by_backslash =
                        m.start() > 0 && result.as_bytes()[m.start() - 1] == b'\\';

                    // Add text before this match
                    new_result.push_str(&result[last_end..m.start()]);

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

    // Handle fractions: a/b -> \frac{a}{b}
    // Must run AFTER subscript/superscript conversion for proper parsing
    result = convert_fractions(&result);

    // Convert Typst lr() for auto-sizing delimiters
    result = convert_lr(&result);

    // Escape % for LaTeX (comment character in LaTeX, literal in Typst)
    result = result.replace('%', "\\%");

    result
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
/// Handles two cases:
/// 1. Explicit fractions marked with ⁄ (from Typst \/) - always converted
/// 2. Regular / - only converted in simple contexts, not inside subscripts
fn convert_fractions(input: &str) -> String {
    // First pass: convert all explicit fractions (⁄ marker from Typst \/)
    // These are always converted regardless of context
    // Loop until no more changes to handle nested explicit fractions
    // (e.g., a \/ b^(c\/d) has two explicit fractions, inner one gets included
    // in denominator and needs another pass to convert)
    let mut result = input.to_string();
    loop {
        let next = convert_explicit_fractions(&result);
        if next == result {
            break;
        }
        result = next;
    }

    // Second pass: convert regular / fractions (only in simple contexts)
    result = convert_regular_fractions(&result);

    result
}

/// Convert explicit Typst fractions (marked with ⁄ from \/)
fn convert_explicit_fractions(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '\u{2044}' {
            // Find the numerator (content before the fraction slash)
            if let Some((num_start, num_end)) = find_fraction_part_before(&chars, i) {
                // Find the denominator (content after the fraction slash)
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
                    // Process only one ⁄ per call to avoid a position
                    // mismatch: the \frac expansion may be longer than the
                    // original chars span, making chars_to_remove wrong
                    // for any subsequent ⁄. The outer loop in
                    // convert_fractions re-calls with a fresh chars array.
                    let tail: String = chars[den_end..].iter().collect();
                    result.push_str(&tail);
                    return result;
                }
            }
            // If we couldn't convert, output as regular slash
            result.push('/');
            i += 1;
            continue;
        }

        result.push(chars[i]);
        i += 1;
    }

    result
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

            // Skip if inside a cases environment (too complex to handle correctly)
            if is_inside_cases_environment(&chars, i) {
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

/// Check if position is inside a \begin{cases}...\end{cases} environment
/// Returns true if we're between \begin{cases} and \end{cases}
fn is_inside_cases_environment(chars: &[char], pos: usize) -> bool {
    let s: String = chars.iter().collect();

    // Find the last \begin{cases} before pos
    let before = &s[..pos];
    let last_begin = before.rfind("\\begin{cases}");

    if let Some(begin_pos) = last_begin {
        // Find the first \end{cases} after begin_pos
        let after_begin = &s[begin_pos..];
        if let Some(end_offset) = after_begin.find("\\end{cases}") {
            let end_pos = begin_pos + end_offset;
            // We're inside if pos is between begin and end
            return pos > begin_pos && pos < end_pos;
        }
        // No \end{cases} found after begin, we're inside an unclosed cases env
        return true;
    }

    false
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
    while start > 0
        && (chars[start - 1].is_alphanumeric()
            || chars[start - 1] == '_'
            || chars[start - 1] == '\\')
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

    // Collect alphanumeric, backslash, and underscores
    while end < chars.len()
        && (chars[end].is_alphanumeric() || chars[end] == '_' || chars[end] == '\\')
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
            if after < chars.len() && (chars[after] == '{' || chars[after] == '(' || chars[after] == '\\') {
                result.push(chars[i]);
                i += 1;
                continue;
            }
            // Count consecutive alphabetic chars
            let ident_start = after;
            let mut j = after;
            while j < chars.len() && chars[j].is_ascii_alphabetic() {
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
fn convert_lr(input: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let input_chars: Vec<char> = input.chars().collect();

    while i < input_chars.len() {
        // Check for lr( pattern
        if i + 2 < input_chars.len()
            && input_chars[i] == 'l'
            && input_chars[i + 1] == 'r'
            && input_chars[i + 2] == '('
        {
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

        result.push(input_chars[i]);
        i += 1;
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
        let result = convert_text_quotes(r#""if" n "is odd""#);
        assert_eq!(result, "\\text{if} n \\text{is odd}");
    }

    #[test]
    fn convert_with_definitions() {
        let mut defs = HashMap::new();
        defs.insert("Center".to_string(), "\\operatorname{Center}".to_string());

        let result = typst_to_latex("Center(x)", &defs);
        assert!(result.contains("\\operatorname{Center}"));
    }

    #[test]
    fn convert_comparison_operators() {
        let defs = HashMap::new();
        let result = typst_to_latex("1 <= i <= n", &defs);
        // Should produce single backslash: \leq
        assert_eq!(result, "1 \\leq i \\leq n");
    }

    #[test]
    fn convert_attach_with_comparison() {
        let defs = HashMap::new();
        let result = typst_to_latex("attach(Median, b: 1 <= i <= n)", &defs);
        // Should produce \underset{1 \leq i \leq n}{Median}
        assert!(result.contains("\\underset{1 \\leq i \\leq n}{Median}"));
    }

    #[test]
    fn convert_explicit_fraction_in_subscript() {
        let defs = HashMap::new();
        // Typst: x_(((n+1)\/2)) should become x_{(\frac{n+1}{2})}
        let result = typst_to_latex("x_(((n+1)\\/2))", &defs);
        assert_eq!(result, "x_{(\\frac{n+1}{2})}");
    }

    #[test]
    fn convert_complex_expression_with_fractions() {
        let defs = HashMap::new();
        // Typst: (x_((n\/2)) + x_((n\/2+1))) / 2
        let result = typst_to_latex("(x_((n\\/2)) + x_((n\\/2+1))) / 2", &defs);
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
        let result = typst_to_latex(input, &defs);
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
        let result = typst_to_latex(input, &defs);
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
        let result = typst_to_latex(input, &defs);
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
        let result = typst_to_latex(input, &defs);
        assert_eq!(
            result, "\\text{Dominance}",
            "Definitions should not be applied inside \\text{{}}"
        );

        // But unquoted Dominance should get the definition applied
        let input2 = "Dominance(x, y)";
        let result2 = typst_to_latex(input2, &defs);
        assert!(
            result2.contains("\\operatorname{Dominance}"),
            "Definitions should be applied outside \\text{{}}"
        );
    }

    #[test]
    fn convert_blackboard_bold() {
        let defs = HashMap::new();
        let result = typst_to_latex("bb(1)", &defs);
        assert_eq!(result, "\\mathbb{1}");
    }

    #[test]
    fn convert_blackboard_bold_in_sum() {
        let defs = HashMap::new();
        let result = typst_to_latex("sum bb(1)(x > y)", &defs);
        assert!(
            result.contains("\\mathbb{1}"),
            "Should convert bb(1): {result}"
        );
    }

    #[test]
    fn convert_binomial() {
        let defs = HashMap::new();
        let result = typst_to_latex("binom(n, k)", &defs);
        assert_eq!(result, "\\binom{n}{k}");
    }

    #[test]
    fn convert_binomial_complex() {
        let defs = HashMap::new();
        let result = typst_to_latex("binom(n+m, n)", &defs);
        assert_eq!(result, "\\binom{n+m}{n}");
    }

    #[test]
    fn convert_floor() {
        let defs = HashMap::new();
        let result = typst_to_latex("floor(x)", &defs);
        assert_eq!(result, "\\lfloor x \\rfloor");
    }

    #[test]
    fn convert_floor_complex() {
        let defs = HashMap::new();
        let result = typst_to_latex("floor((N+1)/2)", &defs);
        assert!(result.contains("\\lfloor"), "Should have lfloor: {result}");
        assert!(result.contains("\\rfloor"), "Should have rfloor: {result}");
    }

    #[test]
    fn convert_ceil() {
        let defs = HashMap::new();
        let result = typst_to_latex("ceil(x)", &defs);
        assert_eq!(result, "\\lceil x \\rceil");
    }

    #[test]
    fn convert_abs() {
        let defs = HashMap::new();
        let result = typst_to_latex("abs(x)", &defs);
        assert_eq!(result, "\\lvert x \\rvert");
    }

    #[test]
    fn convert_abs_complex() {
        let defs = HashMap::new();
        let result = typst_to_latex("abs(x_i - x_j)", &defs);
        assert!(result.contains("\\lvert"), "Should have lvert: {result}");
        assert!(result.contains("\\rvert"), "Should have rvert: {result}");
    }

    #[test]
    fn convert_abs_in_fraction_denominator() {
        // Test that abs() in fraction denominator stays intact
        // This was a bug where \lvert...\rvert got split by fraction conversion
        let defs = HashMap::new();
        let result = typst_to_latex("a / abs(b)", &defs);
        eprintln!("Result: {result}");
        assert!(
            result.contains("\\lvert") && result.contains("\\rvert"),
            "abs should be intact in denominator: {result}"
        );
    }

    #[test]
    fn convert_pr_function() {
        let defs = HashMap::new();
        let result = typst_to_latex("Pr(X > 0)", &defs);
        assert!(result.contains("\\Pr("), "Should have \\Pr(: {result}");
    }

    #[test]
    fn convert_phi_function() {
        let defs = HashMap::new();
        let result = typst_to_latex("Phi(z)", &defs);
        assert!(result.contains("\\Phi("), "Should have \\Phi(: {result}");
    }

    #[test]
    fn convert_phi_standalone() {
        let defs = HashMap::new();
        // Standalone Phi without parentheses should also be converted
        let result = typst_to_latex("where Phi denotes", &defs);
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
        let result = typst_to_latex("Phi(z) and Phi", &defs);
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
        let result = typst_to_latex("\"Var\"[X] / \"Var\"[Y]", &defs);
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
        let result = typst_to_latex("upright(\"mean\")", &defs);
        assert_eq!(result, "\\mathrm{mean}");
    }

    #[test]
    fn convert_upright_no_quotes() {
        let defs = HashMap::new();
        let result = typst_to_latex("upright(stdDev)", &defs);
        assert_eq!(result, "\\mathrm{stdDev}");
    }

    #[test]
    fn convert_subscript_with_text() {
        let defs = HashMap::new();
        // k_"left" -> first converts to k_\text{left}, then should wrap in braces
        let result = typst_to_latex("k_\"left\"", &defs);
        assert_eq!(result, "k_{\\text{left}}");
    }

    #[test]
    fn convert_fraction_with_binom() {
        let defs = HashMap::new();
        // Test the problematic case: 1\/binom(12, 6) should become \frac{1}{\binom{12}{6}}
        let result = typst_to_latex("1\\/binom(12, 6)", &defs);
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
        let result = typst_to_latex("Drift_\"baseline\"(T, X)", &defs);
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
        let result = typst_to_latex("Drift^2", &defs);
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

        let result = typst_to_latex("pmean", &defs);
        assert_eq!(result, "\\mathrm{mean}");
    }

    #[test]
    fn convert_pstddev_definition() {
        let mut defs = HashMap::new();
        defs.insert("pstddev".to_string(), "\\mathrm{stdDev}".to_string());

        let result = typst_to_latex("pstddev", &defs);
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
        let result = typst_to_latex("Additive(pmean, pstddev)", &defs);
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
        let result = typst_to_latex("pstddev^2", &defs);
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

        let result = typst_to_latex("cmad", &defs);
        assert_eq!(result, "c_{\\mathrm{mad}}");
    }

    #[test]
    fn convert_cspr_definition() {
        let mut defs = HashMap::new();
        defs.insert("cspr".to_string(), "c_{\\mathrm{spr}}".to_string());

        let result = typst_to_latex("cspr", &defs);
        assert_eq!(result, "c_{\\mathrm{spr}}");
    }

    #[test]
    fn convert_approxdist_definition() {
        let mut defs = HashMap::new();
        // Use \text{approx} to avoid word_mappings converting approx to \approx
        defs.insert("approxdist".to_string(), "\\sim\\text{approx}".to_string());

        let result = typst_to_latex("X approxdist Y", &defs);
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
        assert_eq!(typst_to_latex("pmean", &defs), "\\mathrm{mean}");
        assert_eq!(typst_to_latex("pstddev", &defs), "\\mathrm{stdDev}");
        assert_eq!(typst_to_latex("plogmean", &defs), "\\mathrm{logMean}");
        assert_eq!(typst_to_latex("plogstddev", &defs), "\\mathrm{logStdDev}");
        assert_eq!(typst_to_latex("pmin", &defs), "\\mathrm{min}");
        assert_eq!(typst_to_latex("pmax", &defs), "\\mathrm{max}");
        assert_eq!(typst_to_latex("pshape", &defs), "\\mathrm{shape}");
        assert_eq!(typst_to_latex("prate", &defs), "\\mathrm{rate}");
    }

    #[test]
    fn convert_expression_with_pstddev_division() {
        let mut defs = HashMap::new();
        defs.insert("pstddev".to_string(), "\\mathrm{stdDev}".to_string());

        // Test pstddev/sqrt(n) pattern - note that fraction conversion doesn't work
        // when the numerator is a LaTeX command result (the converter sees \mathrm{...}
        // and doesn't recognize it as a valid numerator for fractions)
        let result = typst_to_latex("pstddev/sqrt(n)", &defs);
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
        let result = typst_to_latex("sqrt(2) dot pstddev", &defs);
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
        let result = typst_to_latex("Additive(0, sqrt(2) dot pstddev)", &defs);
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
        let result = typst_to_latex("\"pmean\"", &defs);
        assert_eq!(
            result, "\\text{pmean}",
            "pmean inside quotes should not be converted: {result}"
        );

        // But pmean outside quotes should be converted
        let result2 = typst_to_latex("pmean", &defs);
        assert_eq!(result2, "\\mathrm{mean}");
    }

    #[test]
    fn convert_assignment_arrow() {
        let defs = HashMap::new();
        let result = typst_to_latex("x <- x + 1", &defs);
        assert_eq!(result, "x \\leftarrow x + 1");
    }

    #[test]
    fn convert_xor_operator() {
        let defs = HashMap::new();
        let result = typst_to_latex("x xor y", &defs);
        assert_eq!(result, "x \\operatorname{xor} y");
    }

    #[test]
    fn convert_log_operator() {
        let defs = HashMap::new();
        // Standalone log should become \log
        let result = typst_to_latex("O(n log n)", &defs);
        assert_eq!(result, "O(n \\log n)");

        // log with parentheses should also work
        let result2 = typst_to_latex("log(x)", &defs);
        assert_eq!(result2, "\\log(x)");
    }

    #[test]
    fn convert_math_operators() {
        let defs = HashMap::new();
        // Test various math operators
        assert_eq!(typst_to_latex("sin x", &defs), "\\sin x");
        assert_eq!(typst_to_latex("cos x", &defs), "\\cos x");
        assert_eq!(typst_to_latex("max(a, b)", &defs), "\\max(a, b)");
        assert_eq!(typst_to_latex("min(a, b)", &defs), "\\min(a, b)");
        assert_eq!(typst_to_latex("ln x", &defs), "\\ln x");
        assert_eq!(typst_to_latex("exp x", &defs), "\\exp x");
    }

    #[test]
    fn convert_quad_spacing() {
        let defs = HashMap::new();
        let result = typst_to_latex("a quad b", &defs);
        assert_eq!(result, "a \\quad b");
    }

    #[test]
    fn convert_right_shift() {
        let defs = HashMap::new();
        let result = typst_to_latex("x >> 30", &defs);
        assert_eq!(result, "x \\gg 30");
    }

    #[test]
    fn convert_left_shift() {
        let defs = HashMap::new();
        let result = typst_to_latex("x << 3", &defs);
        assert_eq!(result, "x \\ll 3");
    }

    #[test]
    fn convert_splitmix64_formula() {
        let defs = HashMap::new();
        // Test the actual formula from the randomization chapter
        let result = typst_to_latex("x <- (x xor (x >> 30)) times \"0xbf58476d1ce4e5b9\"", &defs);
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
        let result = typst_to_latex("p_(n,m)(c) / binom(n+m, n)", &defs);
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
        let result = typst_to_latex("x_min \\/ (1 - U)^(2)", &defs);
        eprintln!("Result: {result}");
        // The entire (1 - U)^{2} should be in the denominator
        assert!(
            result.contains("\\frac{x_{min}}{(1 - U)^{2}}"),
            "Superscript should be part of denominator: {result}"
        );
    }

    #[test]
    fn convert_fraction_with_nested_fraction_exponent() {
        // Test x_min \/ (1 - U)^(1\/alpha) - the exponent has a fraction inside
        let defs = HashMap::new();
        let result = typst_to_latex("x_min \\/ (1 - U)^(1\\/alpha)", &defs);
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
        let result = typst_to_latex("(n! dot m!) / (n+m)!", &defs);
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
        let result = typst_to_latex("(n! dot m!) \\/ (n+m)!", &defs);
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

        let result = typst_to_latex("Drift^2(T_2, X) / Drift^2(T_1, X)", &defs);
        eprintln!("Result: {result}");

        // Should create a proper fraction with superscripts intact
        assert!(
            result.contains("\\frac{\\operatorname{Drift}^2(T_2, X)}{\\operatorname{Drift}^2(T_1, X)}"),
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

        let result =
            typst_to_latex("n_\"new\" = n_\"original\" dot Drift^2(T_2, X) / Drift^2(T_1, X)", &defs);
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
            result.contains("\\frac{\\operatorname{Drift}^2(T_2, X)}{\\operatorname{Drift}^2(T_1, X)}"),
            "Should have proper fraction with Drift^2: {result}"
        );
    }

    #[test]
    fn convert_greek_with_subscript_parens() {
        // Greek letters followed by subscript in parentheses: sigma_(n,m)
        let defs = HashMap::new();
        let result = typst_to_latex("sigma_(n,m)(d)", &defs);
        assert_eq!(
            result, "\\sigma_{n,m}(d)",
            "sigma with subscript parens should convert: {result}"
        );
    }

    #[test]
    fn convert_greek_with_simple_subscript() {
        // Greek letters followed by simple subscript: epsilon_k
        let defs = HashMap::new();
        let result = typst_to_latex("epsilon_k", &defs);
        assert_eq!(
            result, "\\epsilon_k",
            "epsilon with subscript should convert: {result}"
        );
    }

    #[test]
    fn convert_greek_with_superscript() {
        // Greek letters followed by superscript: sigma^2
        let defs = HashMap::new();
        let result = typst_to_latex("sigma^2", &defs);
        assert_eq!(
            result, "\\sigma^2",
            "sigma with superscript should convert: {result}"
        );
    }

    #[test]
    fn convert_pairwise_margin_formula() {
        // Test the actual formula from fast-pairwise-margin.typ
        let defs = HashMap::new();
        let result = typst_to_latex("sigma_(n,m)(d) = sum_(k|d) epsilon_k dot k", &defs);
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
        let result = typst_to_latex("thesigma", &defs);
        assert_eq!(result, "thesigma", "Embedded sigma should not convert: {result}");

        // "sigmaX" should stay as-is (sigma followed by letter)
        let result = typst_to_latex("sigmaX", &defs);
        assert_eq!(result, "sigmaX", "sigma followed by letter should not convert: {result}");

        // But "sigma X" should convert (space separator)
        let result = typst_to_latex("sigma X", &defs);
        assert_eq!(result, "\\sigma X", "sigma with space should convert: {result}");
    }

    #[test]
    fn greek_standalone_converts() {
        // Standalone Greek letters should convert
        let defs = HashMap::new();
        assert_eq!(typst_to_latex("sigma", &defs), "\\sigma");
        assert_eq!(typst_to_latex("epsilon", &defs), "\\epsilon");
        assert_eq!(typst_to_latex("alpha", &defs), "\\alpha");
        assert_eq!(typst_to_latex("beta", &defs), "\\beta");
    }

    #[test]
    fn greek_with_operators_converts() {
        // Greek letters adjacent to operators should convert
        let defs = HashMap::new();
        assert_eq!(typst_to_latex("sigma + tau", &defs), "\\sigma + \\tau");
        assert_eq!(typst_to_latex("(sigma)", &defs), "(\\sigma)");
        assert_eq!(typst_to_latex("sigma,tau", &defs), "\\sigma,\\tau");
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
        let result = typst_to_latex(input, &defs);
        eprintln!("chained explicit fractions: {result}");
        assert!(
            !result.contains("\\sqrt{2\\frac"),
            "sqrt brace must close before frac: {result}"
        );
    }
}
