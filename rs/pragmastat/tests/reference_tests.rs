use float_cmp::approx_eq;
use pragmastat::pairwise_margin::pairwise_margin;
use pragmastat::signed_rank_margin::signed_rank_margin;
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
    output: Option<usize>,
    expected_error: Option<serde_json::Value>,
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
    output: Option<BoundsOutput>,
    expected_error: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RatioBoundsInput {
    x: Vec<f64>,
    y: Vec<f64>,
    misrate: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct RatioBoundsTestCase {
    input: RatioBoundsInput,
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

fn run_one_sample_tests<F, E>(estimator_name: &str, estimator_func: F)
where
    F: Fn(&[f64]) -> Result<f64, E>,
    E: std::fmt::Debug,
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

    let mut executed_count = 0;
    let total_count = json_files.len();
    let mut failures = Vec::new();

    for json_file in &json_files {
        let content = fs::read_to_string(json_file).unwrap();
        let test_case: OneSampleTestCase = serde_json::from_str(&content).unwrap();

        // Skip test if it returns an error (assumption violation tests handled separately)
        let actual_output = match estimator_func(&test_case.input.x) {
            Ok(val) => val,
            Err(_) => continue,
        };
        let expected_output = test_case.output;

        executed_count += 1;
        if !approx_eq!(f64, actual_output, expected_output, epsilon = 1e-9) {
            failures.push(format!(
                "{:?}: expected {}, got {}",
                json_file.file_name().unwrap(),
                expected_output,
                actual_output
            ));
        }
    }

    // Ensure at least some tests were actually executed
    assert!(
        executed_count > 0,
        "All {} tests were skipped due to assumption violations",
        total_count
    );

    assert!(
        failures.is_empty(),
        "Failed tests:\n{}",
        failures.join("\n")
    );
}

fn run_two_sample_tests<F, E>(estimator_name: &str, estimator_func: F)
where
    F: Fn(&[f64], &[f64]) -> Result<f64, E>,
    E: std::fmt::Debug,
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

    let mut executed_count = 0;
    let total_count = json_files.len();
    let mut failures = Vec::new();

    for json_file in &json_files {
        let content = fs::read_to_string(json_file).unwrap();
        let test_case: TwoSampleTestCase = serde_json::from_str(&content).unwrap();

        // Skip test if it returns an error (assumption violation tests handled separately)
        let actual_output = match estimator_func(&test_case.input.x, &test_case.input.y) {
            Ok(val) => val,
            Err(_) => continue,
        };
        let expected_output = test_case.output;

        executed_count += 1;
        if !(approx_eq!(f64, actual_output, expected_output, epsilon = 1e-9)
            || (actual_output.is_infinite() && expected_output.is_infinite()))
        {
            failures.push(format!(
                "{:?}: expected {}, got {}",
                json_file.file_name().unwrap(),
                expected_output,
                actual_output
            ));
        }
    }

    // Ensure at least some tests were actually executed
    assert!(
        executed_count > 0,
        "All {} tests were skipped due to assumption violations",
        total_count
    );

    assert!(
        failures.is_empty(),
        "Failed tests:\n{}",
        failures.join("\n")
    );
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

    let mut failures = Vec::new();

    for json_file in &json_files {
        let content = fs::read_to_string(json_file).unwrap();
        let test_case: PairwiseMarginTestCase = serde_json::from_str(&content).unwrap();
        let file_name = json_file.file_name().unwrap();

        // Handle error test cases
        if let Some(ref expected_error) = test_case.expected_error {
            let result = pairwise_margin(
                test_case.input.n,
                test_case.input.m,
                test_case.input.misrate,
            );
            match result {
                Ok(_) => failures.push(format!("{file_name:?}: expected error, got Ok")),
                Err(err) => {
                    if let Some(expected_id) = expected_error.get("id").and_then(|v| v.as_str()) {
                        if err.violation().id.as_str() != expected_id {
                            failures.push(format!(
                                "{file_name:?}: expected violation id {expected_id}, got {}",
                                err.violation().id.as_str()
                            ));
                        }
                    }
                }
            }
            continue;
        }

        let actual_output = match pairwise_margin(
            test_case.input.n,
            test_case.input.m,
            test_case.input.misrate,
        ) {
            Ok(val) => val,
            Err(e) => {
                failures.push(format!("{file_name:?}: unexpected error {e:?}"));
                continue;
            }
        };
        let expected_output = test_case.output.expect("Test case must have output");

        if actual_output != expected_output {
            failures.push(format!(
                "{file_name:?}: expected {expected_output}, got {actual_output}"
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "Failed tests:\n{}",
        failures.join("\n")
    );
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

    let mut failures = Vec::new();

    for json_file in &json_files {
        let content = fs::read_to_string(json_file).unwrap();
        let test_case: ShiftBoundsTestCase = serde_json::from_str(&content).unwrap();
        let file_name = json_file.file_name().unwrap();

        // Handle error test cases
        if let Some(ref expected_error) = test_case.expected_error {
            let result = shift_bounds(
                &test_case.input.x,
                &test_case.input.y,
                test_case.input.misrate,
            );
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

        let expected_output = test_case.output.expect("Test case must have output");
        let actual_output = match shift_bounds(
            &test_case.input.x,
            &test_case.input.y,
            test_case.input.misrate,
        ) {
            Ok(val) => val,
            Err(e) => {
                failures.push(format!("{file_name:?}: unexpected error {e:?}"));
                continue;
            }
        };

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

fn run_ratio_bounds_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("ratio-bounds");

    if !test_data_dir.exists() {
        eprintln!("Skipping ratio_bounds tests: test data directory not found");
        return;
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

    if json_files.is_empty() {
        eprintln!("Skipping ratio_bounds tests: no JSON files found");
        return;
    }

    let mut failures = Vec::new();

    for json_file in &json_files {
        let content = fs::read_to_string(json_file).unwrap();
        let test_case: RatioBoundsTestCase = serde_json::from_str(&content).unwrap();
        let file_name = json_file.file_name().unwrap();

        // Handle error test cases
        if let Some(ref expected_error) = test_case.expected_error {
            let result = ratio_bounds(
                &test_case.input.x,
                &test_case.input.y,
                test_case.input.misrate,
            );
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

        let expected_output = test_case.output.expect("Test case must have output");
        let actual_output = match ratio_bounds(
            &test_case.input.x,
            &test_case.input.y,
            test_case.input.misrate,
        ) {
            Ok(val) => val,
            Err(e) => {
                failures.push(format!("{file_name:?}: unexpected error {e:?}"));
                continue;
            }
        };

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
fn test_pairwise_margin() {
    run_pairwise_margin_tests();
}

#[test]
fn test_shift_bounds() {
    run_shift_bounds_tests();
}

#[test]
fn test_ratio_bounds() {
    run_ratio_bounds_tests();
}

// Rng reference tests

#[derive(Debug, Deserialize)]
struct UniformInput {
    seed: i64,
    count: usize,
}

#[derive(Debug, Deserialize)]
struct UniformTestCase {
    input: UniformInput,
    output: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct UniformIntInput {
    seed: i64,
    min: i64,
    max: i64,
    count: usize,
}

#[derive(Debug, Deserialize)]
struct UniformIntTestCase {
    input: UniformIntInput,
    output: Vec<i64>,
}

#[derive(Debug, Deserialize)]
struct StringSeedInput {
    seed: String,
    count: usize,
}

#[derive(Debug, Deserialize)]
struct StringSeedTestCase {
    input: StringSeedInput,
    output: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct UniformRangeInput {
    seed: i64,
    min: f64,
    max: f64,
    count: usize,
}

#[derive(Debug, Deserialize)]
struct UniformRangeTestCase {
    input: UniformRangeInput,
    output: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct UniformF32Input {
    seed: i64,
    count: usize,
}

#[derive(Debug, Deserialize)]
struct UniformF32TestCase {
    input: UniformF32Input,
    output: Vec<f32>,
}

#[derive(Debug, Deserialize)]
struct UniformI32Input {
    seed: i64,
    min: i32,
    max: i32,
    count: usize,
}

#[derive(Debug, Deserialize)]
struct UniformI32TestCase {
    input: UniformI32Input,
    output: Vec<i32>,
}

#[derive(Debug, Deserialize)]
struct UniformBoolInput {
    seed: i64,
    count: usize,
}

#[derive(Debug, Deserialize)]
struct UniformBoolTestCase {
    input: UniformBoolInput,
    output: Vec<bool>,
}

#[derive(Debug, Deserialize)]
struct ShuffleInput {
    seed: i64,
    x: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct ShuffleTestCase {
    input: ShuffleInput,
    output: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct SampleInput {
    seed: i64,
    x: Vec<f64>,
    k: usize,
}

#[derive(Debug, Deserialize)]
struct SampleTestCase {
    input: SampleInput,
    output: Vec<f64>,
}

// Distribution reference tests

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UniformDistInput {
    seed: i64,
    min: f64,
    max: f64,
    count: usize,
}

#[derive(Debug, Deserialize)]
struct UniformDistTestCase {
    input: UniformDistInput,
    output: Vec<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AdditiveDistInput {
    seed: i64,
    mean: f64,
    std_dev: f64,
    count: usize,
}

#[derive(Debug, Deserialize)]
struct AdditiveDistTestCase {
    input: AdditiveDistInput,
    output: Vec<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MultiplicDistInput {
    seed: i64,
    log_mean: f64,
    log_std_dev: f64,
    count: usize,
}

#[derive(Debug, Deserialize)]
struct MultiplicDistTestCase {
    input: MultiplicDistInput,
    output: Vec<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExpDistInput {
    seed: i64,
    rate: f64,
    count: usize,
}

#[derive(Debug, Deserialize)]
struct ExpDistTestCase {
    input: ExpDistInput,
    output: Vec<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PowerDistInput {
    seed: i64,
    min: f64,
    shape: f64,
    count: usize,
}

#[derive(Debug, Deserialize)]
struct PowerDistTestCase {
    input: PowerDistInput,
    output: Vec<f64>,
}

fn run_rng_uniform_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("rng");

    let json_files: Vec<_> = fs::read_dir(&test_data_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = path.file_name()?.to_str()?;
            if name.starts_with("uniform-seed-") && name.ends_with(".json") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    assert!(!json_files.is_empty(), "No uniform seed test files found");

    for json_file in json_files {
        let content = fs::read_to_string(&json_file).unwrap();
        let test_case: UniformTestCase = serde_json::from_str(&content).unwrap();

        let mut rng = Rng::from_seed(test_case.input.seed);
        let actual: Vec<f64> = (0..test_case.input.count).map(|_| rng.uniform()).collect();

        for (i, (actual_val, expected_val)) in
            actual.iter().zip(test_case.output.iter()).enumerate()
        {
            assert!(
                approx_eq!(f64, *actual_val, *expected_val, epsilon = 1e-15),
                "Failed for test file: {:?}, index {}, expected: {}, got: {}",
                json_file.file_name().unwrap(),
                i,
                expected_val,
                actual_val
            );
        }
    }
}

fn run_rng_uniform_int_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("rng");

    let json_files: Vec<_> = fs::read_dir(&test_data_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = path.file_name()?.to_str()?;
            if name.starts_with("uniform-int-") && name.ends_with(".json") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    assert!(!json_files.is_empty(), "No uniform int test files found");

    for json_file in json_files {
        let content = fs::read_to_string(&json_file).unwrap();
        let test_case: UniformIntTestCase = serde_json::from_str(&content).unwrap();

        let mut rng = Rng::from_seed(test_case.input.seed);
        let actual: Vec<i64> = (0..test_case.input.count)
            .map(|_| rng.uniform_i64(test_case.input.min, test_case.input.max))
            .collect();

        assert_eq!(
            actual,
            test_case.output,
            "Failed for test file: {:?}",
            json_file.file_name().unwrap()
        );
    }
}

fn run_rng_string_seed_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("rng");

    let json_files: Vec<_> = fs::read_dir(&test_data_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = path.file_name()?.to_str()?;
            if name.starts_with("uniform-string-") && name.ends_with(".json") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    assert!(!json_files.is_empty(), "No string seed test files found");

    for json_file in json_files {
        let content = fs::read_to_string(&json_file).unwrap();
        let test_case: StringSeedTestCase = serde_json::from_str(&content).unwrap();

        let mut rng = Rng::from_string(&test_case.input.seed);
        let actual: Vec<f64> = (0..test_case.input.count).map(|_| rng.uniform()).collect();

        for (i, (actual_val, expected_val)) in
            actual.iter().zip(test_case.output.iter()).enumerate()
        {
            assert!(
                approx_eq!(f64, *actual_val, *expected_val, epsilon = 1e-15),
                "Failed for test file: {:?}, index {}, expected: {}, got: {}",
                json_file.file_name().unwrap(),
                i,
                expected_val,
                actual_val
            );
        }
    }
}

fn run_rng_uniform_range_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("rng");

    let json_files: Vec<_> = fs::read_dir(&test_data_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = path.file_name()?.to_str()?;
            if name.starts_with("uniform-range-") && name.ends_with(".json") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    assert!(!json_files.is_empty(), "No uniform range test files found");

    for json_file in json_files {
        let content = fs::read_to_string(&json_file).unwrap();
        let test_case: UniformRangeTestCase = serde_json::from_str(&content).unwrap();

        let mut rng = Rng::from_seed(test_case.input.seed);
        let actual: Vec<f64> = (0..test_case.input.count)
            .map(|_| rng.uniform_range(test_case.input.min, test_case.input.max))
            .collect();

        for (i, (actual_val, expected_val)) in
            actual.iter().zip(test_case.output.iter()).enumerate()
        {
            assert!(
                approx_eq!(f64, *actual_val, *expected_val, epsilon = 1e-12),
                "Failed for test file: {:?}, index {}, expected: {}, got: {}",
                json_file.file_name().unwrap(),
                i,
                expected_val,
                actual_val
            );
        }
    }
}

fn run_rng_uniform_f32_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("rng");

    let json_files: Vec<_> = fs::read_dir(&test_data_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = path.file_name()?.to_str()?;
            if name.starts_with("uniform-f32-") && name.ends_with(".json") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    assert!(!json_files.is_empty(), "No uniform f32 test files found");

    for json_file in json_files {
        let content = fs::read_to_string(&json_file).unwrap();
        let test_case: UniformF32TestCase = serde_json::from_str(&content).unwrap();

        let mut rng = Rng::from_seed(test_case.input.seed);
        let actual: Vec<f32> = (0..test_case.input.count)
            .map(|_| rng.uniform_f32())
            .collect();

        for (i, (actual_val, expected_val)) in
            actual.iter().zip(test_case.output.iter()).enumerate()
        {
            assert!(
                approx_eq!(f32, *actual_val, *expected_val, epsilon = 1e-7),
                "Failed for test file: {:?}, index {}, expected: {}, got: {}",
                json_file.file_name().unwrap(),
                i,
                expected_val,
                actual_val
            );
        }
    }
}

fn run_rng_uniform_i32_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("rng");

    let json_files: Vec<_> = fs::read_dir(&test_data_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = path.file_name()?.to_str()?;
            if name.starts_with("uniform-i32-") && name.ends_with(".json") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    assert!(!json_files.is_empty(), "No uniform i32 test files found");

    for json_file in json_files {
        let content = fs::read_to_string(&json_file).unwrap();
        let test_case: UniformI32TestCase = serde_json::from_str(&content).unwrap();

        let mut rng = Rng::from_seed(test_case.input.seed);
        let actual: Vec<i32> = (0..test_case.input.count)
            .map(|_| rng.uniform_i32(test_case.input.min, test_case.input.max))
            .collect();

        assert_eq!(
            actual,
            test_case.output,
            "Failed for test file: {:?}",
            json_file.file_name().unwrap()
        );
    }
}

fn run_rng_uniform_bool_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("rng");

    let json_files: Vec<_> = fs::read_dir(&test_data_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = path.file_name()?.to_str()?;
            if name.starts_with("uniform-bool-seed-") && name.ends_with(".json") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    assert!(!json_files.is_empty(), "No uniform bool test files found");

    for json_file in json_files {
        let content = fs::read_to_string(&json_file).unwrap();
        let test_case: UniformBoolTestCase = serde_json::from_str(&content).unwrap();

        let mut rng = Rng::from_seed(test_case.input.seed);
        let actual: Vec<bool> = (0..test_case.input.count)
            .map(|_| rng.uniform_bool())
            .collect();

        assert_eq!(
            actual,
            test_case.output,
            "Failed for test file: {:?}",
            json_file.file_name().unwrap()
        );
    }
}

fn run_shuffle_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("shuffle");

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

    assert!(!json_files.is_empty(), "No shuffle test files found");

    for json_file in json_files {
        let content = fs::read_to_string(&json_file).unwrap();
        let test_case: ShuffleTestCase = serde_json::from_str(&content).unwrap();

        let mut rng = Rng::from_seed(test_case.input.seed);
        let actual = rng.shuffle(&test_case.input.x);

        for (i, (actual_val, expected_val)) in
            actual.iter().zip(test_case.output.iter()).enumerate()
        {
            assert!(
                approx_eq!(f64, *actual_val, *expected_val, epsilon = 1e-15),
                "Failed for test file: {:?}, index {}, expected: {}, got: {}",
                json_file.file_name().unwrap(),
                i,
                expected_val,
                actual_val
            );
        }
    }
}

fn run_sample_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("sample");

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

    assert!(!json_files.is_empty(), "No sample test files found");

    for json_file in json_files {
        let content = fs::read_to_string(&json_file).unwrap();
        let test_case: SampleTestCase = serde_json::from_str(&content).unwrap();

        let mut rng = Rng::from_seed(test_case.input.seed);
        let actual = rng.sample(&test_case.input.x, test_case.input.k);

        for (i, (actual_val, expected_val)) in
            actual.iter().zip(test_case.output.iter()).enumerate()
        {
            assert!(
                approx_eq!(f64, *actual_val, *expected_val, epsilon = 1e-15),
                "Failed for test file: {:?}, index {}, expected: {}, got: {}",
                json_file.file_name().unwrap(),
                i,
                expected_val,
                actual_val
            );
        }
    }
}

fn run_resample_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("resample");

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

    assert!(!json_files.is_empty(), "No resample test files found");

    for json_file in json_files {
        let content = fs::read_to_string(&json_file).unwrap();
        let test_case: SampleTestCase = serde_json::from_str(&content).unwrap();

        let mut rng = Rng::from_seed(test_case.input.seed);
        let actual = rng.resample(&test_case.input.x, test_case.input.k);

        for (i, (actual_val, expected_val)) in
            actual.iter().zip(test_case.output.iter()).enumerate()
        {
            assert!(
                approx_eq!(f64, *actual_val, *expected_val, epsilon = 1e-15),
                "Failed for test file: {:?}, index {}, expected: {}, got: {}",
                json_file.file_name().unwrap(),
                i,
                expected_val,
                actual_val
            );
        }
    }
}

fn run_uniform_distribution_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root
        .join("tests")
        .join("distributions")
        .join("uniform");

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
        "No uniform distribution test files found"
    );

    for json_file in json_files {
        let content = fs::read_to_string(&json_file).unwrap();
        let test_case: UniformDistTestCase = serde_json::from_str(&content).unwrap();

        let mut rng = Rng::from_seed(test_case.input.seed);
        let dist = Uniform::new(test_case.input.min, test_case.input.max);
        let actual: Vec<f64> = (0..test_case.input.count)
            .map(|_| dist.sample(&mut rng))
            .collect();

        for (i, (actual_val, expected_val)) in
            actual.iter().zip(test_case.output.iter()).enumerate()
        {
            assert!(
                approx_eq!(f64, *actual_val, *expected_val, epsilon = 1e-12),
                "Failed for test file: {:?}, index {}, expected: {}, got: {}",
                json_file.file_name().unwrap(),
                i,
                expected_val,
                actual_val
            );
        }
    }
}

fn run_additive_distribution_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root
        .join("tests")
        .join("distributions")
        .join("additive");

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
        "No additive distribution test files found"
    );

    for json_file in json_files {
        let content = fs::read_to_string(&json_file).unwrap();
        let test_case: AdditiveDistTestCase = serde_json::from_str(&content).unwrap();

        let mut rng = Rng::from_seed(test_case.input.seed);
        let dist = Additive::new(test_case.input.mean, test_case.input.std_dev);
        let actual: Vec<f64> = (0..test_case.input.count)
            .map(|_| dist.sample(&mut rng))
            .collect();

        for (i, (actual_val, expected_val)) in
            actual.iter().zip(test_case.output.iter()).enumerate()
        {
            assert!(
                approx_eq!(f64, *actual_val, *expected_val, epsilon = 1e-12),
                "Failed for test file: {:?}, index {}, expected: {}, got: {}",
                json_file.file_name().unwrap(),
                i,
                expected_val,
                actual_val
            );
        }
    }
}

fn run_multiplic_distribution_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root
        .join("tests")
        .join("distributions")
        .join("multiplic");

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
        "No multiplic distribution test files found"
    );

    for json_file in json_files {
        let content = fs::read_to_string(&json_file).unwrap();
        let test_case: MultiplicDistTestCase = serde_json::from_str(&content).unwrap();

        let mut rng = Rng::from_seed(test_case.input.seed);
        let dist = Multiplic::new(test_case.input.log_mean, test_case.input.log_std_dev);
        let actual: Vec<f64> = (0..test_case.input.count)
            .map(|_| dist.sample(&mut rng))
            .collect();

        for (i, (actual_val, expected_val)) in
            actual.iter().zip(test_case.output.iter()).enumerate()
        {
            assert!(
                approx_eq!(f64, *actual_val, *expected_val, epsilon = 1e-12),
                "Failed for test file: {:?}, index {}, expected: {}, got: {}",
                json_file.file_name().unwrap(),
                i,
                expected_val,
                actual_val
            );
        }
    }
}

fn run_exp_distribution_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("distributions").join("exp");

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
        "No exp distribution test files found"
    );

    for json_file in json_files {
        let content = fs::read_to_string(&json_file).unwrap();
        let test_case: ExpDistTestCase = serde_json::from_str(&content).unwrap();

        let mut rng = Rng::from_seed(test_case.input.seed);
        let dist = Exp::new(test_case.input.rate);
        let actual: Vec<f64> = (0..test_case.input.count)
            .map(|_| dist.sample(&mut rng))
            .collect();

        for (i, (actual_val, expected_val)) in
            actual.iter().zip(test_case.output.iter()).enumerate()
        {
            assert!(
                approx_eq!(f64, *actual_val, *expected_val, epsilon = 1e-12),
                "Failed for test file: {:?}, index {}, expected: {}, got: {}",
                json_file.file_name().unwrap(),
                i,
                expected_val,
                actual_val
            );
        }
    }
}

fn run_power_distribution_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("distributions").join("power");

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
        "No power distribution test files found"
    );

    for json_file in json_files {
        let content = fs::read_to_string(&json_file).unwrap();
        let test_case: PowerDistTestCase = serde_json::from_str(&content).unwrap();

        let mut rng = Rng::from_seed(test_case.input.seed);
        let dist = Power::new(test_case.input.min, test_case.input.shape);
        let actual: Vec<f64> = (0..test_case.input.count)
            .map(|_| dist.sample(&mut rng))
            .collect();

        for (i, (actual_val, expected_val)) in
            actual.iter().zip(test_case.output.iter()).enumerate()
        {
            assert!(
                approx_eq!(f64, *actual_val, *expected_val, epsilon = 1e-12),
                "Failed for test file: {:?}, index {}, expected: {}, got: {}",
                json_file.file_name().unwrap(),
                i,
                expected_val,
                actual_val
            );
        }
    }
}

#[test]
fn test_rng_uniform() {
    run_rng_uniform_tests();
}

#[test]
fn test_rng_uniform_int() {
    run_rng_uniform_int_tests();
}

#[test]
fn test_rng_string_seed() {
    run_rng_string_seed_tests();
}

#[test]
fn test_rng_uniform_range() {
    run_rng_uniform_range_tests();
}

#[test]
fn test_rng_uniform_f32() {
    run_rng_uniform_f32_tests();
}

#[test]
fn test_rng_uniform_i32() {
    run_rng_uniform_i32_tests();
}

#[test]
fn test_rng_uniform_bool() {
    run_rng_uniform_bool_tests();
}

#[test]
fn test_shuffle() {
    run_shuffle_tests();
}

#[test]
fn test_sample() {
    run_sample_tests();
}

#[test]
fn test_resample() {
    run_resample_tests();
}

#[test]
fn test_uniform_distribution() {
    run_uniform_distribution_tests();
}

#[test]
fn test_additive_distribution() {
    run_additive_distribution_tests();
}

#[test]
fn test_multiplic_distribution() {
    run_multiplic_distribution_tests();
}

#[test]
fn test_exp_distribution() {
    run_exp_distribution_tests();
}

#[test]
fn test_power_distribution() {
    run_power_distribution_tests();
}

// One-sample bounds tests

#[derive(Debug, Deserialize)]
struct SignedRankMarginInput {
    n: usize,
    misrate: f64,
}

#[derive(Debug, Deserialize)]
struct SignedRankMarginTestCase {
    input: SignedRankMarginInput,
    output: Option<usize>,
    expected_error: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct CenterBoundsInput {
    x: Vec<f64>,
    misrate: f64,
}

#[derive(Debug, Deserialize)]
struct CenterBoundsTestCase {
    input: CenterBoundsInput,
    output: Option<BoundsOutput>,
    expected_error: Option<serde_json::Value>,
}

fn run_signed_rank_margin_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("signed-rank-margin");

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
        let test_case: SignedRankMarginTestCase = serde_json::from_str(&content).unwrap();
        let file_name = json_file.file_name().unwrap();

        // Handle error test cases
        if let Some(ref expected_error) = test_case.expected_error {
            let result = signed_rank_margin(test_case.input.n, test_case.input.misrate);
            match result {
                Ok(_) => failures.push(format!("{file_name:?}: expected error, got Ok")),
                Err(err) => {
                    if let Some(expected_id) = expected_error.get("id").and_then(|v| v.as_str()) {
                        if err.violation().id.as_str() != expected_id {
                            failures.push(format!(
                                "{file_name:?}: expected violation id {expected_id}, got {}",
                                err.violation().id.as_str()
                            ));
                        }
                    }
                }
            }
            continue;
        }

        let actual_output = match signed_rank_margin(test_case.input.n, test_case.input.misrate) {
            Ok(val) => val,
            Err(e) => {
                failures.push(format!("{file_name:?}: unexpected error {e:?}"));
                continue;
            }
        };
        let expected_output = test_case.output.expect("Test case must have output");

        if actual_output != expected_output {
            failures.push(format!(
                "{file_name:?}: expected {expected_output}, got {actual_output}"
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "Failed tests:\n{}",
        failures.join("\n")
    );
}

fn run_center_bounds_tests() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("center-bounds");

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
        let test_case: CenterBoundsTestCase = serde_json::from_str(&content).unwrap();
        let file_name = json_file.file_name().unwrap();

        // Handle error test cases
        if let Some(ref expected_error) = test_case.expected_error {
            let result = center_bounds(&test_case.input.x, test_case.input.misrate);
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

        let actual_output = match center_bounds(&test_case.input.x, test_case.input.misrate) {
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
fn test_signed_rank_margin() {
    run_signed_rank_margin_tests();
}

#[test]
fn test_center_bounds() {
    run_center_bounds_tests();
}
