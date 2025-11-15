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

#[derive(Debug, Deserialize, Serialize)]
struct PairwiseMarginInput {
    n: usize,
    m: usize,
    misrate: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct PairwiseMarginTestCase {
    input: PairwiseMarginInput,
    output: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct ShiftBoundsInput {
    x: Vec<f64>,
    y: Vec<f64>,
    misrate: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct BoundsOutput {
    lower: f64,
    upper: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct ShiftBoundsTestCase {
    input: ShiftBoundsInput,
    output: BoundsOutput,
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
fn test_rel_spread() {
    run_one_sample_tests("rel-spread", rel_spread);
}

#[test]
fn test_shift() {
    run_two_sample_tests("shift", shift);
}

#[test]
fn test_ratio() {
    run_two_sample_tests("ratio", ratio);
}

#[test]
fn test_avg_spread() {
    run_two_sample_tests("avg-spread", avg_spread);
}

#[test]
fn test_disparity() {
    run_two_sample_tests("disparity", disparity);
}

fn run_pairwise_margin_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("pairwise-margin");

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
        let test_case: PairwiseMarginTestCase = serde_json::from_str(&content).unwrap();

        let actual_output = pairwise_margin(
            test_case.input.n,
            test_case.input.m,
            test_case.input.misrate,
        );
        let expected_output = test_case.output;

        assert_eq!(
            actual_output,
            expected_output,
            "Failed for test file: {:?}, expected: {}, got: {}",
            json_file.file_name().unwrap(),
            expected_output,
            actual_output
        );
    }
}

fn run_shift_bounds_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("shift-bounds");

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
        let test_case: ShiftBoundsTestCase = serde_json::from_str(&content).unwrap();

        let actual_output = shift_bounds(
            &test_case.input.x,
            &test_case.input.y,
            test_case.input.misrate,
        )
        .unwrap();
        let expected_lower = test_case.output.lower;
        let expected_upper = test_case.output.upper;

        assert!(
            approx_eq!(f64, actual_output.lower, expected_lower, epsilon = 1e-10),
            "Failed for test file: {:?}, expected lower: {}, got: {}",
            json_file.file_name().unwrap(),
            expected_lower,
            actual_output.lower
        );

        assert!(
            approx_eq!(f64, actual_output.upper, expected_upper, epsilon = 1e-10),
            "Failed for test file: {:?}, expected upper: {}, got: {}",
            json_file.file_name().unwrap(),
            expected_upper,
            actual_output.upper
        );
    }
}

#[test]
fn test_pairwise_margin() {
    run_pairwise_margin_tests();
}

#[test]
fn test_shift_bounds() {
    run_shift_bounds_tests();
}
