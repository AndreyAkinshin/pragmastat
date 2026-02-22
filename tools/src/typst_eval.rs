//! Typst evaluation context for variable resolution
//!
//! This module provides a minimal Typst interpreter to evaluate:
//! - Variable bindings (#let x = ...)
//! - Variable references (#var, #var.field)
//! - String concatenation ("a" + "b" + var)
//! - Dictionary literals and access

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

/// A Typst value that can be stored in the evaluation context
#[derive(Debug, Clone)]
pub enum TypstValue {
    String(String),
    None,
    Dictionary(HashMap<String, TypstValue>),
}

impl TypstValue {
    /// Get string value, returning empty string for None
    pub fn as_string(&self) -> &str {
        match self {
            TypstValue::String(s) => s,
            TypstValue::None | TypstValue::Dictionary(_) => "",
        }
    }

    /// Get field from dictionary, returning None if not a dictionary or field doesn't exist
    pub fn get_field(&self, field: &str) -> Option<&TypstValue> {
        match self {
            TypstValue::Dictionary(d) => d.get(field),
            _ => None,
        }
    }
}

/// Evaluation context holding variable bindings
#[derive(Debug, Clone, Default)]
pub struct EvalContext {
    /// Variable bindings
    pub vars: HashMap<String, TypstValue>,
}

impl EvalContext {
    pub fn new(_base_path: &Path) -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    /// Look up a variable by name
    pub fn get(&self, name: &str) -> Option<&TypstValue> {
        self.vars.get(name)
    }

    /// Set a variable
    pub fn set(&mut self, name: &str, value: TypstValue) {
        self.vars.insert(name.to_string(), value);
    }

    /// Resolve a variable reference path like "lang.title"
    pub fn resolve(&self, path: &str) -> Option<&TypstValue> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return None;
        }

        let mut current = self.get(parts[0])?;
        for part in &parts[1..] {
            current = current.get_field(part)?;
        }
        Some(current)
    }

    /// Resolve a path to a string value
    pub fn resolve_string(&self, path: &str) -> String {
        self.resolve(path)
            .map(|v| v.as_string().to_string())
            .unwrap_or_default()
    }
}

/// Parse the definitions.typ file and extract variables
pub fn parse_definitions(path: &Path) -> Result<EvalContext> {
    let content = std::fs::read_to_string(path)?;
    let base_path = path.parent().unwrap_or(Path::new("."));
    let mut ctx = EvalContext::new(base_path);
    let chars: Vec<char> = content.chars().collect();

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

    let mut i = 0;

    while i < chars.len() {
        // Skip whitespace and comments
        while i < chars.len()
            && (chars[i].is_whitespace()
                || (i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '/'))
        {
            if chars[i] == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                // Skip to end of line
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
            }
            if i < chars.len() {
                i += 1;
            }
        }

        // Check for #import "path": var
        if starts_with_at(i, "#import") {
            i += 7;
            // Skip whitespace
            while i < chars.len() && chars[i].is_whitespace() {
                i += 1;
            }

            // Parse import path (string literal)
            if i < chars.len() && chars[i] == '"' {
                let (import_path, new_i) = parse_string_literal_chars(&chars, i)?;
                i = new_i;

                // Skip whitespace and colon
                while i < chars.len() && (chars[i].is_whitespace() || chars[i] == ':') {
                    i += 1;
                }

                // Parse imported variable name(s): single name, star, or comma-separated list
                let mut import_vars = Vec::new();
                let var_start = i;
                if i < chars.len() && chars[i] == '*' {
                    import_vars.push("*".to_string());
                    i += 1;
                } else {
                    while i < chars.len()
                        && (chars[i].is_alphanumeric() || chars[i] == '_')
                    {
                        i += 1;
                    }
                    import_vars.push(chars_to_string(var_start, i));

                    // Parse additional comma-separated names
                    loop {
                        // Skip whitespace
                        while i < chars.len() && chars[i] == ' ' {
                            i += 1;
                        }
                        if i >= chars.len() || chars[i] != ',' {
                            break;
                        }
                        i += 1; // Skip comma
                        // Skip whitespace
                        while i < chars.len() && chars[i] == ' ' {
                            i += 1;
                        }
                        let next_start = i;
                        while i < chars.len()
                            && (chars[i].is_alphanumeric() || chars[i] == '_')
                        {
                            i += 1;
                        }
                        if i > next_start {
                            import_vars.push(chars_to_string(next_start, i));
                        }
                    }
                }

                // Resolve import path relative to definitions file
                let import_file_path = base_path.join(&import_path);
                if import_file_path.exists() {
                    // Recursively parse the imported file
                    let imported_ctx = parse_definitions(&import_file_path)?;

                    // Import the specified variable(s)
                    for import_var in &import_vars {
                        if import_var == "*" {
                            for (name, value) in &imported_ctx.vars {
                                ctx.set(name, value.clone());
                            }
                        } else if let Some(value) = imported_ctx.get(import_var) {
                            ctx.set(import_var, value.clone());
                        }
                    }
                }
            }
            // Skip to end of line
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }

        // Check for #let
        if starts_with_at(i, "#let") {
            i += 4;
            // Skip whitespace
            while i < chars.len() && chars[i].is_whitespace() {
                i += 1;
            }

            // Read variable name
            let name_start = i;
            while i < chars.len()
                && (chars[i].is_alphanumeric() || chars[i] == '-' || chars[i] == '_')
            {
                i += 1;
            }
            let name = chars_to_string(name_start, i).replace('-', "_"); // Normalize hyphen to underscore

            // Skip whitespace and =
            while i < chars.len() && (chars[i].is_whitespace() || chars[i] == '=') {
                i += 1;
            }

            // Parse value
            let (value, new_i) = parse_value_chars(&chars, i, &ctx)?;
            i = new_i;
            ctx.set(&name, value);
        } else if i < chars.len() {
            i += 1;
        }
    }

    Ok(ctx)
}

/// Parse a Typst value starting at position i using character arrays
fn parse_value_chars(
    chars: &[char],
    mut i: usize,
    ctx: &EvalContext,
) -> Result<(TypstValue, usize)> {
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
    while i < chars.len() && chars[i].is_whitespace() && chars[i] != '\n' {
        i += 1;
    }

    if i >= chars.len() {
        return Ok((TypstValue::None, i));
    }

    // String literal
    if chars[i] == '"' {
        let (s, new_i) = parse_string_expr_chars(chars, i, ctx)?;
        return Ok((TypstValue::String(s), new_i));
    }

    // Dictionary literal
    if chars[i] == '(' {
        let (dict, new_i) = parse_dictionary_chars(chars, i, ctx)?;
        return Ok((TypstValue::Dictionary(dict), new_i));
    }

    // 'none' literal
    if starts_with_at(i, "none") {
        return Ok((TypstValue::None, i + 4));
    }

    // Variable reference or identifier
    if chars[i].is_alphabetic() || chars[i] == '_' {
        let name_start = i;
        while i < chars.len()
            && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-' || chars[i] == '.')
        {
            i += 1;
        }
        let name = chars_to_string(name_start, i).replace('-', "_");

        // Check for string concatenation
        let mut result = ctx.resolve_string(&name);

        // Look for + concatenation
        loop {
            // Skip whitespace
            while i < chars.len() && chars[i].is_whitespace() && chars[i] != '\n' {
                i += 1;
            }

            if i < chars.len() && chars[i] == '+' {
                i += 1;
                // Skip whitespace
                while i < chars.len() && chars[i].is_whitespace() && chars[i] != '\n' {
                    i += 1;
                }

                // Parse next part
                if i < chars.len() && chars[i] == '"' {
                    let (s, new_i) = parse_string_literal_chars(chars, i)?;
                    result.push_str(&s);
                    i = new_i;
                } else if i < chars.len() && (chars[i].is_alphabetic() || chars[i] == '_') {
                    let var_start = i;
                    while i < chars.len()
                        && (chars[i].is_alphanumeric()
                            || chars[i] == '_'
                            || chars[i] == '-'
                            || chars[i] == '.')
                    {
                        i += 1;
                    }
                    let var_name = chars_to_string(var_start, i).replace('-', "_");
                    result.push_str(&ctx.resolve_string(&var_name));
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        return Ok((TypstValue::String(result), i));
    }

    // Skip to end of line for unrecognized content
    while i < chars.len() && chars[i] != '\n' {
        i += 1;
    }

    Ok((TypstValue::None, i))
}

/// Parse a string literal starting at position i (at the opening ")
#[allow(clippy::unnecessary_wraps)]
fn parse_string_literal_chars(chars: &[char], mut i: usize) -> Result<(String, usize)> {
    if i >= chars.len() || chars[i] != '"' {
        return Ok((String::new(), i));
    }
    i += 1; // Skip opening "

    let mut result = String::new();
    while i < chars.len() && chars[i] != '"' {
        if chars[i] == '\\' && i + 1 < chars.len() {
            // Escape sequence
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
        i += 1; // Skip closing "
    }

    Ok((result, i))
}

/// Parse a string expression (string literal potentially with concatenation)
fn parse_string_expr_chars(
    chars: &[char],
    mut i: usize,
    ctx: &EvalContext,
) -> Result<(String, usize)> {
    // Helper to get substring from char slice
    let chars_to_string =
        |start: usize, end: usize| -> String { chars[start..end].iter().collect() };

    let (mut result, new_i) = parse_string_literal_chars(chars, i)?;
    i = new_i;

    // Look for + concatenation
    loop {
        // Skip whitespace
        while i < chars.len() && chars[i].is_whitespace() && chars[i] != '\n' {
            i += 1;
        }

        if i < chars.len() && chars[i] == '+' {
            i += 1;
            // Skip whitespace
            while i < chars.len() && chars[i].is_whitespace() && chars[i] != '\n' {
                i += 1;
            }

            if i < chars.len() && chars[i] == '"' {
                let (s, new_i) = parse_string_literal_chars(chars, i)?;
                result.push_str(&s);
                i = new_i;
            } else if i < chars.len() && (chars[i].is_alphabetic() || chars[i] == '_') {
                // Variable reference
                let var_start = i;
                while i < chars.len()
                    && (chars[i].is_alphanumeric()
                        || chars[i] == '_'
                        || chars[i] == '-'
                        || chars[i] == '.')
                {
                    i += 1;
                }
                let var_name = chars_to_string(var_start, i).replace('-', "_");
                result.push_str(&ctx.resolve_string(&var_name));
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok((result, i))
}

/// Parse a dictionary literal starting at position i (at the opening paren)
fn parse_dictionary_chars(
    chars: &[char],
    mut i: usize,
    ctx: &EvalContext,
) -> Result<(HashMap<String, TypstValue>, usize)> {
    let mut dict = HashMap::new();

    // Helper to get substring from char slice
    let chars_to_string =
        |start: usize, end: usize| -> String { chars[start..end].iter().collect() };

    if i >= chars.len() || chars[i] != '(' {
        return Ok((dict, i));
    }
    i += 1; // Skip opening (

    loop {
        // Skip whitespace and newlines
        while i < chars.len() && (chars[i].is_whitespace() || chars[i] == ',') {
            i += 1;
        }

        // Check for closing paren or nested dict end
        if i >= chars.len() || chars[i] == ')' {
            if i < chars.len() {
                i += 1;
            }
            break;
        }

        // Parse key
        let key_start = i;
        while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-')
        {
            i += 1;
        }
        let key = chars_to_string(key_start, i);

        // Skip : and whitespace
        while i < chars.len() && (chars[i] == ':' || chars[i].is_whitespace()) {
            i += 1;
        }

        // Parse value
        let (value, new_i) = parse_value_chars(chars, i, ctx)?;
        i = new_i;

        dict.insert(key, value);
    }

    Ok((dict, i))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_real_definitions_file() {
        // Test parsing the actual definitions.typ file
        let def_path = std::path::Path::new("../manual/definitions.typ");
        if def_path.exists() {
            let ctx = parse_definitions(def_path).expect("Should parse definitions.typ");

            // Check that 'version' is imported from version.typ via #import
            let version = ctx.resolve_string("version");
            assert!(
                !version.is_empty(),
                "version should be imported from version.typ, got empty string"
            );
            assert!(
                version.chars().all(|c| c.is_ascii_digit() || c == '.'),
                "version should be a semver string like '4.0.1', got: '{version}'"
            );

            // Check languages dictionary
            let py_title = ctx.resolve_string("languages.py.title");
            assert_eq!(py_title, "Python", "languages.py.title should be 'Python'");

            let py_demo = ctx.resolve_string("languages.py.demo");
            assert!(
                !py_demo.is_empty(),
                "languages.py.demo should not be empty, got: '{py_demo}'"
            );
            assert!(
                py_demo.contains("py"),
                "languages.py.demo should contain 'py', got: '{py_demo}'"
            );

            // Check github-repo is parsed (defined directly, not imported)
            let github_repo = ctx.resolve_string("github_repo");
            assert!(
                github_repo.contains("github.com"),
                "github_repo should contain github.com, got: '{github_repo}'"
            );

            // Check github-tree includes version (demonstrates #import + concatenation)
            let github_tree = ctx.resolve_string("github_tree");
            assert!(
                github_tree.contains(&version),
                "github_tree should contain version '{version}', got: '{github_tree}'"
            );
        }
    }

    #[test]
    fn parse_import_statement() {
        // Create a temporary directory with test files
        let temp_dir = std::env::temp_dir().join("typst_eval_test");
        let _ = std::fs::create_dir_all(&temp_dir);

        // Create version.typ
        std::fs::write(temp_dir.join("version.typ"), "#let version = \"1.2.3\"\n")
            .expect("Failed to write version.typ");

        // Create definitions.typ that imports version.typ
        std::fs::write(
            temp_dir.join("definitions.typ"),
            "#import \"version.typ\": version\n#let name = \"test\" + version\n",
        )
        .expect("Failed to write definitions.typ");

        let ctx =
            parse_definitions(&temp_dir.join("definitions.typ")).expect("Should parse definitions");

        // Check that version was imported
        assert_eq!(
            ctx.resolve_string("version"),
            "1.2.3",
            "version should be imported from version.typ"
        );

        // Check that concatenation with imported variable works
        assert_eq!(
            ctx.resolve_string("name"),
            "test1.2.3",
            "name should concatenate 'test' + version"
        );

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn parse_import_star() {
        // Create a temporary directory with test files
        let temp_dir = std::env::temp_dir().join("typst_eval_test_star");
        let _ = std::fs::create_dir_all(&temp_dir);

        // Create config.typ with multiple values
        std::fs::write(
            temp_dir.join("config.typ"),
            "#let foo = \"bar\"\n#let num = \"42\"\n",
        )
        .expect("Failed to write config.typ");

        // Create definitions.typ that imports all from config.typ
        std::fs::write(
            temp_dir.join("definitions.typ"),
            "#import \"config.typ\": *\n",
        )
        .expect("Failed to write definitions.typ");

        let ctx =
            parse_definitions(&temp_dir.join("definitions.typ")).expect("Should parse definitions");

        // Check that all values were imported
        assert_eq!(ctx.resolve_string("foo"), "bar", "foo should be imported");
        assert_eq!(ctx.resolve_string("num"), "42", "num should be imported");

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn parse_string_literal_simple() {
        let chars: Vec<char> = r#""hello""#.chars().collect();
        let (s, _) = parse_string_literal_chars(&chars, 0).unwrap();
        assert_eq!(s, "hello");
    }

    #[test]
    fn parse_string_with_escape() {
        let chars: Vec<char> = r#""hello \"world\"""#.chars().collect();
        let (s, _) = parse_string_literal_chars(&chars, 0).unwrap();
        assert_eq!(s, "hello \"world\"");
    }

    #[test]
    fn eval_variable_reference() {
        let mut ctx = EvalContext::new(Path::new("."));
        ctx.set("version", TypstValue::String("1.0.0".to_string()));

        assert_eq!(ctx.resolve_string("version"), "1.0.0");
    }

    #[test]
    fn eval_nested_field() {
        let mut ctx = EvalContext::new(Path::new("."));
        let mut lang = HashMap::new();
        lang.insert(
            "title".to_string(),
            TypstValue::String("Python".to_string()),
        );
        lang.insert("code".to_string(), TypstValue::String("python".to_string()));
        ctx.set("lang", TypstValue::Dictionary(lang));

        assert_eq!(ctx.resolve_string("lang.title"), "Python");
        assert_eq!(ctx.resolve_string("lang.code"), "python");
    }

    #[test]
    fn eval_dictionary_access() {
        let mut ctx = EvalContext::new(Path::new("."));
        let mut langs = HashMap::new();
        let mut py = HashMap::new();
        py.insert(
            "title".to_string(),
            TypstValue::String("Python".to_string()),
        );
        langs.insert("py".to_string(), TypstValue::Dictionary(py));
        ctx.set("languages", TypstValue::Dictionary(langs));

        assert_eq!(ctx.resolve_string("languages.py.title"), "Python");
    }
}
