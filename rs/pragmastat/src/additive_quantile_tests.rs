//! The reproducible inverse normal CDF against the shared `additive-quantile` fixture.
//!
//! Every port carries its own `additive_quantile` because IEEE 754 fixes nothing about an inverse normal CDF, and
//! a distribution that inverts its cumulative function turns a last-bit difference into a
//! different quantile. Nothing calls the function yet, so unlike the exponential it is not reached
//! even indirectly through a margin: without this suite the seven implementations would meet
//! nowhere at all.
//!
//! The suite is declared `exact` in `tests/manifest.json`, so the comparison is by binary64
//! payload. A tolerance here would pass on precisely the divergence the suite exists to catch:
//! two implementations one ULP apart agree to 1e-15 and disagree on the quantile.
//!
//! This module lives in `src` rather than in `tests/` because `additive_quantile` is `pub(crate)`, the
//! same reason `exp_function_tests` does.

use crate::additive_quantile::additive_quantile;
use crate::conformance::bitwise_mismatch;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// One fixture file: the whole argument list, not one case per file as in the estimator suites.
///
/// `name` is read and checked rather than ignored. The files share a shape with every other
/// single-value suite, so a file from a different suite deserializes into this struct without
/// complaint and would be silently checked against the wrong function.
#[derive(Debug, Deserialize)]
struct AdditiveQuantileInput {
    name: String,
    arg: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct AdditiveQuantileTestCase {
    input: AdditiveQuantileInput,
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

/// Checks `additive_quantile` on every argument of every file in the suite.
///
/// The arguments reach the smallest subnormal (`5E-324`, payload 0x1), which is why the crate asks
/// serde_json for `float_roundtrip`: the fast parser is accurate to within an ULP and not
/// correctly rounded, and an ULP at that magnitude is the entire value. Here the mangling would
/// land on the input rather than on the output, and the inverse normal CDF of a neighboring subnormal
/// differs in the last places, so a parser that got it wrong would fail here rather than pass
/// quietly.
#[test]
fn test_additive_quantile_reference() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("additive-quantile");

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
        let test_case: AdditiveQuantileTestCase = serde_json::from_str(&content).unwrap();
        let file_name = json_file.file_name().unwrap();

        assert_eq!(
            test_case.input.name, "additive_quantile",
            "{file_name:?}: expected the additive_quantile suite, got \"{}\"",
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
                &format!("log({arg}) [arg {i}]"),
                expected,
                additive_quantile(arg),
            ) {
                failures.push(format!("{file_name:?}: {mismatch}"));
            }
        }
    }

    // The size of the generated fixture: six files, 2838 arguments. Pinned rather than counted
    // from the directory, because counting the directory and then asserting the count matches the
    // directory asserts nothing. The other six ports pin the same two numbers; a loader that only
    // asserts it did some work cannot see a file that stopped being read or an argument list that
    // was truncated.
    const FIXTURE_FILE_COUNT: usize = 5;
    const FIXTURE_ARGUMENT_COUNT: usize = 547;

    assert_eq!(
        json_files.len(),
        FIXTURE_FILE_COUNT,
        "additive-quantile: read {} files, expected {FIXTURE_FILE_COUNT}",
        json_files.len()
    );
    assert_eq!(
        checked, FIXTURE_ARGUMENT_COUNT,
        "additive-quantile: compared {checked} arguments, expected {FIXTURE_ARGUMENT_COUNT}"
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
