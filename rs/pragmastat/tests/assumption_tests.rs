//! Assumption violation conformance tests.
//!
//! These tests verify that assumption violations are reported correctly and
//! consistently across all languages. The test data is loaded from shared
//! JSON files in tests/assumptions/.

use pragmastat::{
    avg_spread, center, center_bounds, center_bounds_approx_with_seed, disparity, median_bounds,
    ratio, rel_spread, shift, signed_rank_margin, spread, AssumptionError, AssumptionId,
    EstimatorError,
};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// Expected violation from the test case.
#[derive(Debug, Deserialize)]
struct ExpectedViolation {
    id: String,
}

/// Input data for assumption tests.
/// Values can be numbers or special strings like "NaN", "Infinity", "-Infinity".
#[derive(Debug, Deserialize)]
struct TestInputs {
    x: Option<Vec<serde_json::Value>>,
    y: Option<Vec<serde_json::Value>>,
    misrate: Option<serde_json::Value>,
    n: Option<usize>,
    seed: Option<String>,
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
        "domain" => AssumptionId::Domain,
        _ => panic!("Unknown assumption ID: {}", s),
    }
}

/// Dispatches to the appropriate estimator function.
/// Returns Ok(()) on success, Err with the assumption error on violation.
fn call_function(func_name: &str, inputs: &TestInputs) -> Result<(), AssumptionError> {
    let x = parse_array(&inputs.x);
    let y = parse_array(&inputs.y);

    let extract = |r: Result<_, EstimatorError>| -> Result<(), AssumptionError> {
        match r {
            Ok(_) => Ok(()),
            Err(EstimatorError::Assumption(a)) => Err(a),
            Err(e) => panic!("Unexpected error: {e}"),
        }
    };

    match func_name {
        "Center" => extract(center(&x).map(|_| ()))?,
        "Ratio" => extract(ratio(&x, &y).map(|_| ()))?,
        "RelSpread" => extract(rel_spread(&x).map(|_| ()))?,
        "Spread" => extract(spread(&x).map(|_| ()))?,
        "Shift" => extract(shift(&x, &y).map(|_| ()))?,
        "AvgSpread" => extract(avg_spread(&x, &y).map(|_| ()))?,
        "Disparity" => extract(disparity(&x, &y).map(|_| ()))?,
        "MedianBounds" => {
            extract(median_bounds(&x, parse_value(inputs.misrate.as_ref().unwrap())).map(|_| ()))?
        }
        "CenterBounds" => {
            extract(center_bounds(&x, parse_value(inputs.misrate.as_ref().unwrap())).map(|_| ()))?
        }
        "CenterBoundsApprox" => extract(
            center_bounds_approx_with_seed(
                &x,
                parse_value(inputs.misrate.as_ref().unwrap()),
                inputs.seed.as_deref(),
            )
            .map(|_| ()),
        )?,
        "SignedRankMargin" => {
            signed_rank_margin(
                inputs.n.unwrap(),
                parse_value(inputs.misrate.as_ref().unwrap()),
            )?;
        }
        _ => panic!("Unknown function: {}", func_name),
    }
    Ok(())
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

            let expected_id = assumption_id_from_str(&test_case.expected_violation.id);

            let result = call_function(&test_case.function, &test_case.inputs);

            match result {
                Ok(_) => {
                    failures.push(format!(
                        "{}/{}: Expected violation {} but got success",
                        suite.suite, test_case.name, test_case.expected_violation.id
                    ));
                }
                Err(err) => {
                    let violation = err.violation();
                    if violation.id == expected_id {
                        passed_tests += 1;
                    } else {
                        failures.push(format!(
                            "{}/{}: Expected {} but got {}",
                            suite.suite,
                            test_case.name,
                            expected_id.as_str(),
                            violation.id.as_str()
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
