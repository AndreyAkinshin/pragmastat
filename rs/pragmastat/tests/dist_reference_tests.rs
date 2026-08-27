//! `LinearDistModel` and `NeighborDistModel` against their shared `dist-linear`/`dist-neighbor`
//! fixtures.
//!
//! Unlike the five parametric families, this suite's oracle is not one of the seven ports: it is
//! an independent binary64 + exact-Fraction reimplementation (`tests/oracles/dist_reference.py`,
//! see `tests/manifest.json`), written directly from the normative algorithm so a bug shared by
//! every language cannot pass itself. Every case is compared by binary64 payload, matching the
//! suites' declared `exact` class.
//!
//! Both models are reached through the raw vector path only (`::new`, never `::from_sample`):
//! the measured, `Sample`-wrapped surface has its own tests and is out of scope here.

mod conformance;

use conformance::Conformance;
use pragmastat::{
    Distribution, EstimatorError, LinearDistModel, MeasuredModel, NeighborDistModel, Rng,
};
use serde::Deserialize;
use serde_json::Value;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

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

fn json_files_in(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<_> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("{dir:?}: {e}"))
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.extension()?.to_str()? == "json" {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    files.sort();
    assert!(!files.is_empty(), "No JSON test files found in {dir:?}");
    files
}

/// Reads a fixture float back from JSON. JSON has no literal for a NaN or an infinity, so a
/// handful of fixtures spell them as the strings `"NaN"`, `"Infinity"` and `"-Infinity"` in
/// place of a number.
fn parse_special_float(value: &Value) -> f64 {
    match value {
        Value::Number(n) => n.as_f64().unwrap_or_else(|| panic!("not a binary64: {n}")),
        Value::String(s) => match s.as_str() {
            "NaN" => f64::NAN,
            "Infinity" => f64::INFINITY,
            "-Infinity" => f64::NEG_INFINITY,
            other => panic!("unexpected float string {other:?}"),
        },
        other => panic!("unexpected JSON value for a float: {other:?}"),
    }
}

fn parse_special_floats(values: &[Value]) -> Vec<f64> {
    values.iter().map(parse_special_float).collect()
}

#[derive(Debug, Deserialize)]
struct ExpectedError {
    id: String,
    subject: String,
}

/// Input shape shared by the quantile/cumulative/density sub-suites. `x` and `y` arrive as raw
/// JSON values, not `Vec<f64>`, for the NaN/Infinity reason [`parse_special_float`] documents.
#[derive(Debug, Deserialize)]
struct DistQueryInput {
    x: Vec<Value>,
    y: Vec<Value>,
    #[serde(default)]
    assume_sorted: bool,
}

#[derive(Debug, Deserialize)]
struct DistQueryTestCase {
    input: DistQueryInput,
    #[serde(default)]
    output: Vec<Value>,
    expected_error: Option<ExpectedError>,
}

#[derive(Debug, Deserialize)]
struct DistSampleInput {
    x: Vec<Value>,
    seed: String,
    count: usize,
}

/// `output` carries raw JSON values for the same reason [`DistQueryInput`]'s `x`/`y` do: a
/// drawn cumulance can land on a value that overflows to Infinity, the same kernel the
/// quantile suite's `extreme-overflow` fixtures already exercise.
#[derive(Debug, Deserialize)]
struct DistSampleTestCase {
    input: DistSampleInput,
    output: Vec<Value>,
}

#[derive(Debug, Deserialize)]
struct DistLandscapeInput {
    x: Vec<Value>,
}

/// The expected staircase: k0, radius, cell boundaries, cumulative values, masses, heights and
/// source indices. The float fields carry raw JSON values for the same reason
/// [`DistQueryInput`]'s do: a subulp density can be an Infinity (see `extreme-overflow-3.json`).
#[derive(Debug, Deserialize)]
struct DistLandscapeOutput {
    k0: usize,
    radius: usize,
    boundaries: Vec<Value>,
    cumulative: Vec<Value>,
    masses: Vec<Value>,
    densities: Vec<Value>,
    sources: Vec<usize>,
}

#[derive(Debug, Deserialize)]
struct DistLandscapeTestCase {
    input: DistLandscapeInput,
    output: DistLandscapeOutput,
}

/// Builds the model a suite names, from a raw vector -- never through `Sample`. Both
/// constructors run `validity(x)` before this returns, and the two remaining assumptions
/// (`domain`, `distinctness`) inside it, so a build error here is always one of those three.
fn build_query_model(
    model: &str,
    x: &[f64],
    assume_sorted: bool,
) -> Result<Box<dyn MeasuredModel>, EstimatorError> {
    match model {
        "linear" => Ok(Box::new(LinearDistModel::new(x, assume_sorted)?)),
        "neighbor" => Ok(Box::new(NeighborDistModel::new(x, assume_sorted)?)),
        other => panic!("unknown dist model {other}"),
    }
}

/// The `Distribution` counterpart of [`build_query_model`], for the sample sub-suite.
fn build_distribution(
    model: &str,
    x: &[f64],
    assume_sorted: bool,
) -> Result<Box<dyn Distribution>, EstimatorError> {
    match model {
        "linear" => Ok(Box::new(LinearDistModel::new(x, assume_sorted)?)),
        "neighbor" => Ok(Box::new(NeighborDistModel::new(x, assume_sorted)?)),
        other => panic!("unknown dist model {other}"),
    }
}

/// Dispatches to the operation the fixture directory names.
fn run_dist_op(model: &dyn MeasuredModel, op: &str, y: &[f64]) -> Result<Vec<f64>, EstimatorError> {
    match op {
        "quantile" => model.quantile(y),
        "cumulative" => model.cumulative(y),
        "density" => model.density(y),
        other => panic!("unknown dist operation {other}"),
    }
}

/// Checks a raised error against a fixture's `expected_error`. Callers only reach this after
/// deciding, from the fixture alone, that an error is what should have happened; it never runs
/// as a follow-up to an `Err` that happened to come out.
fn assert_matches_expected_error(
    failures: &mut Vec<String>,
    file_name: &OsStr,
    err: &EstimatorError,
    expected: &ExpectedError,
) {
    match err {
        EstimatorError::Assumption(ae) => {
            let violation = ae.violation();
            if violation.id.as_str() != expected.id {
                failures.push(format!(
                    "{file_name:?}: expected violation id \"{}\", got \"{}\"",
                    expected.id,
                    violation.id.as_str()
                ));
            }
            if violation.subject.as_str() != expected.subject {
                failures.push(format!(
                    "{file_name:?}: expected violation subject \"{}\", got \"{}\"",
                    expected.subject,
                    violation.subject.as_str()
                ));
            }
        }
        other => failures.push(format!(
            "{file_name:?}: expected AssumptionError, got {other:?}"
        )),
    }
}

/// Runs every fixture under `dist-<model>/<op>`.
///
/// A case that declares `expected_error` is checked *because* it declares one, before any call
/// runs: the model is built first (a build failure is checked and the case returns), and only a
/// model that built successfully has the operation run on it (a query failure is checked there
/// instead). A case with no `expected_error` field is required to succeed; an `Err` there is a
/// bug and fails the test outright rather than being matched against a field that is not there.
fn check_query_suite(model: &str, op: &str) {
    let test_data_dir = find_repo_root()
        .join("tests")
        .join(format!("dist-{model}"))
        .join(op);
    let json_files = json_files_in(&test_data_dir);
    let mut failures = Vec::new();

    for json_file in &json_files {
        let content = fs::read_to_string(json_file).unwrap();
        let case: DistQueryTestCase = serde_json::from_str(&content).unwrap();
        let name = json_file.file_name().unwrap();

        let x = parse_special_floats(&case.input.x);
        let y = parse_special_floats(&case.input.y);

        if let Some(expected_error) = &case.expected_error {
            match build_query_model(model, &x, case.input.assume_sorted) {
                Err(build_err) => {
                    assert_matches_expected_error(&mut failures, name, &build_err, expected_error);
                }
                Ok(built) => match run_dist_op(built.as_ref(), op, &y) {
                    Err(query_err) => {
                        assert_matches_expected_error(
                            &mut failures,
                            name,
                            &query_err,
                            expected_error,
                        );
                    }
                    Ok(_) => failures.push(format!("{name:?}: expected an error, got Ok")),
                },
            }
            continue;
        }

        let built = build_query_model(model, &x, case.input.assume_sorted)
            .unwrap_or_else(|e| panic!("{name:?}: unexpected build error: {e:?}"));
        let actual = run_dist_op(built.as_ref(), op, &y)
            .unwrap_or_else(|e| panic!("{name:?}: unexpected error: {e:?}"));
        let expected = parse_special_floats(&case.output);

        assert_eq!(
            actual.len(),
            expected.len(),
            "{name:?}: {} values expected, got {}",
            expected.len(),
            actual.len()
        );
        for (i, (&got, &want)) in actual.iter().zip(expected.iter()).enumerate() {
            if !Conformance::Exact.matches(got, want) {
                failures.push(format!(
                    "{name:?} [{i}]: expected {want:?} ({:#018x}), got {got:?} ({:#018x})",
                    want.to_bits(),
                    got.to_bits()
                ));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {} files differ in dist-{model}/{op}:\n{}",
        failures.len(),
        json_files.len(),
        failures.join("\n")
    );
}

/// Runs every fixture under `dist-<model>/sample`. No case here declares an error: an invalid
/// `x` is already covered by the query suites above, and drawing from a built model cannot fail
/// on its own.
fn check_sample_suite(model: &str) {
    let test_data_dir = find_repo_root()
        .join("tests")
        .join(format!("dist-{model}"))
        .join("sample");
    let json_files = json_files_in(&test_data_dir);
    let mut failures = Vec::new();

    for json_file in &json_files {
        let content = fs::read_to_string(json_file).unwrap();
        let case: DistSampleTestCase = serde_json::from_str(&content).unwrap();
        let name = json_file.file_name().unwrap();

        let x = parse_special_floats(&case.input.x);
        let built = build_distribution(model, &x, false)
            .unwrap_or_else(|e| panic!("{name:?}: unexpected build error: {e:?}"));

        let mut rng = Rng::from_string(&case.input.seed);
        let actual: Vec<f64> = (0..case.input.count)
            .map(|_| built.sample(&mut rng))
            .collect();

        let expected = parse_special_floats(&case.output);
        assert_eq!(
            actual.len(),
            expected.len(),
            "{name:?}: {} values expected, got {}",
            expected.len(),
            actual.len()
        );
        for (i, (&got, &want)) in actual.iter().zip(expected.iter()).enumerate() {
            if !Conformance::Exact.matches(got, want) {
                failures.push(format!(
                    "{name:?} [{i}]: expected {want:?} ({:#018x}), got {got:?} ({:#018x})",
                    want.to_bits(),
                    got.to_bits()
                ));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {} files differ in dist-{model}/sample:\n{}",
        failures.len(),
        json_files.len(),
        failures.join("\n")
    );
}

/// Runs every fixture under `dist-neighbor/landscape`. Every case here builds successfully by
/// construction: the generator only emits a landscape for a sample that already passed the
/// query suites' validity checks.
fn check_neighbor_landscape_suite() {
    let test_data_dir = find_repo_root()
        .join("tests")
        .join("dist-neighbor")
        .join("landscape");
    let json_files = json_files_in(&test_data_dir);
    let mut failures = Vec::new();

    for json_file in &json_files {
        let content = fs::read_to_string(json_file).unwrap();
        let case: DistLandscapeTestCase = serde_json::from_str(&content).unwrap();
        let name = json_file.file_name().unwrap();

        let x = parse_special_floats(&case.input.x);
        let built = NeighborDistModel::new(&x, false)
            .unwrap_or_else(|e| panic!("{name:?}: unexpected build error: {e:?}"));
        let landscape = built.get_landscape();

        if landscape.scale_index() != case.output.k0 {
            failures.push(format!(
                "{name:?}: k0 = {}, want {}",
                landscape.scale_index(),
                case.output.k0
            ));
        }
        if landscape.radius() != case.output.radius {
            failures.push(format!(
                "{name:?}: radius = {}, want {}",
                landscape.radius(),
                case.output.radius
            ));
        }

        for (label, actual, expected_raw) in [
            (
                "boundaries",
                landscape.boundaries(),
                &case.output.boundaries,
            ),
            (
                "cumulative",
                landscape.cumulative(),
                &case.output.cumulative,
            ),
            ("masses", landscape.masses(), &case.output.masses),
            ("densities", landscape.densities(), &case.output.densities),
        ] {
            let expected = parse_special_floats(expected_raw);
            if actual.len() != expected.len() {
                failures.push(format!(
                    "{name:?} {label}: {} values expected, got {}",
                    expected.len(),
                    actual.len()
                ));
                continue;
            }
            for (i, (&got, &want)) in actual.iter().zip(expected.iter()).enumerate() {
                if !Conformance::Exact.matches(got, want) {
                    failures.push(format!(
                        "{name:?} {label}[{i}]: expected {want:?} ({:#018x}), got {got:?} ({:#018x})",
                        want.to_bits(),
                        got.to_bits()
                    ));
                }
            }
        }

        let actual_sources = landscape.sources();
        if actual_sources.len() != case.output.sources.len() {
            failures.push(format!(
                "{name:?} sources: {} values expected, got {}",
                case.output.sources.len(),
                actual_sources.len()
            ));
        } else {
            for (i, (&got, &want)) in actual_sources
                .iter()
                .zip(case.output.sources.iter())
                .enumerate()
            {
                if got != want {
                    failures.push(format!("{name:?} sources[{i}]: expected {want}, got {got}"));
                }
            }
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {} files differ in dist-neighbor/landscape:\n{}",
        failures.len(),
        json_files.len(),
        failures.join("\n")
    );
}

#[test]
fn test_dist_linear_quantile_reference() {
    check_query_suite("linear", "quantile");
}

#[test]
fn test_dist_linear_cumulative_reference() {
    check_query_suite("linear", "cumulative");
}

#[test]
fn test_dist_linear_density_reference() {
    check_query_suite("linear", "density");
}

#[test]
fn test_dist_linear_sample_reference() {
    check_sample_suite("linear");
}

#[test]
fn test_dist_neighbor_quantile_reference() {
    check_query_suite("neighbor", "quantile");
}

#[test]
fn test_dist_neighbor_cumulative_reference() {
    check_query_suite("neighbor", "cumulative");
}

#[test]
fn test_dist_neighbor_density_reference() {
    check_query_suite("neighbor", "density");
}

#[test]
fn test_dist_neighbor_sample_reference() {
    check_sample_suite("neighbor");
}

#[test]
fn test_dist_neighbor_landscape_reference() {
    check_neighbor_landscape_suite();
}
