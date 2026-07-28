//! The reproducible exponential against the shared `portable-exp` fixture.
//!
//! Every port carries its own `portable_exp` because IEEE 754 fixes nothing about the
//! exponential, and the margins turn a last-bit difference into a different order statistic and
//! so into a different confidence interval. Until this suite existed, nothing compared the seven
//! implementations directly: they met only through the margin fixtures, which reach the
//! exponential wherever an Edgeworth expansion happens to look and nowhere else.
//!
//! The suite is declared `exact` in `tests/manifest.json`, so the comparison is by binary64
//! payload. A tolerance here would pass on precisely the divergence the suite exists to catch:
//! two implementations one ULP apart agree to 1e-15 and disagree on the interval.
//!
//! This module lives in `src` rather than in `tests/` because `portable_exp` is `pub(crate)`,
//! the same reason `pairwise_margin_tests` and `signed_rank_margin_tests` do.

use crate::conformance::bitwise_mismatch;
use crate::portable_exp::portable_exp;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// One fixture file: the whole argument list, not one case per file as in the estimator suites.
///
/// `name` is read and checked rather than ignored. The four files share a shape with every other
/// single-value suite, so a file from a different suite deserializes into this struct without
/// complaint and would be silently checked against the wrong function.
#[derive(Debug, Deserialize)]
struct PortableExpInput {
    name: String,
    arg: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct PortableExpTestCase {
    input: PortableExpInput,
    output: Vec<f64>,
}

fn find_repo_root() -> PathBuf {
    let mut current_dir = std::env::current_dir().unwrap();
    loop {
        if current_dir.join("CITATION.cff").exists() {
            return current_dir;
        }
        if !current_dir.pop() {
            panic!("Could not find repository root (CITATION.cff not found)");
        }
    }
}

/// Checks `portable_exp` on every argument of every file in the suite.
///
/// Only the outputs are compared. `boundaries.json` lists both `0` and `-0`, which JSON does not
/// distinguish and which serde therefore hands over as `+0.0` twice; both are arguments to
/// `exp`, both give 1, and nothing downstream depends on which one arrived. Comparing the parsed
/// input against the file would turn a property of JSON into a test failure.
///
/// The outputs reach the smallest denormal (`5E-324`, payload 0x1), which is why the crate asks
/// serde_json for `float_roundtrip`: the fast parser is accurate to within an ULP and not
/// correctly rounded, and an ULP at that magnitude is the entire value. A parser that mangled
/// them would fail here rather than pass quietly, since the comparison is on payloads.
#[test]
fn test_portable_exp_reference() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("portable-exp");

    if !test_data_dir.exists() {
        panic!("Test data directory not found: {test_data_dir:?}");
    }

    let mut json_files: Vec<_> = fs::read_dir(&test_data_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension()?.to_str()? == "json" {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    // read_dir order is whatever the filesystem says; sorting keeps a failure report comparable
    // between runs and between machines.
    json_files.sort();

    assert!(
        !json_files.is_empty(),
        "No JSON test files found in {test_data_dir:?}"
    );

    let mut checked = 0;
    let mut failures = Vec::new();

    for json_file in &json_files {
        let content = fs::read_to_string(json_file).unwrap();
        let test_case: PortableExpTestCase = serde_json::from_str(&content).unwrap();
        let file_name = json_file.file_name().unwrap();

        assert_eq!(
            test_case.input.name, "portable_exp",
            "{file_name:?}: expected the portable_exp suite, got \"{}\"",
            test_case.input.name
        );
        assert_eq!(
            test_case.input.arg.len(),
            test_case.output.len(),
            "{file_name:?}: {} arguments against {} outputs",
            test_case.input.arg.len(),
            test_case.output.len()
        );
        // A file that parses into empty vectors passes every comparison below, which is the one
        // way this test could report success while checking nothing.
        assert!(
            !test_case.input.arg.is_empty(),
            "{file_name:?}: no arguments"
        );

        for (i, (&arg, &expected)) in test_case
            .input
            .arg
            .iter()
            .zip(test_case.output.iter())
            .enumerate()
        {
            checked += 1;
            if let Some(mismatch) = bitwise_mismatch(
                &format!("exp({arg}) [arg {i}]"),
                expected,
                portable_exp(arg),
            ) {
                failures.push(format!("{file_name:?}: {mismatch}"));
            }
        }
    }

    assert!(
        checked > 0,
        "No arguments were checked out of {} files",
        json_files.len()
    );

    assert!(
        failures.is_empty(),
        "Failed {} of {} arguments across {} files:\n{}",
        failures.len(),
        checked,
        json_files.len(),
        failures.join("\n")
    );
}
