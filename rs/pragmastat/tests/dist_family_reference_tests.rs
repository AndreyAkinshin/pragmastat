//! The five parametric families against their shared `dist-*` fixtures.
//!
//! Each family answers three questions about itself, and each answer is assembled out of the
//! toolkit's own exponential, logarithm, normal cumulative and normal quantile. Nothing on these
//! paths reaches a platform transcendental, so the seven ports owe each other identical bits and
//! the suites are declared `exact` in `tests/manifest.json`: the comparison below is by binary64
//! payload. A tolerance would pass on exactly the divergence these suites exist to catch.
//!
//! The sibling `distributions` suite stays approximate, because drawing a sample still goes
//! through the platform's log, exp, cos and pow. That is a separate change with its own effect on
//! every seeded stream.

mod conformance;

use conformance::Conformance;
use pragmastat::Rng;
use pragmastat::distributions::{Additive, Distribution, Exp, Multiplic, Power, Uniform};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct FamilyInput {
    parameters: HashMap<String, f64>,
    x: Vec<f64>,
    p: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct FamilyOutput {
    density: Vec<f64>,
    cumulative: Vec<f64>,
    quantile: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct FamilyTestCase {
    input: FamilyInput,
    output: FamilyOutput,
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

fn parameter(parameters: &HashMap<String, f64>, name: &str, file: &Path) -> f64 {
    *parameters
        .get(name)
        .unwrap_or_else(|| panic!("{file:?}: no parameter named {name}"))
}

/// Builds the family a suite names, from the parameters the fixture carries.
///
/// The suite name decides the type rather than the parameter names deciding it: two families could
/// come to share a parameter spelling, and a fixture read into the wrong family would still
/// produce numbers and still be compared.
fn evaluate(suite: &str, case: &FamilyTestCase, file: &Path) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let parameters = &case.input.parameters;
    let distribution: Box<dyn Distribution> = match suite {
        "dist-uniform" => Box::new(Uniform::new(
            parameter(parameters, "min", file),
            parameter(parameters, "max", file),
        )),
        "dist-exp" => Box::new(Exp::new(parameter(parameters, "rate", file))),
        "dist-additive" => Box::new(Additive::new(
            parameter(parameters, "mean", file),
            parameter(parameters, "stdDev", file),
        )),
        "dist-multiplic" => Box::new(Multiplic::new(
            parameter(parameters, "logMean", file),
            parameter(parameters, "logStdDev", file),
        )),
        "dist-power" => Box::new(Power::new(
            parameter(parameters, "min", file),
            parameter(parameters, "shape", file),
        )),
        other => panic!("unknown suite {other}"),
    };

    let density = case
        .input
        .x
        .iter()
        .map(|&x| distribution.density_at(x))
        .collect();
    let cumulative = case
        .input
        .x
        .iter()
        .map(|&x| distribution.cumulative_at(x))
        .collect();
    let quantile = case
        .input
        .p
        .iter()
        .map(|&p| distribution.quantile_at(p))
        .collect();
    (density, cumulative, quantile)
}

fn check_suite(suite: &str) {
    let test_data_dir = find_repo_root().join("tests").join(suite);
    assert!(
        test_data_dir.exists(),
        "Test data directory not found: {test_data_dir:?}"
    );

    let mut json_files: Vec<_> = fs::read_dir(&test_data_dir)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.extension()?.to_str()? == "json" {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    json_files.sort();
    assert!(
        !json_files.is_empty(),
        "No JSON test files found in {test_data_dir:?}"
    );

    let mut checked = 0;
    let mut failures = Vec::new();

    for json_file in &json_files {
        let content = fs::read_to_string(json_file).unwrap();
        let case: FamilyTestCase = serde_json::from_str(&content).unwrap();
        let name = json_file.file_name().unwrap();

        // A file that parses into empty vectors passes every comparison below, which is the one
        // way this test could report success while checking nothing.
        assert!(!case.input.x.is_empty(), "{name:?}: no x values");
        assert!(!case.input.p.is_empty(), "{name:?}: no p values");
        assert_eq!(
            case.input.x.len(),
            case.output.density.len(),
            "{name:?}: {} x values against {} densities",
            case.input.x.len(),
            case.output.density.len()
        );
        assert_eq!(
            case.input.p.len(),
            case.output.quantile.len(),
            "{name:?}: {} p values against {} quantiles",
            case.input.p.len(),
            case.output.quantile.len()
        );

        let (density, cumulative, quantile) = evaluate(suite, &case, json_file);
        for (label, actual, expected) in [
            ("density", &density, &case.output.density),
            ("cumulative", &cumulative, &case.output.cumulative),
            ("quantile", &quantile, &case.output.quantile),
        ] {
            for (i, (&got, &want)) in actual.iter().zip(expected.iter()).enumerate() {
                checked += 1;
                if !Conformance::Exact.matches(got, want) {
                    failures.push(format!(
                        "{name:?} {label}[{i}]: expected {want:?} ({:#018x}), got {got:?} ({:#018x})",
                        want.to_bits(),
                        got.to_bits()
                    ));
                }
            }
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {checked} values differ in {suite}:\n{}",
        failures.len(),
        failures.join("\n")
    );
    assert!(checked > 0, "{suite}: no values compared");
}

#[test]
fn test_dist_uniform_reference() {
    check_suite("dist-uniform");
}

#[test]
fn test_dist_exp_reference() {
    check_suite("dist-exp");
}

#[test]
fn test_dist_additive_reference() {
    check_suite("dist-additive");
}

#[test]
fn test_dist_multiplic_reference() {
    check_suite("dist-multiplic");
}

#[test]
fn test_dist_power_reference() {
    check_suite("dist-power");
}

/// The arguments the fixtures cannot carry, because JSON writes neither a NaN nor an infinity.
///
/// Without this table the fixtures are the only check the trio has, and they sample the interior
/// of each support through the family's own quantile. That left two defects alive: Uniform
/// reported a perfectly ordinary density for NaN in six ports, since neither half of
/// `x < min || x >= max` holds for a NaN and control fell straight through, and R raised an error
/// from thirteen of the fifteen functions. Neither is reachable from a stored fixture.
fn families() -> Vec<(&'static str, Box<dyn Distribution>, f64, f64)> {
    vec![
        ("dist-uniform", Box::new(Uniform::new(0.0, 1.0)), 0.0, 1.0),
        ("dist-exp", Box::new(Exp::new(1.0)), 0.0, f64::INFINITY),
        (
            "dist-additive",
            Box::new(Additive::new(0.0, 1.0)),
            f64::NEG_INFINITY,
            f64::INFINITY,
        ),
        (
            "dist-multiplic",
            Box::new(Multiplic::new(0.0, 1.0)),
            0.0,
            f64::INFINITY,
        ),
        (
            "dist-power",
            Box::new(Power::new(1.0, 2.0)),
            1.0,
            f64::INFINITY,
        ),
    ]
}

#[test]
fn nan_propagates_through_all_three() {
    for (name, d, _, _) in families() {
        assert!(d.density_at(f64::NAN).is_nan(), "{name}: density_at(NaN)");
        assert!(
            d.cumulative_at(f64::NAN).is_nan(),
            "{name}: cumulative_at(NaN)"
        );
        assert!(d.quantile_at(f64::NAN).is_nan(), "{name}: quantile_at(NaN)");
    }
}

#[test]
fn the_infinities_lie_outside_every_support() {
    for (name, d, _, _) in families() {
        assert_eq!(d.density_at(f64::NEG_INFINITY), 0.0, "{name}");
        assert_eq!(d.density_at(f64::INFINITY), 0.0, "{name}");
        assert_eq!(d.cumulative_at(f64::NEG_INFINITY), 0.0, "{name}");
        assert_eq!(d.cumulative_at(f64::INFINITY), 1.0, "{name}");
    }
}

#[test]
fn a_probability_outside_the_unit_interval_returns_the_endpoints() {
    for (name, d, lower, upper) in families() {
        for p in [0.0, -0.1, f64::NEG_INFINITY] {
            assert_eq!(d.quantile_at(p), lower, "{name}: quantile_at({p})");
        }
        for p in [1.0, 1.1, f64::INFINITY] {
            assert_eq!(d.quantile_at(p), upper, "{name}: quantile_at({p})");
        }
    }
}

/// Every family draws by inverting its own quantile.
///
/// A fixture says two numbers agreed on the day it was written. This says they cannot disagree:
/// the same seed drives a draw and a direct quantile call, and the two must land on the same bits
/// for every family. It is what makes "a distribution is three functions" a statement about the
/// code rather than about the documentation, and it would fail the moment a family grew a second
/// sampling route.
#[test]
fn sampling_is_the_quantile_of_a_drawn_cumulance() {
    // lifts is true for the two families whose quantile_at(0) is not a sample.
    let families: Vec<(&str, Box<dyn Distribution>, bool)> = vec![
        ("Uniform", Box::new(Uniform::new(-2.5, 7.5)), false),
        ("Exp", Box::new(Exp::new(0.5)), false),
        ("Additive", Box::new(Additive::new(1.0, 2.0)), true),
        ("Multiplic", Box::new(Multiplic::new(-1.0, 0.5)), true),
        ("Power", Box::new(Power::new(2.0, 3.0)), false),
    ];

    for (name, d, lifts) in families {
        let mut drawing = Rng::from_string("identity-seed");
        let mut inverting = Rng::from_string("identity-seed");
        for i in 0..1000 {
            let drawn = d.sample(&mut drawing);
            // The same cumulance the draw consumed, taken from a generator on the same seed.
            let mut u = inverting.uniform_f64();
            if lifts && u == 0.0 {
                u = 5e-324;
            }
            let inverted = d.quantile_at(u);
            assert_eq!(
                drawn.to_bits(),
                inverted.to_bits(),
                "{name} draw {i}: sample gave {drawn}, quantile_at({u}) gave {inverted}"
            );
        }
    }
}
