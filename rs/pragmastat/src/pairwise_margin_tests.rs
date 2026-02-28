use crate::pairwise_margin::pairwise_margin;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct PairwiseMarginInput {
    n: usize,
    m: usize,
    misrate: f64,
}

#[derive(Debug, Deserialize)]
struct ExpectedError {
    id: String,
    subject: String,
}

#[derive(Debug, Deserialize)]
struct PairwiseMarginTestCase {
    input: PairwiseMarginInput,
    output: Option<u64>,
    expected_error: Option<ExpectedError>,
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
fn test_pairwise_margin_reference() {
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

        if let Some(ref expected_error) = test_case.expected_error {
            match pairwise_margin(
                test_case.input.n,
                test_case.input.m,
                test_case.input.misrate,
            ) {
                Ok(_) => failures.push(format!("{file_name:?}: expected error, got Ok")),
                Err(ae) => {
                    let violation = ae.violation();
                    if violation.id.as_str() != expected_error.id {
                        failures.push(format!(
                            "{file_name:?}: expected violation id \"{}\", got \"{}\"",
                            expected_error.id,
                            violation.id.as_str()
                        ));
                    }
                    if violation.subject.as_str() != expected_error.subject {
                        failures.push(format!(
                            "{file_name:?}: expected violation subject \"{}\", got \"{}\"",
                            expected_error.subject,
                            violation.subject.as_str()
                        ));
                    }
                }
            }
            continue;
        }

        let expected_output = test_case.output.expect("Test case must have output");
        match pairwise_margin(
            test_case.input.n,
            test_case.input.m,
            test_case.input.misrate,
        ) {
            Ok(actual) => {
                if actual != expected_output {
                    failures.push(format!(
                        "{file_name:?}: expected {expected_output}, got {actual}",
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
