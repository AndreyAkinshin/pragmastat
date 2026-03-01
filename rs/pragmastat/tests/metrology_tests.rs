use float_cmp::approx_eq;
use pragmastat::*;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

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

/// Parse a JSON value that may be a number or a special float string ("NaN", "Infinity", "-Infinity").
fn parse_float(v: &Value) -> f64 {
    match v {
        Value::Number(n) => n.as_f64().unwrap(),
        Value::String(s) => match s.as_str() {
            "NaN" => f64::NAN,
            "Infinity" => f64::INFINITY,
            "-Infinity" => f64::NEG_INFINITY,
            other => panic!("unexpected string value: {other}"),
        },
        other => panic!("unexpected JSON value type: {other}"),
    }
}

fn parse_float_vec(arr: &[Value]) -> Vec<f64> {
    arr.iter().map(parse_float).collect()
}

// =============================================================================
// Sample construction tests
// =============================================================================

#[test]
fn test_sample_construction() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("sample-construction");

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
        let raw: Value = serde_json::from_str(&content).unwrap();
        let file_name = json_file.file_name().unwrap();

        let input = &raw["input"];
        let values_arr = input["values"].as_array().unwrap();
        let values = parse_float_vec(values_arr);
        let weights: Option<Vec<f64>> = input.get("weights").and_then(|w| {
            w.as_array()
                .map(|arr| arr.iter().map(|v| v.as_f64().unwrap()).collect())
        });

        if raw.get("expected_error").is_some() {
            let result = if let Some(ref w) = weights {
                Sample::weighted(values, w.clone(), MeasurementUnit::number())
            } else {
                Sample::new(values)
            };
            if result.is_ok() {
                failures.push(format!("{file_name:?}: expected error but got Ok"));
            }
            continue;
        }

        let output = &raw["output"];
        let expected_size = output["size"].as_u64().unwrap() as usize;
        let expected_is_weighted = output["is_weighted"].as_bool().unwrap();

        let result = if let Some(ref w) = weights {
            Sample::weighted(values, w.clone(), MeasurementUnit::number())
        } else {
            Sample::new(values)
        };

        match result {
            Ok(s) => {
                if s.size() != expected_size {
                    failures.push(format!(
                        "{file_name:?}: size = {}, want {expected_size}",
                        s.size()
                    ));
                }
                if s.is_weighted() != expected_is_weighted {
                    failures.push(format!(
                        "{file_name:?}: is_weighted = {}, want {expected_is_weighted}",
                        s.is_weighted()
                    ));
                }
            }
            Err(e) => {
                failures.push(format!("{file_name:?}: unexpected error {e:?}"));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "Failed tests:\n{}",
        failures.join("\n")
    );
}

// =============================================================================
// Unit propagation tests
// =============================================================================

#[test]
fn test_unit_propagation() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("unit-propagation");
    let registry = UnitRegistry::standard();

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
        let raw: Value = serde_json::from_str(&content).unwrap();
        let file_name = json_file.file_name().unwrap();
        let input = &raw["input"];

        // Handle expected_error case (weighted-rejected)
        if raw.get("expected_error").is_some() {
            let estimator = input["estimator"].as_str().unwrap();
            let x_values: Vec<f64> = input["x"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap())
                .collect();
            let x_weights: Vec<f64> = input["x_weights"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap())
                .collect();
            let sx = Sample::weighted(x_values, x_weights, MeasurementUnit::number());
            match sx {
                Ok(sx) => {
                    let result = match estimator {
                        "center" => center(&sx),
                        other => {
                            failures.push(format!(
                                "{file_name:?}: unknown estimator for error case: {other}"
                            ));
                            continue;
                        }
                    };
                    if result.is_ok() {
                        failures.push(format!(
                            "{file_name:?}: expected error for weighted sample, got Ok"
                        ));
                    }
                }
                Err(_) => {
                    // Construction error counts as expected error
                }
            }
            continue;
        }

        let estimator = input["estimator"].as_str().unwrap();
        let x_values: Vec<f64> = input["x"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64().unwrap())
            .collect();
        let x_unit_id = input["x_unit"].as_str().unwrap();
        let x_unit = registry.resolve(x_unit_id).unwrap();

        let sx = Sample::with_unit(x_values, x_unit.clone()).unwrap();

        let output = &raw["output"];
        let expected_unit = output["unit"].as_str().unwrap();
        let expected_value = output.get("value").and_then(|v| v.as_f64());

        match estimator {
            "center" => match center(&sx) {
                Ok(m) => {
                    if m.unit.id() != expected_unit {
                        failures.push(format!(
                            "{file_name:?}: unit = {:?}, want {expected_unit:?}",
                            m.unit.id()
                        ));
                    }
                    if let Some(ev) = expected_value {
                        if !approx_eq!(f64, m.value, ev, epsilon = 1e-9) {
                            failures.push(format!("{file_name:?}: value = {}, want {ev}", m.value));
                        }
                    }
                }
                Err(e) => failures.push(format!("{file_name:?}: center error: {e:?}")),
            },
            "spread" => match spread(&sx) {
                Ok(m) => {
                    if m.unit.id() != expected_unit {
                        failures.push(format!(
                            "{file_name:?}: unit = {:?}, want {expected_unit:?}",
                            m.unit.id()
                        ));
                    }
                }
                Err(e) => failures.push(format!("{file_name:?}: spread error: {e:?}")),
            },
            "shift" => {
                let y_values: Vec<f64> = input["y"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_f64().unwrap())
                    .collect();
                let y_unit_id = input["y_unit"].as_str().unwrap();
                let y_unit = registry.resolve(y_unit_id).unwrap();
                let sy = Sample::with_unit(y_values, y_unit.clone()).unwrap();
                match shift(&sx, &sy) {
                    Ok(m) => {
                        if m.unit.id() != expected_unit {
                            failures.push(format!(
                                "{file_name:?}: unit = {:?}, want {expected_unit:?}",
                                m.unit.id()
                            ));
                        }
                    }
                    Err(e) => failures.push(format!("{file_name:?}: shift error: {e:?}")),
                }
            }
            "ratio" => {
                let y_values: Vec<f64> = input["y"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_f64().unwrap())
                    .collect();
                let y_unit_id = input["y_unit"].as_str().unwrap();
                let y_unit = registry.resolve(y_unit_id).unwrap();
                let sy = Sample::with_unit(y_values, y_unit.clone()).unwrap();
                match ratio(&sx, &sy) {
                    Ok(m) => {
                        if m.unit.id() != expected_unit {
                            failures.push(format!(
                                "{file_name:?}: unit = {:?}, want {expected_unit:?}",
                                m.unit.id()
                            ));
                        }
                    }
                    Err(e) => failures.push(format!("{file_name:?}: ratio error: {e:?}")),
                }
            }
            "disparity" => {
                let y_values: Vec<f64> = input["y"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_f64().unwrap())
                    .collect();
                let y_unit_id = input["y_unit"].as_str().unwrap();
                let y_unit = registry.resolve(y_unit_id).unwrap();
                let sy = Sample::with_unit(y_values, y_unit.clone()).unwrap();
                match disparity(&sx, &sy) {
                    Ok(m) => {
                        if m.unit.id() != expected_unit {
                            failures.push(format!(
                                "{file_name:?}: unit = {:?}, want {expected_unit:?}",
                                m.unit.id()
                            ));
                        }
                    }
                    Err(e) => failures.push(format!("{file_name:?}: disparity error: {e:?}")),
                }
            }
            other => {
                failures.push(format!("{file_name:?}: unknown estimator: {other}"));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "Failed tests:\n{}",
        failures.join("\n")
    );
}
