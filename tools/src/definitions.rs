use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

/// A collection of math macro definitions
pub type Definitions = HashMap<String, String>;

/// Load definitions from a YAML file
pub fn load_definitions(path: &Path) -> Result<Definitions> {
    let content = std::fs::read_to_string(path)?;
    let definitions: Definitions = serde_yaml::from_str(&content)?;
    Ok(definitions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_definitions() {
        let yaml = r"
Center: \operatorname{Center}
x: \mathbf{x}
";
        let defs: Definitions = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            defs.get("Center"),
            Some(&r"\operatorname{Center}".to_string())
        );
        assert_eq!(defs.get("x"), Some(&r"\mathbf{x}".to_string()));
    }

    #[test]
    fn parse_distribution_parameters() {
        let yaml = r"
pmean: \mathrm{mean}
pstddev: \mathrm{stdDev}
plogmean: \mathrm{logMean}
plogstddev: \mathrm{logStdDev}
pmin: \mathrm{min}
pmax: \mathrm{max}
pshape: \mathrm{shape}
prate: \mathrm{rate}
";
        let defs: Definitions = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(defs.get("pmean"), Some(&r"\mathrm{mean}".to_string()));
        assert_eq!(defs.get("pstddev"), Some(&r"\mathrm{stdDev}".to_string()));
        assert_eq!(defs.get("plogmean"), Some(&r"\mathrm{logMean}".to_string()));
        assert_eq!(
            defs.get("plogstddev"),
            Some(&r"\mathrm{logStdDev}".to_string())
        );
        assert_eq!(defs.get("pmin"), Some(&r"\mathrm{min}".to_string()));
        assert_eq!(defs.get("pmax"), Some(&r"\mathrm{max}".to_string()));
        assert_eq!(defs.get("pshape"), Some(&r"\mathrm{shape}".to_string()));
        assert_eq!(defs.get("prate"), Some(&r"\mathrm{rate}".to_string()));
    }

    #[test]
    fn parse_asymptotic_constants() {
        let yaml = r"
cmad: c_{\mathrm{mad}}
cspr: c_{\mathrm{spr}}
";
        let defs: Definitions = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(defs.get("cmad"), Some(&r"c_{\mathrm{mad}}".to_string()));
        assert_eq!(defs.get("cspr"), Some(&r"c_{\mathrm{spr}}".to_string()));
    }

    #[test]
    fn parse_special_symbols() {
        let yaml = r"
misrate: \mathrm{misrate}
approxdist: \sim\mathrm{approx}
";
        let defs: Definitions = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(defs.get("misrate"), Some(&r"\mathrm{misrate}".to_string()));
        assert_eq!(
            defs.get("approxdist"),
            Some(&r"\sim\mathrm{approx}".to_string())
        );
    }

    #[test]
    fn load_real_definitions_file() {
        let def_path = std::path::Path::new("../manual/definitions.yaml");
        if def_path.exists() {
            let defs = load_definitions(def_path).expect("Should load definitions.yaml");

            // Verify estimators are present
            assert!(defs.contains_key("Center"), "Should have Center");
            assert!(defs.contains_key("Spread"), "Should have Spread");
            assert!(defs.contains_key("Mean"), "Should have Mean");
            assert!(defs.contains_key("Median"), "Should have Median");

            // Verify distributions are present
            assert!(defs.contains_key("Additive"), "Should have Additive");
            assert!(defs.contains_key("Multiplic"), "Should have Multiplic");

            // Verify distribution parameters are present
            assert!(defs.contains_key("pmean"), "Should have pmean");
            assert!(defs.contains_key("pstddev"), "Should have pstddev");
            assert!(defs.contains_key("plogmean"), "Should have plogmean");
            assert!(defs.contains_key("plogstddev"), "Should have plogstddev");
            assert!(defs.contains_key("pmin"), "Should have pmin");
            assert!(defs.contains_key("pmax"), "Should have pmax");
            assert!(defs.contains_key("pshape"), "Should have pshape");
            assert!(defs.contains_key("prate"), "Should have prate");

            // Verify asymptotic constants are present
            assert!(defs.contains_key("cmad"), "Should have cmad");
            assert!(defs.contains_key("cspr"), "Should have cspr");

            // Verify special symbols are present
            assert!(defs.contains_key("misrate"), "Should have misrate");
            assert!(defs.contains_key("approxdist"), "Should have approxdist");

            // Verify the actual values
            assert_eq!(
                defs.get("pmean"),
                Some(&r"\mathrm{mean}".to_string()),
                "pmean should map to \\mathrm{{mean}}"
            );
            assert_eq!(
                defs.get("pstddev"),
                Some(&r"\mathrm{stdDev}".to_string()),
                "pstddev should map to \\mathrm{{stdDev}}"
            );
        }
    }
}
