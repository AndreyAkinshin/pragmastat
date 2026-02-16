use crate::assumptions::{AssumptionId, EstimatorError, Subject};
use crate::estimators::{disparity_bounds, disparity_bounds_with_seed};
use float_cmp::approx_eq;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct DisparityBoundsInput {
    x: Vec<f64>,
    y: Vec<f64>,
    misrate: f64,
    seed: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BoundsOutput {
    lower: f64,
    upper: f64,
}

#[derive(Debug, Deserialize)]
struct DisparityBoundsTestCase {
    input: DisparityBoundsInput,
    output: Option<BoundsOutput>,
    expected_error: Option<serde_json::Value>,
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

#[test]
fn test_disparity_bounds_reference() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("disparity-bounds");

    if !test_data_dir.exists() {
        panic!("Test data directory not found: {:?}", test_data_dir);
    }

    let json_files: Vec<_> = fs::read_dir(&test_data_dir)
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

    assert!(
        !json_files.is_empty(),
        "No JSON test files found in {:?}",
        test_data_dir
    );

    let mut failures = Vec::new();

    for json_file in &json_files {
        let content = fs::read_to_string(json_file).unwrap();
        let test_case: DisparityBoundsTestCase = serde_json::from_str(&content).unwrap();
        let file_name = json_file.file_name().unwrap();

        let seed = test_case.input.seed.as_deref();

        if let Some(ref expected_error) = test_case.expected_error {
            let result = match seed {
                Some(s) => disparity_bounds_with_seed(
                    &test_case.input.x,
                    &test_case.input.y,
                    test_case.input.misrate,
                    s,
                ),
                None => disparity_bounds(
                    &test_case.input.x,
                    &test_case.input.y,
                    test_case.input.misrate,
                ),
            };
            match result {
                Ok(_) => failures.push(format!("{file_name:?}: expected error, got Ok")),
                Err(err) => {
                    if let Some(expected_id) = expected_error.get("id").and_then(|v| v.as_str()) {
                        if let EstimatorError::Assumption(ref ae) = err {
                            if ae.violation().id.as_str() != expected_id {
                                failures.push(format!(
                                    "{file_name:?}: expected violation id {expected_id}, got {}",
                                    ae.violation().id.as_str()
                                ));
                            }
                        } else {
                            failures.push(format!(
                                "{file_name:?}: expected AssumptionError, got {err:?}"
                            ));
                        }
                    }
                }
            }
            continue;
        }

        let actual_output = match seed {
            Some(s) => disparity_bounds_with_seed(
                &test_case.input.x,
                &test_case.input.y,
                test_case.input.misrate,
                s,
            ),
            None => disparity_bounds(
                &test_case.input.x,
                &test_case.input.y,
                test_case.input.misrate,
            ),
        };
        let actual_output = match actual_output {
            Ok(val) => val,
            Err(e) => {
                failures.push(format!("{file_name:?}: unexpected error {e:?}"));
                continue;
            }
        };
        let expected_output = test_case.output.expect("Test case must have output");

        if !approx_eq!(
            f64,
            actual_output.lower,
            expected_output.lower,
            epsilon = 1e-9
        ) {
            failures.push(format!(
                "{file_name:?}: expected lower {}, got {}",
                expected_output.lower, actual_output.lower
            ));
        }
        if !approx_eq!(
            f64,
            actual_output.upper,
            expected_output.upper,
            epsilon = 1e-9
        ) {
            failures.push(format!(
                "{file_name:?}: expected upper {}, got {}",
                expected_output.upper, actual_output.upper
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "Failed tests:\n{}",
        failures.join("\n")
    );
}

#[test]
fn disparity_bounds_empty_x() {
    assert!(disparity_bounds(&[], &[1.0, 2.0], 0.1).is_err());
}

#[test]
fn disparity_bounds_empty_y() {
    assert!(disparity_bounds(&[1.0, 2.0], &[], 0.1).is_err());
}

#[test]
fn disparity_bounds_misrate_below_min() {
    let result = disparity_bounds(&[1.0, 2.0, 3.0, 4.0], &[1.0, 2.0, 3.0, 4.0], 0.1);
    assert!(result.is_err());
    if let Err(EstimatorError::Assumption(ref ae)) = result {
        assert_eq!(ae.violation().id, AssumptionId::Domain);
        assert_eq!(ae.violation().subject, Subject::Misrate);
    } else {
        panic!("Expected AssumptionError::Domain for misrate");
    }
}
