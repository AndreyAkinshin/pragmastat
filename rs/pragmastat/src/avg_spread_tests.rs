use crate::estimators::avg_spread;
use float_cmp::approx_eq;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct TwoSampleInput {
    x: Vec<f64>,
    y: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct TwoSampleTestCase {
    input: TwoSampleInput,
    output: f64,
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
fn test_avg_spread_reference() {
    let repo_root = find_repo_root();
    let test_data_dir = repo_root.join("tests").join("avg-spread");

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

        let actual_output = match avg_spread(&test_case.input.x, &test_case.input.y) {
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

    assert!(
        executed_count > 0,
        "No test cases were executed out of {} files",
        total_count
    );

    assert!(
        failures.is_empty(),
        "Failed {} out of {} tests:\n{}",
        failures.len(),
        total_count,
        failures.join("\n")
    );
}

#[test]
fn avg_spread_empty_x() {
    assert!(avg_spread(&[], &[1.0, 2.0]).is_err());
}

#[test]
fn avg_spread_empty_y() {
    assert!(avg_spread(&[1.0, 2.0], &[]).is_err());
}

#[test]
fn avg_spread_equal() {
    use crate::estimators::spread;
    let samples: Vec<Vec<f64>> = vec![
        vec![1.0, 2.0, 3.0, 4.0, 5.0],
        vec![10.0, 20.0, 30.0],
        vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0],
    ];
    for x in &samples {
        let as_val = avg_spread(x, x).unwrap();
        let s_val = spread(x).unwrap();
        assert!(
            approx_eq!(f64, as_val, s_val, epsilon = 1e-9),
            "avg_spread(x, x) = {} != spread(x) = {}",
            as_val,
            s_val
        );
    }
}

#[test]
fn avg_spread_symmetry() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![10.0, 20.0, 30.0];
    let xy = avg_spread(&x, &y).unwrap();
    let yx = avg_spread(&y, &x).unwrap();
    assert!(
        approx_eq!(f64, xy, yx, epsilon = 1e-9),
        "avg_spread(x, y) = {} != avg_spread(y, x) = {}",
        xy,
        yx
    );
}

#[test]
fn avg_spread_average() {
    use crate::estimators::spread;
    use crate::rng::Rng;
    let mut rng = Rng::from_seed(1729);
    for n in 2..=10 {
        let x: Vec<f64> = (0..n).map(|_| rng.uniform_f64()).collect();
        let x5: Vec<f64> = x.iter().map(|&v| v * 5.0).collect();
        let as_val = avg_spread(&x, &x5).unwrap();
        let expected = 3.0 * spread(&x).unwrap();
        assert!(
            approx_eq!(f64, as_val, expected, epsilon = 1e-9),
            "n={}: avg_spread(x, 5*x) = {} != 3*spread(x) = {}",
            n,
            as_val,
            expected
        );
    }
}

#[test]
fn avg_spread_scale() {
    use crate::rng::Rng;
    let mut rng = Rng::from_seed(1729);
    for n in 2..=10 {
        let x: Vec<f64> = (0..n).map(|_| rng.uniform_f64()).collect();
        let y: Vec<f64> = (0..n).map(|_| rng.uniform_f64()).collect();
        let x2: Vec<f64> = x.iter().map(|&v| v * -2.0).collect();
        let y2: Vec<f64> = y.iter().map(|&v| v * -2.0).collect();
        let scaled = avg_spread(&x2, &y2).unwrap();
        let expected = 2.0 * avg_spread(&x, &y).unwrap();
        assert!(
            approx_eq!(f64, scaled, expected, epsilon = 1e-9),
            "n={}: avg_spread(-2x, -2y) = {} != 2*avg_spread(x, y) = {}",
            n,
            scaled,
            expected
        );
    }
}
