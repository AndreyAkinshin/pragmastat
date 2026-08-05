//! The reproducible logarithm against the shared `log-function` fixture.
//!
//! Every port carries its own `log_function` because IEEE 754 fixes nothing about the logarithm,
//! and ratio is exp(median(log x - log y)): a last-bit difference in log moves the median's input
//! and changes which pairwise element gets selected. That is a different answer, not a small error
//! in the same one.
//!
//! Unlike the exponential, this function is reached by every ratio fixture. That is still not
//! cover, because those suites compare with a tolerance, and a tolerance forgives exactly the
//! last-bit divergence described above. This suite is declared `exact` in `tests/manifest.json`
//! and compares binary64 payloads.
//!
//! This module lives in `src` rather than in `tests/` because `log_function` is `pub(crate)`, the
//! same reason `exp_function_tests` does.

use crate::conformance::bitwise_mismatch;
use crate::log_function::log_function;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// One fixture file: the whole argument list, not one case per file as in the estimator suites.
///
/// `name` is read and checked rather than ignored. The five files share a shape with every other
/// single-value suite, so a file from a different suite deserializes into this struct without
/// complaint and would be silently checked against the wrong function.
#[derive(Debug, Deserialize)]
struct LogFunctionInput {
    name: String,
    arg: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct LogFunctionTestCase {
    input: LogFunctionInput,
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

/// Checks `log_function` on every argument of every file in the suite.
///
/// Only the outputs are compared. The suite makes no claim about the payload of an input, and
/// comparing the parsed arguments against the file would test the JSON reader rather than the
/// logarithm.
///
/// The arguments, unlike the exponential suite's outputs, are what reach the denormal range:
/// `boundaries.json` carries `5E-324` and `1e-320`, and `finite-range.json` starts there. That is
/// why the crate asks serde_json for `float_roundtrip`. The fast parser is accurate to within an
/// ULP and not correctly rounded, and at that magnitude an ULP is the entire value, which would
/// arrive here as a wrong argument and be reported as a wrong logarithm.
#[test]
fn test_log_function_reference() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("log-function");

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
        let test_case: LogFunctionTestCase = serde_json::from_str(&content).unwrap();
        let file_name = json_file.file_name().unwrap();

        assert_eq!(
            test_case.input.name, "log_function",
            "{file_name:?}: expected the log_function suite, got \"{}\"",
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
                log_function(arg),
            ) {
                failures.push(format!("{file_name:?}: {mismatch}"));
            }
        }
    }

    // The size of the generated fixture: five files, 1745 arguments. Pinned rather than counted
    // from the directory, because counting the directory and then asserting the count matches the
    // directory asserts nothing. The other six ports pin the same two numbers; a loader that only
    // asserts it did some work cannot see a file that stopped being read or an argument list that
    // was truncated.
    const FIXTURE_FILE_COUNT: usize = 5;
    const FIXTURE_ARGUMENT_COUNT: usize = 1745;

    assert_eq!(
        json_files.len(),
        FIXTURE_FILE_COUNT,
        "log-function: read {} files, expected {FIXTURE_FILE_COUNT}",
        json_files.len()
    );
    assert_eq!(
        checked, FIXTURE_ARGUMENT_COUNT,
        "log-function: compared {checked} arguments, expected {FIXTURE_ARGUMENT_COUNT}"
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
