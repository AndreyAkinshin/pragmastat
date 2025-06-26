use float_cmp::approx_eq;
use pragmastat::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
struct OneSampleInput {
    x: Vec<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TwoSampleInput {
    x: Vec<f64>,
    y: Vec<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct OneSampleTestCase {
    input: OneSampleInput,
    output: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct TwoSampleTestCase {
    input: TwoSampleInput,
    output: f64,
}

fn find_repo_root() -> PathBuf {
    let mut current_dir = std::env::current_dir().unwrap();
    loop {
        if current_dir.join("build.cmd").exists() {
            return current_dir;
        }
        if !current_dir.pop() {
            panic!("Could not find repository root (build.cmd not found)");
        }
    }
}

fn run_one_sample_tests<F>(estimator_name: &str, estimator_func: F)
where
    F: Fn(&[f64]) -> Result<f64, &'static str>,
{
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join(estimator_name);

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

    for json_file in json_files {
        let content = fs::read_to_string(&json_file).unwrap();
        let test_case: OneSampleTestCase = serde_json::from_str(&content).unwrap();

        let actual_output = estimator_func(&test_case.input.x).unwrap();
        let expected_output = test_case.output;

        assert!(
            approx_eq!(f64, actual_output, expected_output, epsilon = 1e-10),
            "Failed for test file: {:?}, expected: {}, got: {}",
            json_file.file_name().unwrap(),
            expected_output,
            actual_output
        );
    }
}

fn run_two_sample_tests<F>(estimator_name: &str, estimator_func: F)
where
    F: Fn(&[f64], &[f64]) -> Result<f64, &'static str>,
{
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join(estimator_name);

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

    for json_file in json_files {
        let content = fs::read_to_string(&json_file).unwrap();
        let test_case: TwoSampleTestCase = serde_json::from_str(&content).unwrap();

        let actual_output = estimator_func(&test_case.input.x, &test_case.input.y).unwrap();
        let expected_output = test_case.output;

        assert!(
            approx_eq!(f64, actual_output, expected_output, epsilon = 1e-10)
                || (actual_output.is_infinite() && expected_output.is_infinite()),
            "Failed for test file: {:?}, expected: {}, got: {}",
            json_file.file_name().unwrap(),
            expected_output,
            actual_output
        );
    }
}

#[test]
fn test_center() {
    run_one_sample_tests("center", center);
}

#[test]
fn test_spread() {
    run_one_sample_tests("spread", spread);
}

#[test]
fn test_volatility() {
    run_one_sample_tests("volatility", volatility);
}

#[test]
fn test_precision() {
    run_one_sample_tests("precision", precision);
}

#[test]
fn test_med_shift() {
    run_two_sample_tests("med-shift", med_shift);
}

#[test]
fn test_med_ratio() {
    run_two_sample_tests("med-ratio", med_ratio);
}

#[test]
fn test_med_spread() {
    run_two_sample_tests("med-spread", med_spread);
}

#[test]
fn test_med_disparity() {
    run_two_sample_tests("med-disparity", med_disparity);
}
