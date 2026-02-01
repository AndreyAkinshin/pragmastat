use std::collections::HashMap;

/// Cross-reference mapping for internal links
///
/// Maps Typst label names (e.g., "ch-algorithms") to web URLs (e.g., "/manual/algorithms")
pub struct XRefMap {
    mappings: HashMap<String, String>,
}

impl XRefMap {
    /// Create a new cross-reference map with predefined mappings
    #[must_use]
    pub fn new() -> Self {
        let mut mappings = HashMap::new();

        // Chapter labels -> chapter URLs
        mappings.insert("ch-algorithms".into(), "/algorithms".into());
        mappings.insert("ch-assumptions".into(), "/assumptions".into());
        mappings.insert("ch-tests".into(), "/tests".into());

        // Assumptions chapter sections
        mappings.insert("sec-assumptions".into(), "/assumptions".into());
        mappings.insert(
            "sec-positivity".into(),
            "/assumptions#positivity-assumption".into(),
        );
        mappings.insert(
            "sec-sparity".into(),
            "/assumptions#sparity-assumption".into(),
        );

        // Section labels -> chapter URLs with anchors
        mappings.insert(
            "sec-prng".into(),
            "/algorithms#pseudorandom-number-generation".into(),
        );
        mappings.insert(
            "sec-test-framework".into(),
            "/tests#test-framework".into(),
        );
        mappings.insert(
            "sec-fast-center".into(),
            "/algorithms#fast-center".into(),
        );
        mappings.insert(
            "sec-fast-spread".into(),
            "/algorithms#fast-spread".into(),
        );
        mappings.insert(
            "sec-fast-shift".into(),
            "/algorithms#fast-shift".into(),
        );
        mappings.insert(
            "sec-fast-pairwise-margin".into(),
            "/algorithms#fast-pairwisemargin".into(),
        );

        Self { mappings }
    }

    /// Resolve an internal label to its web URL
    ///
    /// Returns `None` if the label is not found
    #[must_use]
    pub fn resolve(&self, label: &str) -> Option<&str> {
        self.mappings.get(label).map(String::as_str)
    }
}

impl Default for XRefMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_chapter_label() {
        let xref = XRefMap::new();
        assert_eq!(xref.resolve("ch-algorithms"), Some("/algorithms"));
        assert_eq!(xref.resolve("ch-tests"), Some("/tests"));
    }

    #[test]
    fn resolve_section_label() {
        let xref = XRefMap::new();
        assert_eq!(
            xref.resolve("sec-prng"),
            Some("/algorithms#pseudorandom-number-generation")
        );
        assert_eq!(
            xref.resolve("sec-test-framework"),
            Some("/tests#test-framework")
        );
    }

    #[test]
    fn resolve_unknown_label() {
        let xref = XRefMap::new();
        assert_eq!(xref.resolve("unknown-label"), None);
    }
}
