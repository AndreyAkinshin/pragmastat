//! Assumption violation conformance tests.
//!
//! These tests verify that assumption violations are reported correctly and
//! consistently across all languages. The test data is loaded from shared
//! JSON files in tests/assumptions/.

use pragmastat::{
    avg_spread, center, disparity, ratio, rel_spread, shift, spread, AssumptionError, AssumptionId,
    Subject,
};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// Expected violation from the test case.
#[derive(Debug, Deserialize)]
struct ExpectedViolation {
    id: String,
    subject: String,
}

/// Input data for assumption tests.
/// Values can be numbers or special strings like "NaN", "Infinity", "-Infinity".
#[derive(Debug, Deserialize)]
struct TestInputs {
    x: Option<Vec<serde_json::Value>>,
    y: Option<Vec<serde_json::Value>>,
}

/// A single test case for assumption violation testing.
#[derive(Debug, Deserialize)]
struct AssumptionTestCase {
    name: String,
    function: String,
    inputs: TestInputs,
    expected_violation: ExpectedViolation,
}

/// A test suite containing multiple test cases.
#[derive(Debug, Deserialize)]
struct AssumptionTestSuite {
    suite: String,
    #[allow(dead_code)]
    description: String,
    cases: Vec<AssumptionTestCase>,
}

/// The manifest describing all available test suites.
#[derive(Debug, Deserialize)]
struct SuiteEntry {
    #[allow(dead_code)]
    name: String,
    file: String,
    #[allow(dead_code)]
    description: String,
}

#[derive(Debug, Deserialize)]
struct Manifest {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    description: String,
    suites: Vec<SuiteEntry>,
}

/// Finds the repository root by looking for CITATION.cff.
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

/// Parses a JSON value into an f64, handling special values.
fn parse_value(v: &serde_json::Value) -> f64 {
    match v {
        serde_json::Value::Number(n) => n.as_f64().unwrap(),
        serde_json::Value::String(s) => match s.as_str() {
            "NaN" => f64::NAN,
            "Infinity" => f64::INFINITY,
            "-Infinity" => f64::NEG_INFINITY,
            _ => panic!("Unknown string value: {}", s),
        },
        _ => panic!("Unexpected value type: {:?}", v),
    }
}

/// Parses an optional array of JSON values into a Vec<f64>.
fn parse_array(arr: &Option<Vec<serde_json::Value>>) -> Vec<f64> {
    arr.as_ref()
        .map(|a| a.iter().map(parse_value).collect())
        .unwrap_or_default()
}

/// Maps assumption ID string to AssumptionId enum.
fn assumption_id_from_str(s: &str) -> AssumptionId {
    match s {
        "validity" => AssumptionId::Validity,
        "positivity" => AssumptionId::Positivity,
        "sparity" => AssumptionId::Sparity,
        _ => panic!("Unknown assumption ID: {}", s),
    }
}

/// Maps subject string to Subject enum.
fn subject_from_str(s: &str) -> Subject {
    match s {
        "x" => Subject::X,
        "y" => Subject::Y,
        _ => panic!("Unknown subject: {}", s),
    }
}

/// Function dispatch: maps function names to actual implementations.
fn call_function(func_name: &str, x: &[f64], y: &[f64]) -> Result<f64, AssumptionError> {
    match func_name {
        "Center" => center(x),
        "Ratio" => ratio(x, y),
        "RelSpread" => rel_spread(x),
        "Spread" => spread(x),
        "Shift" => shift(x, y),
        "AvgSpread" => avg_spread(x, y),
        "Disparity" => disparity(x, y),
        _ => panic!("Unknown function: {}", func_name),
    }
}

/// Loads and runs all assumption test suites.
fn run_assumption_tests() {
    let repo_root = find_repo_root();
    let assumptions_dir = repo_root.join("tests").join("assumptions");

    // Load manifest
    let manifest_path = assumptions_dir.join("manifest.json");
    let manifest_content = fs::read_to_string(&manifest_path)
        .unwrap_or_else(|_| panic!("Failed to read manifest: {:?}", manifest_path));
    let manifest: Manifest = serde_json::from_str(&manifest_content)
        .unwrap_or_else(|e| panic!("Failed to parse manifest: {}", e));

    // Track results for summary
    let mut total_tests = 0;
    let mut passed_tests = 0;
    let mut failures: Vec<String> = Vec::new();

    // Run each suite
    for suite_entry in &manifest.suites {
        let suite_path = assumptions_dir.join(&suite_entry.file);
        let suite_content = fs::read_to_string(&suite_path)
            .unwrap_or_else(|_| panic!("Failed to read suite: {:?}", suite_path));
        let suite: AssumptionTestSuite = serde_json::from_str(&suite_content)
            .unwrap_or_else(|e| panic!("Failed to parse suite {}: {}", suite_entry.name, e));

        for test_case in &suite.cases {
            total_tests += 1;
            let x = parse_array(&test_case.inputs.x);
            let y = parse_array(&test_case.inputs.y);

            let expected_id = assumption_id_from_str(&test_case.expected_violation.id);
            let expected_subject = subject_from_str(&test_case.expected_violation.subject);

            let result = call_function(&test_case.function, &x, &y);

            match result {
                Ok(_) => {
                    failures.push(format!(
                        "{}/{}: Expected violation {}({}) but got success",
                        suite.suite,
                        test_case.name,
                        test_case.expected_violation.id,
                        test_case.expected_violation.subject
                    ));
                }
                Err(err) => {
                    let violation = err.violation();
                    if violation.id == expected_id && violation.subject == expected_subject {
                        passed_tests += 1;
                    } else {
                        failures.push(format!(
                            "{}/{}: Expected {}({}) but got {}({})",
                            suite.suite,
                            test_case.name,
                            expected_id.as_str(),
                            expected_subject.as_str(),
                            violation.id.as_str(),
                            violation.subject.as_str()
                        ));
                    }
                }
            }
        }
    }

    // Print summary
    println!(
        "\nAssumption Tests: {}/{} passed",
        passed_tests, total_tests
    );

    if !failures.is_empty() {
        println!("\nFailures:");
        for failure in &failures {
            println!("  - {}", failure);
        }
        panic!(
            "{} assumption test(s) failed out of {}",
            failures.len(),
            total_tests
        );
    }
}

#[test]
fn test_assumption_violations() {
    run_assumption_tests();
}
