//! Tests for the `one_sample_summary` and `one_sample_summary_with_seed` APIs.
//!
//! Verifies that batch results match individually-called estimators and
//! exercises error paths (small n, non-finite input, misrate out of range,
//! zero-spread data).  Also validates equivalence against JSON reference
//! fixtures used by all language implementations.

use pragmastat::assumptions::{AssumptionId, EstimatorError, Subject};
use pragmastat::estimators::raw;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

const TOL: f64 = 1e-9;

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < TOL
}

// ---------------------------------------------------------------------------
// Batch vs. individual consistency
// ---------------------------------------------------------------------------

#[test]
fn summary_matches_individual_estimators() {
    let x: Vec<f64> = (1..=30).map(|i| i as f64).collect();
    let misrate = 0.05;
    let seed = "test-seed";

    let summary = raw::one_sample_summary_with_seed(&x, misrate, seed).unwrap();

    let center_val = raw::center(&x).unwrap();
    let spread_val = raw::spread(&x).unwrap();
    let center_bounds = raw::center_bounds(&x, misrate).unwrap();
    let spread_bounds = raw::spread_bounds_with_seed(&x, misrate, seed).unwrap();

    assert!(
        approx_eq(summary.center, center_val),
        "center mismatch: {} vs {}",
        summary.center,
        center_val
    );
    assert!(
        approx_eq(summary.spread, spread_val),
        "spread mismatch: {} vs {}",
        summary.spread,
        spread_val
    );
    assert!(
        approx_eq(summary.center_bounds.lower, center_bounds.lower),
        "center_bounds.lower mismatch: {} vs {}",
        summary.center_bounds.lower,
        center_bounds.lower
    );
    assert!(
        approx_eq(summary.center_bounds.upper, center_bounds.upper),
        "center_bounds.upper mismatch: {} vs {}",
        summary.center_bounds.upper,
        center_bounds.upper
    );
    assert!(
        approx_eq(summary.spread_bounds.lower, spread_bounds.lower),
        "spread_bounds.lower mismatch: {} vs {}",
        summary.spread_bounds.lower,
        spread_bounds.lower
    );
    assert!(
        approx_eq(summary.spread_bounds.upper, spread_bounds.upper),
        "spread_bounds.upper mismatch: {} vs {}",
        summary.spread_bounds.upper,
        spread_bounds.upper
    );
}

#[test]
fn summary_matches_individual_larger_sample() {
    let x: Vec<f64> = (1..=20).map(|i| i as f64).collect();
    let misrate = 0.01;
    let seed = "larger-seed";

    let summary = raw::one_sample_summary_with_seed(&x, misrate, seed).unwrap();

    let center_val = raw::center(&x).unwrap();
    let spread_val = raw::spread(&x).unwrap();
    let center_bounds = raw::center_bounds(&x, misrate).unwrap();
    let spread_bounds = raw::spread_bounds_with_seed(&x, misrate, seed).unwrap();

    assert!(
        approx_eq(summary.center, center_val),
        "center mismatch: {} vs {}",
        summary.center,
        center_val
    );
    assert!(
        approx_eq(summary.spread, spread_val),
        "spread mismatch: {} vs {}",
        summary.spread,
        spread_val
    );
    assert!(
        approx_eq(summary.center_bounds.lower, center_bounds.lower),
        "center_bounds.lower mismatch"
    );
    assert!(
        approx_eq(summary.center_bounds.upper, center_bounds.upper),
        "center_bounds.upper mismatch"
    );
    assert!(
        approx_eq(summary.spread_bounds.lower, spread_bounds.lower),
        "spread_bounds.lower mismatch"
    );
    assert!(
        approx_eq(summary.spread_bounds.upper, spread_bounds.upper),
        "spread_bounds.upper mismatch"
    );
}

#[test]
fn summary_matches_individual_n3_minimum() {
    // n=3 is the smallest valid input for one_sample_summary
    let x = [1.0, 2.0, 4.0];
    let misrate = 1.0; // Use maximum misrate to avoid min_misrate issues

    let summary = raw::one_sample_summary_with_seed(&x, misrate, "n3-seed").unwrap();

    let center_val = raw::center(&x).unwrap();
    let spread_val = raw::spread(&x).unwrap();

    assert!(
        approx_eq(summary.center, center_val),
        "center mismatch at n=3: {} vs {}",
        summary.center,
        center_val
    );
    assert!(
        approx_eq(summary.spread, spread_val),
        "spread mismatch at n=3: {} vs {}",
        summary.spread,
        spread_val
    );
}

// ---------------------------------------------------------------------------
// Error paths: small n
// ---------------------------------------------------------------------------

#[test]
fn summary_empty_input() {
    let err = raw::one_sample_summary(&[], 0.05).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Validity)
    );
}

#[test]
fn summary_single_element() {
    let err = raw::one_sample_summary(&[1.0], 0.05).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Domain)
    );
}

#[test]
fn summary_n2_rejected() {
    // n=2 must be rejected because the presorted algorithms require n >= 3
    let err = raw::one_sample_summary(&[1.0, 2.0], 1.0).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Domain
            && ae.violation().subject == Subject::X),
        "n=2 should produce domain error for x, got: {err:?}"
    );
}

// ---------------------------------------------------------------------------
// Error paths: non-finite input
// ---------------------------------------------------------------------------

#[test]
fn summary_nan_input() {
    let err = raw::one_sample_summary(&[1.0, f64::NAN, 3.0], 0.05).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Validity)
    );
}

#[test]
fn summary_infinite_input() {
    let err = raw::one_sample_summary(&[1.0, f64::INFINITY, 3.0], 0.05).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Validity)
    );
}

// ---------------------------------------------------------------------------
// Error paths: misrate
// ---------------------------------------------------------------------------

#[test]
fn summary_nan_misrate() {
    let x = [1.0, 2.0, 3.0, 4.0, 5.0];
    let err = raw::one_sample_summary(&x, f64::NAN).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Domain
        && ae.violation().subject == Subject::Misrate)
    );
}

#[test]
fn summary_negative_misrate() {
    let x = [1.0, 2.0, 3.0, 4.0, 5.0];
    let err = raw::one_sample_summary(&x, -0.1).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Domain
        && ae.violation().subject == Subject::Misrate)
    );
}

#[test]
fn summary_misrate_too_large() {
    let x = [1.0, 2.0, 3.0, 4.0, 5.0];
    let err = raw::one_sample_summary(&x, 1.5).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Domain
        && ae.violation().subject == Subject::Misrate)
    );
}

#[test]
fn summary_misrate_below_minimum() {
    // n=3 has a relatively high minimum misrate; a very small misrate should fail
    let x = [1.0, 2.0, 3.0];
    let err = raw::one_sample_summary(&x, 1e-10).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Domain
        && ae.violation().subject == Subject::Misrate)
    );
}

// ---------------------------------------------------------------------------
// Error paths: sparity (zero-spread data)
// ---------------------------------------------------------------------------

#[test]
fn summary_constant_data() {
    let x = [5.0, 5.0, 5.0, 5.0, 5.0];
    let err = raw::one_sample_summary(&x, 1.0).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Sparity),
        "constant data should produce sparity error, got: {err:?}"
    );
}

// ---------------------------------------------------------------------------
// Seeded vs. unseeded API
// ---------------------------------------------------------------------------

#[test]
fn summary_with_seed_is_deterministic() {
    let x: Vec<f64> = (1..=30).map(|i| i as f64).collect();
    let misrate = 0.05;
    let seed = "determinism-check";

    let s1 = raw::one_sample_summary_with_seed(&x, misrate, seed).unwrap();
    let s2 = raw::one_sample_summary_with_seed(&x, misrate, seed).unwrap();

    assert_eq!(s1.center, s2.center);
    assert_eq!(s1.spread, s2.spread);
    assert_eq!(s1.center_bounds.lower, s2.center_bounds.lower);
    assert_eq!(s1.center_bounds.upper, s2.center_bounds.upper);
    assert_eq!(s1.spread_bounds.lower, s2.spread_bounds.lower);
    assert_eq!(s1.spread_bounds.upper, s2.spread_bounds.upper);
}

// ---------------------------------------------------------------------------
// Reference fixture validation — batch vs. individual on real fixture data
// ---------------------------------------------------------------------------

fn find_repo_root() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !dir.join("tests").join("center").exists() {
        dir = dir.parent().expect("repo root not found").to_path_buf();
    }
    dir
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

#[derive(Debug, Deserialize)]
struct SpreadBoundsInput {
    x: Vec<f64>,
    misrate: f64,
    seed: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SpreadBoundsTestCase {
    input: SpreadBoundsInput,
    output: Option<BoundsOutput>,
    expected_error: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct BoundsOutput {
    lower: f64,
    upper: f64,
}

/// For every center-bounds fixture that has a valid output, verify that
/// `one_sample_summary_with_seed` produces matching center, spread,
/// and center_bounds values.
#[test]
fn summary_matches_center_bounds_reference_fixtures() {
    let repo_root = find_repo_root();
    let test_dir = repo_root.join("tests").join("center-bounds");

    let mut tested = 0;
    let mut failures = Vec::new();

    for entry in fs::read_dir(&test_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let content = fs::read_to_string(&path).unwrap();
        let tc: CenterBoundsTestCase = serde_json::from_str(&content).unwrap();
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();

        // Skip error cases — those are tested separately
        if tc.expected_error.is_some() {
            continue;
        }

        let x = &tc.input.x;
        let misrate = tc.input.misrate;

        // one_sample_summary has stricter requirements (n >= 3 for presorted,
        // and misrate must be achievable for both center and spread bounds).
        // Skip fixtures that wouldn't be valid for the batch function.
        let summary = match raw::one_sample_summary_with_seed(x, misrate, "ref-fixture") {
            Ok(s) => s,
            Err(_) => continue,
        };

        let expected = tc.output.unwrap();

        // Verify center_bounds from batch matches the fixture
        if !approx_eq(summary.center_bounds.lower, expected.lower) {
            failures.push(format!(
                "{file_name}: center_bounds.lower: batch={}, fixture={}",
                summary.center_bounds.lower, expected.lower
            ));
        }
        if !approx_eq(summary.center_bounds.upper, expected.upper) {
            failures.push(format!(
                "{file_name}: center_bounds.upper: batch={}, fixture={}",
                summary.center_bounds.upper, expected.upper
            ));
        }

        // Also verify center and spread match individual calls
        let center_val = raw::center(x).unwrap();
        let spread_val = raw::spread(x).unwrap();
        if !approx_eq(summary.center, center_val) {
            failures.push(format!(
                "{file_name}: center: batch={}, individual={}",
                summary.center, center_val
            ));
        }
        if !approx_eq(summary.spread, spread_val) {
            failures.push(format!(
                "{file_name}: spread: batch={}, individual={}",
                summary.spread, spread_val
            ));
        }

        tested += 1;
    }

    assert!(
        tested > 0,
        "No center-bounds fixtures were valid for batch testing"
    );
    assert!(
        failures.is_empty(),
        "Failed on {}/{} fixtures:\n{}",
        failures.len(),
        tested,
        failures.join("\n")
    );
}

/// For every spread-bounds fixture with a seed and valid output, verify that
/// `one_sample_summary_with_seed` produces matching spread_bounds values.
#[test]
fn summary_matches_spread_bounds_reference_fixtures() {
    let repo_root = find_repo_root();
    let test_dir = repo_root.join("tests").join("spread-bounds");

    let mut tested = 0;
    let mut failures = Vec::new();

    for entry in fs::read_dir(&test_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let content = fs::read_to_string(&path).unwrap();
        let tc: SpreadBoundsTestCase = serde_json::from_str(&content).unwrap();
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();

        // Skip error cases and unseeded fixtures
        if tc.expected_error.is_some() {
            continue;
        }
        let seed = match tc.input.seed.as_deref() {
            Some(s) => s,
            None => continue,
        };

        let x = &tc.input.x;
        let misrate = tc.input.misrate;

        let summary = match raw::one_sample_summary_with_seed(x, misrate, seed) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let expected = tc.output.unwrap();

        if !approx_eq(summary.spread_bounds.lower, expected.lower) {
            failures.push(format!(
                "{file_name}: spread_bounds.lower: batch={}, fixture={}",
                summary.spread_bounds.lower, expected.lower
            ));
        }
        if !approx_eq(summary.spread_bounds.upper, expected.upper) {
            failures.push(format!(
                "{file_name}: spread_bounds.upper: batch={}, fixture={}",
                summary.spread_bounds.upper, expected.upper
            ));
        }

        tested += 1;
    }

    assert!(
        tested > 0,
        "No spread-bounds fixtures were valid for batch testing"
    );
    assert!(
        failures.is_empty(),
        "Failed on {}/{} fixtures:\n{}",
        failures.len(),
        tested,
        failures.join("\n")
    );
}

// ---------------------------------------------------------------------------
// Additional edge cases
// ---------------------------------------------------------------------------

#[test]
fn summary_unsorted_input_matches_individual() {
    // Deliberately unsorted to verify the batch function sorts correctly
    let x = [9.5, 2.1, 7.3, 1.0, 5.5, 8.8, 3.2, 6.7, 4.4, 10.0];
    let misrate = 0.1;
    let seed = "unsorted-test";

    let summary = raw::one_sample_summary_with_seed(&x, misrate, seed).unwrap();

    let center_val = raw::center(&x).unwrap();
    let spread_val = raw::spread(&x).unwrap();
    let center_bounds = raw::center_bounds(&x, misrate).unwrap();
    let spread_bounds = raw::spread_bounds_with_seed(&x, misrate, seed).unwrap();

    assert!(
        approx_eq(summary.center, center_val),
        "unsorted center: {} vs {}",
        summary.center,
        center_val
    );
    assert!(
        approx_eq(summary.spread, spread_val),
        "unsorted spread: {} vs {}",
        summary.spread,
        spread_val
    );
    assert!(approx_eq(summary.center_bounds.lower, center_bounds.lower));
    assert!(approx_eq(summary.center_bounds.upper, center_bounds.upper));
    assert!(approx_eq(summary.spread_bounds.lower, spread_bounds.lower));
    assert!(approx_eq(summary.spread_bounds.upper, spread_bounds.upper));
}

#[test]
fn summary_negative_values() {
    let x = [-10.0, -7.5, -3.2, -1.0, 0.5, 2.0, 4.8, 8.1, 11.0, 15.0];
    let misrate = 0.1;
    let seed = "neg-test";

    let summary = raw::one_sample_summary_with_seed(&x, misrate, seed).unwrap();

    let center_val = raw::center(&x).unwrap();
    let spread_val = raw::spread(&x).unwrap();
    let center_bounds = raw::center_bounds(&x, misrate).unwrap();
    let spread_bounds = raw::spread_bounds_with_seed(&x, misrate, seed).unwrap();

    assert!(approx_eq(summary.center, center_val));
    assert!(approx_eq(summary.spread, spread_val));
    assert!(approx_eq(summary.center_bounds.lower, center_bounds.lower));
    assert!(approx_eq(summary.center_bounds.upper, center_bounds.upper));
    assert!(approx_eq(summary.spread_bounds.lower, spread_bounds.lower));
    assert!(approx_eq(summary.spread_bounds.upper, spread_bounds.upper));
}

#[test]
fn summary_extreme_misrate_boundary() {
    // misrate = 1.0 is the maximum valid value; should succeed
    let x: Vec<f64> = (1..=10).map(|i| i as f64).collect();
    let seed = "extreme-misrate";

    let summary = raw::one_sample_summary_with_seed(&x, 1.0, seed).unwrap();
    let center_val = raw::center(&x).unwrap();
    let spread_val = raw::spread(&x).unwrap();

    assert!(approx_eq(summary.center, center_val));
    assert!(approx_eq(summary.spread, spread_val));
}

#[test]
fn summary_misrate_zero() {
    // misrate = 0.0 should fail (below minimum achievable)
    let x: Vec<f64> = (1..=10).map(|i| i as f64).collect();
    let err = raw::one_sample_summary(&x, 0.0).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Domain
            && ae.violation().subject == Subject::Misrate),
        "misrate=0.0 should produce domain error for misrate, got: {err:?}"
    );
}

#[test]
fn summary_large_sample() {
    // n=100 to exercise algorithm with a non-trivial sample size
    let x: Vec<f64> = (1..=100).map(|i| i as f64 * 0.7).collect();
    let misrate = 0.01;
    let seed = "large-sample";

    let summary = raw::one_sample_summary_with_seed(&x, misrate, seed).unwrap();

    let center_val = raw::center(&x).unwrap();
    let spread_val = raw::spread(&x).unwrap();
    let center_bounds = raw::center_bounds(&x, misrate).unwrap();
    let spread_bounds = raw::spread_bounds_with_seed(&x, misrate, seed).unwrap();

    assert!(approx_eq(summary.center, center_val));
    assert!(approx_eq(summary.spread, spread_val));
    assert!(approx_eq(summary.center_bounds.lower, center_bounds.lower));
    assert!(approx_eq(summary.center_bounds.upper, center_bounds.upper));
    assert!(approx_eq(summary.spread_bounds.lower, spread_bounds.lower));
    assert!(approx_eq(summary.spread_bounds.upper, spread_bounds.upper));
}

#[test]
fn summary_duplicates_non_constant() {
    // Has duplicates but is not all-equal, so spread > 0
    let x = [1.0, 1.0, 2.0, 2.0, 3.0, 3.0, 4.0, 4.0, 5.0, 5.0];
    let misrate = 0.1;
    let seed = "dup-test";

    let summary = raw::one_sample_summary_with_seed(&x, misrate, seed).unwrap();
    let center_val = raw::center(&x).unwrap();
    let spread_val = raw::spread(&x).unwrap();

    assert!(approx_eq(summary.center, center_val));
    assert!(approx_eq(summary.spread, spread_val));
    assert!(
        summary.spread > 0.0,
        "spread should be positive for non-constant data with duplicates"
    );
}

// ---------------------------------------------------------------------------
// Sample-level API
// ---------------------------------------------------------------------------

#[test]
fn sample_level_summary_matches_raw() {
    use pragmastat::Sample;

    let values: Vec<f64> = (1..=20).map(|i| i as f64).collect();
    let misrate = 0.05;
    let seed = "sample-level-test";

    let sample = Sample::new(values.clone()).unwrap();
    let sample_result = pragmastat::one_sample_summary_with_seed(&sample, misrate, seed).unwrap();
    let raw_result = raw::one_sample_summary_with_seed(&values, misrate, seed).unwrap();

    assert!(approx_eq(sample_result.center.value, raw_result.center));
    assert!(approx_eq(sample_result.spread.value, raw_result.spread));
    assert!(approx_eq(
        sample_result.center_bounds.lower,
        raw_result.center_bounds.lower
    ));
    assert!(approx_eq(
        sample_result.center_bounds.upper,
        raw_result.center_bounds.upper
    ));
    assert!(approx_eq(
        sample_result.spread_bounds.lower,
        raw_result.spread_bounds.lower
    ));
    assert!(approx_eq(
        sample_result.spread_bounds.upper,
        raw_result.spread_bounds.upper
    ));
}

// ---------------------------------------------------------------------------
// Benchmark: batch vs. individual calls (runs as a test, prints timings)
// ---------------------------------------------------------------------------

#[test]
fn bench_batch_vs_individual() {
    use std::time::Instant;

    let seed = "bench-seed";
    let misrate = 0.01;

    println!();
    println!(
        "  {:>6}  {:>12}  {:>12}  {:>8}",
        "n", "individual", "batch", "speedup"
    );
    println!("  {:->6}  {:->12}  {:->12}  {:->8}", "", "", "", "");

    for &n in &[100, 1_000, 10_000] {
        let x: Vec<f64> = (0..n).map(|i| (i as f64) * 0.31 + 1.0).collect();
        let iters = if n <= 1_000 { 50 } else { 10 };

        // Warm up
        for _ in 0..3 {
            let _ = raw::one_sample_summary_with_seed(&x, misrate, seed).unwrap();
            let _ = raw::center(&x).unwrap();
        }

        // Bench individual calls
        let start = Instant::now();
        for _ in 0..iters {
            let _ = raw::center(&x).unwrap();
            let _ = raw::spread(&x).unwrap();
            let _ = raw::center_bounds(&x, misrate).unwrap();
            let _ = raw::spread_bounds_with_seed(&x, misrate, seed).unwrap();
        }
        let individual_us = start.elapsed().as_micros() / iters as u128;

        // Bench batch
        let start = Instant::now();
        for _ in 0..iters {
            let _ = raw::one_sample_summary_with_seed(&x, misrate, seed).unwrap();
        }
        let batch_us = start.elapsed().as_micros() / iters as u128;

        let speedup = individual_us as f64 / batch_us as f64;

        println!("  n={n:>5}: {individual_us:>8}us    {batch_us:>8}us    {speedup:.2}x");
    }
}
