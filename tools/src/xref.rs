use std::collections::HashMap;

/// Cross-reference mapping for internal links
///
/// Maps Typst label names (e.g., "sec-alg-center") to web URLs (e.g., "/center#algorithm")
pub struct XRefMap {
    mappings: HashMap<String, String>,
}

impl XRefMap {
    /// Create a new cross-reference map with predefined mappings
    #[must_use]
    pub fn new() -> Self {
        let mut mappings = HashMap::new();

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
        mappings.insert(
            "sec-weak-symmetry".into(),
            "/assumptions#weak-symmetry-assumption".into(),
        );

        // Test framework -> methodology
        mappings.insert(
            "sec-test-framework".into(),
            "/methodology#test-framework".into(),
        );

        // Function page labels
        mappings.insert("sec-center".into(), "/center".into());
        mappings.insert("sec-center-bounds".into(), "/center-bounds".into());
        mappings.insert("sec-spread".into(), "/spread".into());
        mappings.insert("sec-spread-bounds".into(), "/spread-bounds".into());

        mappings.insert("sec-shift".into(), "/shift".into());
        mappings.insert("sec-shift-bounds".into(), "/shift-bounds".into());
        mappings.insert("sec-ratio".into(), "/ratio".into());
        mappings.insert("sec-ratio-bounds".into(), "/ratio-bounds".into());
        mappings.insert("sec-disparity".into(), "/disparity".into());
        mappings.insert("sec-disparity-bounds".into(), "/disparity-bounds".into());
        mappings.insert("sec-avg-spread".into(), "/avg-spread".into());
        mappings.insert("sec-avg-spread-bounds".into(), "/avg-spread-bounds".into());
        mappings.insert("sec-median".into(), "/median".into());
        mappings.insert("sec-sign-margin".into(), "/sign-margin".into());
        mappings.insert("sec-pairwise-margin".into(), "/pairwise-margin".into());
        mappings.insert(
            "sec-signed-rank-margin".into(),
            "/signed-rank-margin".into(),
        );
        mappings.insert("sec-rng".into(), "/rng".into());
        mappings.insert("sec-uniform-int".into(), "/uniform-int".into());
        mappings.insert("sec-uniform-float".into(), "/uniform-float".into());
        mappings.insert("sec-sample".into(), "/sample".into());
        mappings.insert("sec-shuffle".into(), "/shuffle".into());
        mappings.insert("sec-resample".into(), "/resample".into());

        // Algorithm sections -> per-function pages
        mappings.insert("sec-alg-center".into(), "/center#algorithm".into());
        mappings.insert(
            "sec-alg-center-bounds".into(),
            "/center-bounds#algorithm".into(),
        );
        mappings.insert("sec-alg-spread".into(), "/spread#algorithm".into());
        mappings.insert(
            "sec-alg-spread-bounds".into(),
            "/spread-bounds#algorithm".into(),
        );

        mappings.insert("sec-alg-shift".into(), "/shift#algorithm".into());
        mappings.insert(
            "sec-alg-shift-bounds".into(),
            "/shift-bounds#algorithm".into(),
        );
        mappings.insert("sec-alg-ratio".into(), "/ratio#algorithm".into());
        mappings.insert(
            "sec-alg-ratio-bounds".into(),
            "/ratio-bounds#algorithm".into(),
        );
        mappings.insert(
            "sec-alg-disparity".into(),
            "/disparity#algorithm".into(),
        );
        mappings.insert(
            "sec-alg-disparity-bounds".into(),
            "/disparity-bounds#algorithm".into(),
        );
        mappings.insert("sec-alg-rng".into(), "/rng#algorithm".into());
        mappings.insert(
            "sec-alg-uniform".into(),
            "/uniform-float#algorithm".into(),
        );
        mappings.insert("sec-alg-sample".into(), "/sample#algorithm".into());
        mappings.insert("sec-alg-shuffle".into(), "/shuffle#algorithm".into());
        mappings.insert(
            "sec-alg-resample".into(),
            "/resample#algorithm".into(),
        );
        mappings.insert(
            "sec-alg-avg-spread".into(),
            "/avg-spread#algorithm".into(),
        );
        mappings.insert(
            "sec-alg-avg-spread-bounds".into(),
            "/avg-spread-bounds#algorithm".into(),
        );
        mappings.insert("sec-alg-median".into(), "/median#algorithm".into());
        mappings.insert(
            "sec-alg-sign-margin".into(),
            "/sign-margin#algorithm".into(),
        );
        mappings.insert(
            "sec-alg-pairwise-margin".into(),
            "/pairwise-margin#algorithm".into(),
        );
        mappings.insert(
            "sec-alg-signed-rank-margin".into(),
            "/signed-rank-margin#algorithm".into(),
        );

        // Notes sections -> per-function pages
        mappings.insert(
            "sec-reframing-uniform".into(),
            "/uniform-float#notes".into(),
        );
        mappings.insert(
            "sec-efficiency-to-drift".into(),
            "/foundations#from-statistical-efficiency-to-drift".into(),
        );
        mappings.insert(
            "sec-median-bounds-efficiency".into(),
            "/center-bounds#on-misrate-efficiency-of-medianbounds".into(),
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
    fn resolve_assumptions_label() {
        let xref = XRefMap::new();
        assert_eq!(xref.resolve("sec-assumptions"), Some("/assumptions"));
    }

    #[test]
    fn resolve_algorithm_label() {
        let xref = XRefMap::new();
        assert_eq!(xref.resolve("sec-alg-center"), Some("/center#algorithm"));
        assert_eq!(
            xref.resolve("sec-alg-rng"),
            Some("/rng#algorithm")
        );
    }

    #[test]
    fn resolve_test_framework() {
        let xref = XRefMap::new();
        assert_eq!(
            xref.resolve("sec-test-framework"),
            Some("/methodology#test-framework")
        );
    }

    #[test]
    fn resolve_unknown_label() {
        let xref = XRefMap::new();
        assert_eq!(xref.resolve("unknown-label"), None);
    }
}
