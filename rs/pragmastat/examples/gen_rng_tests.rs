//! Generate cross-language reference test data for Rng and distributions
//!
//! Run with: cargo run --example gen_rng_tests
//!
//! This generates JSON test files in the tests/ directory that all language
//! implementations must pass to ensure cross-language consistency.

use pragmastat::distributions::{Additive, Distribution, Exp, Multiplic, Power, Uniform};
use pragmastat::Rng;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize)]
struct UniformTestInput {
    seed: i64,
    count: usize,
}

#[derive(Serialize)]
struct UniformTestCase {
    input: UniformTestInput,
    output: Vec<f64>,
}

#[derive(Serialize)]
struct UniformRangeTestInput {
    seed: i64,
    min: f64,
    max: f64,
    count: usize,
}

#[derive(Serialize)]
struct UniformRangeTestCase {
    input: UniformRangeTestInput,
    output: Vec<f64>,
}

#[derive(Serialize)]
struct UniformF32TestInput {
    seed: i64,
    count: usize,
}

#[derive(Serialize)]
struct UniformF32TestCase {
    input: UniformF32TestInput,
    output: Vec<f32>,
}

#[derive(Serialize)]
struct UniformIntTestInput {
    seed: i64,
    min: i64,
    max: i64,
    count: usize,
}

#[derive(Serialize)]
struct UniformIntTestCase {
    input: UniformIntTestInput,
    output: Vec<i64>,
}

#[derive(Serialize)]
struct UniformI32TestInput {
    seed: i64,
    min: i32,
    max: i32,
    count: usize,
}

#[derive(Serialize)]
struct UniformI32TestCase {
    input: UniformI32TestInput,
    output: Vec<i32>,
}

#[derive(Serialize)]
struct UniformBoolTestInput {
    seed: i64,
    count: usize,
}

#[derive(Serialize)]
struct UniformBoolTestCase {
    input: UniformBoolTestInput,
    output: Vec<bool>,
}

#[derive(Serialize)]
struct StringSeedTestInput {
    seed: String,
    count: usize,
}

#[derive(Serialize)]
struct StringSeedTestCase {
    input: StringSeedTestInput,
    output: Vec<f64>,
}

#[derive(Serialize)]
struct ShuffleTestInput {
    seed: i64,
    x: Vec<f64>,
}

#[derive(Serialize)]
struct ShuffleTestCase {
    input: ShuffleTestInput,
    output: Vec<f64>,
}

#[derive(Serialize)]
struct SampleTestInput {
    seed: i64,
    x: Vec<f64>,
    k: usize,
}

#[derive(Serialize)]
struct SampleTestCase {
    input: SampleTestInput,
    output: Vec<f64>,
}

#[derive(Serialize)]
struct ResampleTestInput {
    seed: i64,
    x: Vec<f64>,
    k: usize,
}

#[derive(Serialize)]
struct ResampleTestCase {
    input: ResampleTestInput,
    output: Vec<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UniformDistInput {
    seed: i64,
    min: f64,
    max: f64,
    count: usize,
}

#[derive(Serialize)]
struct UniformDistTestCase {
    input: UniformDistInput,
    output: Vec<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AdditiveDistInput {
    seed: i64,
    mean: f64,
    std_dev: f64,
    count: usize,
}

#[derive(Serialize)]
struct AdditiveDistTestCase {
    input: AdditiveDistInput,
    output: Vec<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MultiplicDistInput {
    seed: i64,
    log_mean: f64,
    log_std_dev: f64,
    count: usize,
}

#[derive(Serialize)]
struct MultiplicDistTestCase {
    input: MultiplicDistInput,
    output: Vec<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExpDistInput {
    seed: i64,
    rate: f64,
    count: usize,
}

#[derive(Serialize)]
struct ExpDistTestCase {
    input: ExpDistInput,
    output: Vec<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PowerDistInput {
    seed: i64,
    min: f64,
    shape: f64,
    count: usize,
}

#[derive(Serialize)]
struct PowerDistTestCase {
    input: PowerDistInput,
    output: Vec<f64>,
}

fn find_tests_dir() -> PathBuf {
    // Find repository root by looking for CITATION.cff
    let mut current = std::env::current_dir().expect("Cannot get current dir");
    loop {
        if current.join("CITATION.cff").exists() {
            return current.join("tests");
        }
        if !current.pop() {
            panic!("Could not find repository root (CITATION.cff not found)");
        }
    }
}

fn write_json<T: Serialize>(path: &PathBuf, data: &T) {
    let json = serde_json::to_string_pretty(data).expect("Failed to serialize");
    fs::write(path, json).expect("Failed to write file");
    println!("  Written: {}", path.display());
}

fn string_seed_filename(seed: &str) -> String {
    if seed.is_empty() {
        return "empty".to_string();
    }

    let ascii_safe = seed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == ' ');
    if ascii_safe {
        return seed.replace(' ', "_");
    }

    let hex = seed
        .as_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    format!("utf8-{}", hex)
}

fn generate_uniform_tests(tests_dir: &PathBuf) {
    let rng_dir = tests_dir.join("rng");
    fs::create_dir_all(&rng_dir).expect("Failed to create rng test dir");

    let seeds: Vec<i64> = vec![
        0,
        1,
        1729,
        123,
        999,
        12345,
        2147483647,
        -1,
        -42,
        -2147483648,
    ];
    let count = 20;

    for seed in &seeds {
        let mut rng = Rng::from_seed(*seed);
        let values: Vec<f64> = (0..count).map(|_| rng.uniform()).collect();

        let test_case = UniformTestCase {
            input: UniformTestInput { seed: *seed, count },
            output: values,
        };

        let filename = format!("uniform-seed-{}.json", seed);
        write_json(&rng_dir.join(filename), &test_case);
    }
}

fn generate_uniform_range_tests(tests_dir: &PathBuf) {
    let rng_dir = tests_dir.join("rng");
    fs::create_dir_all(&rng_dir).expect("Failed to create rng test dir");

    let test_configs: Vec<(i64, f64, f64, usize)> = vec![
        // (seed, min, max, count)
        (1729, 0.0, 1.0, 20),
        (1729, -1.0, 1.0, 20),
        (1729, 0.0, 100.0, 20),
        (1729, -50.0, 50.0, 20),
        (123, 0.0, 1.0, 20),
        (0, -1.0, 1.0, 20),
        (999, 0.0, 100.0, 20),
    ];

    for (seed, min, max, count) in test_configs {
        let mut rng = Rng::from_seed(seed);
        let values: Vec<f64> = (0..count).map(|_| rng.uniform_range(min, max)).collect();

        let test_case = UniformRangeTestCase {
            input: UniformRangeTestInput {
                seed,
                min,
                max,
                count,
            },
            output: values,
        };

        let filename = format!("uniform-range-seed-{}-{}-{}.json", seed, min, max);
        write_json(&rng_dir.join(filename), &test_case);
    }
}

fn generate_uniform_f32_tests(tests_dir: &PathBuf) {
    let rng_dir = tests_dir.join("rng");
    fs::create_dir_all(&rng_dir).expect("Failed to create rng test dir");

    let seeds: Vec<i64> = vec![0, 1, 1729, 123, 999, -1, -42];
    let count = 20;

    for seed in &seeds {
        let mut rng = Rng::from_seed(*seed);
        let values: Vec<f32> = (0..count).map(|_| rng.uniform_f32()).collect();

        let test_case = UniformF32TestCase {
            input: UniformF32TestInput { seed: *seed, count },
            output: values,
        };

        let filename = format!("uniform-f32-seed-{}.json", seed);
        write_json(&rng_dir.join(filename), &test_case);
    }
}

fn generate_uniform_i32_tests(tests_dir: &PathBuf) {
    let rng_dir = tests_dir.join("rng");
    fs::create_dir_all(&rng_dir).expect("Failed to create rng test dir");

    let test_configs: Vec<(i64, i32, i32, usize)> = vec![
        // (seed, min, max, count)
        (1729, 0, 1000, 20),
        (1729, -500, 500, 20),
        (123, 0, 1000, 20),
        (0, -500, 500, 20),
        (999, 0, 100, 20),
    ];

    for (seed, min, max, count) in test_configs {
        let mut rng = Rng::from_seed(seed);
        let values: Vec<i32> = (0..count).map(|_| rng.uniform_i32(min, max)).collect();

        let test_case = UniformI32TestCase {
            input: UniformI32TestInput {
                seed,
                min,
                max,
                count,
            },
            output: values,
        };

        let filename = format!("uniform-i32-seed-{}-{}-{}.json", seed, min, max);
        write_json(&rng_dir.join(filename), &test_case);
    }
}

fn generate_uniform_bool_tests(tests_dir: &PathBuf) {
    let rng_dir = tests_dir.join("rng");
    fs::create_dir_all(&rng_dir).expect("Failed to create rng test dir");

    let seeds: Vec<i64> = vec![0, 1, 1729, 123, 999, -1, -42];
    let count = 100;

    for seed in &seeds {
        let mut rng = Rng::from_seed(*seed);
        let values: Vec<bool> = (0..count).map(|_| rng.uniform_bool()).collect();

        let test_case = UniformBoolTestCase {
            input: UniformBoolTestInput { seed: *seed, count },
            output: values,
        };

        let filename = format!("uniform-bool-seed-{}.json", seed);
        write_json(&rng_dir.join(filename), &test_case);
    }
}

fn generate_uniform_int_tests(tests_dir: &PathBuf) {
    let rng_dir = tests_dir.join("rng");
    fs::create_dir_all(&rng_dir).expect("Failed to create rng test dir");

    let test_configs: Vec<(i64, i64, i64, usize)> = vec![
        // (seed, min, max, count)
        (1729, 0, 100, 20),
        (1729, -50, 50, 20),
        (1729, 0, 10, 20),
        (1729, 1000, 2000, 20),
        (123, 0, 100, 20),
        (0, 0, 100, 20),
        (999, -100, 100, 20),
    ];

    for (seed, min, max, count) in test_configs {
        let mut rng = Rng::from_seed(seed);
        let values: Vec<i64> = (0..count).map(|_| rng.uniform_i64(min, max)).collect();

        let test_case = UniformIntTestCase {
            input: UniformIntTestInput {
                seed,
                min,
                max,
                count,
            },
            output: values,
        };

        let filename = format!("uniform-int-seed-{}-{}-{}.json", seed, min, max);
        write_json(&rng_dir.join(filename), &test_case);
    }
}

fn generate_string_seed_tests(tests_dir: &PathBuf) {
    let rng_dir = tests_dir.join("rng");
    fs::create_dir_all(&rng_dir).expect("Failed to create rng test dir");

    let seeds: Vec<&str> = vec![
        "test",
        "experiment-1",
        "abc",
        "hello world",
        "",
        "a",
        "Rng",
        "pragmastat",
        "héllo",
        "π",
        "你好",
    ];
    let count = 20;

    for seed in &seeds {
        let mut rng = Rng::from_string(seed);
        let values: Vec<f64> = (0..count).map(|_| rng.uniform()).collect();

        let test_case = StringSeedTestCase {
            input: StringSeedTestInput {
                seed: seed.to_string(),
                count,
            },
            output: values,
        };

        // Use a safe filename
        let safe_name = string_seed_filename(seed);
        let filename = format!("uniform-string-{}.json", safe_name);
        write_json(&rng_dir.join(filename), &test_case);
    }
}

fn generate_shuffle_tests(tests_dir: &PathBuf) {
    let shuffle_dir = tests_dir.join("shuffle");
    fs::create_dir_all(&shuffle_dir).expect("Failed to create shuffle test dir");

    // (seed, x, suffix for unique naming)
    let test_configs: Vec<(i64, Vec<f64>, &str)> = vec![
        (1729, vec![1.0, 2.0, 3.0, 4.0, 5.0], "basic"),
        (1729, (0..10).map(|i| i as f64).collect(), "seq"),
        (1729, (0..20).map(|i| i as f64).collect(), "seq"),
        (123, vec![1.0, 2.0, 3.0, 4.0, 5.0], "basic"),
        (123, (0..10).map(|i| i as f64).collect(), "seq"),
        (0, vec![1.0, 2.0, 3.0, 4.0, 5.0], "basic"),
        (999, vec![1.0, 2.0, 3.0, 4.0, 5.0], "basic"),
        (1729, vec![1.0, 2.0], "basic"),
        (1729, vec![1.0], "single"),
        (1729, vec![0.0, 0.0, 0.0, 0.0, 0.0], "zeros"),
        (1729, vec![-5.0, -3.0, -1.0, 1.0, 3.0, 5.0], "neg"),
        (1729, (0..100).map(|i| i as f64).collect(), "seq"),
    ];

    for (seed, x, suffix) in test_configs {
        let mut rng = Rng::from_seed(seed);
        let shuffled = rng.shuffle(&x);

        let test_case = ShuffleTestCase {
            input: ShuffleTestInput { seed, x: x.clone() },
            output: shuffled,
        };

        let filename = format!("seed-{}-n{}-{}.json", seed, x.len(), suffix);
        write_json(&shuffle_dir.join(filename), &test_case);
    }
}

fn generate_sample_tests(tests_dir: &PathBuf) {
    let sample_dir = tests_dir.join("sample");
    fs::create_dir_all(&sample_dir).expect("Failed to create sample test dir");

    let test_configs: Vec<(i64, Vec<f64>, usize)> = vec![
        // (seed, x, k)
        (1729, (0..10).map(|i| i as f64).collect(), 3),
        (1729, (0..10).map(|i| i as f64).collect(), 5),
        (1729, (0..10).map(|i| i as f64).collect(), 1),
        (1729, (0..10).map(|i| i as f64).collect(), 10),
        (1729, (0..10).map(|i| i as f64).collect(), 15), // k > n
        (123, (0..10).map(|i| i as f64).collect(), 3),
        (0, (0..10).map(|i| i as f64).collect(), 3),
        (999, (0..10).map(|i| i as f64).collect(), 3),
        (1729, (0..20).map(|i| i as f64).collect(), 5),
        (1729, (0..20).map(|i| i as f64).collect(), 10),
        (1729, (0..100).map(|i| i as f64).collect(), 10),
        (1729, (0..100).map(|i| i as f64).collect(), 25),
        (1729, vec![1.0, 2.0, 3.0, 4.0, 5.0], 3),
        (1729, vec![1.0, 2.0], 1),
        (1729, vec![1.0], 1),
    ];

    for (seed, x, k) in test_configs {
        let mut rng = Rng::from_seed(seed);
        let sampled = rng.sample(&x, k);

        let test_case = SampleTestCase {
            input: SampleTestInput {
                seed,
                x: x.clone(),
                k,
            },
            output: sampled,
        };

        let filename = format!("seed-{}-n{}-k{}.json", seed, x.len(), k);
        write_json(&sample_dir.join(filename), &test_case);
    }
}

fn generate_resample_tests(tests_dir: &PathBuf) {
    let resample_dir = tests_dir.join("resample");
    fs::create_dir_all(&resample_dir).expect("Failed to create resample test dir");

    let test_configs: Vec<(i64, Vec<f64>, usize)> = vec![
        // (seed, x, k)
        (1729, (0..10).map(|i| i as f64).collect(), 3),
        (1729, (0..10).map(|i| i as f64).collect(), 5),
        (1729, (0..10).map(|i| i as f64).collect(), 10),
        (1729, (0..10).map(|i| i as f64).collect(), 15),
        (1729, (0..10).map(|i| i as f64).collect(), 1),
        (1729, vec![0.0, 1.0, 2.0, 3.0, 4.0], 3),
        (1729, vec![0.0, 1.0, 2.0, 3.0, 4.0], 7),
        (1729, vec![0.0], 1),
        (1729, vec![0.0, 1.0], 1),
        (1729, (0..20).map(|i| i as f64).collect(), 5),
        (1729, (0..20).map(|i| i as f64).collect(), 10),
        (1729, (0..100).map(|i| i as f64).collect(), 10),
        (1729, (0..100).map(|i| i as f64).collect(), 25),
        (123, (0..10).map(|i| i as f64).collect(), 3),
        (0, (0..10).map(|i| i as f64).collect(), 3),
        (999, (0..10).map(|i| i as f64).collect(), 3),
    ];

    for (seed, x, k) in test_configs {
        let mut rng = Rng::from_seed(seed);
        let resampled = rng.resample(&x, k);

        let test_case = ResampleTestCase {
            input: ResampleTestInput {
                seed,
                x: x.clone(),
                k,
            },
            output: resampled,
        };

        let filename = format!("seed-{}-n{}-k{}.json", seed, x.len(), k);
        write_json(&resample_dir.join(filename), &test_case);
    }
}

fn generate_uniform_distribution_tests(tests_dir: &PathBuf) {
    let dist_dir = tests_dir.join("distributions").join("uniform");
    fs::create_dir_all(&dist_dir).expect("Failed to create uniform distribution test dir");

    let test_configs: Vec<(i64, f64, f64, usize)> =
        vec![(1729, -1.0, 1.0, 10), (123, 0.0, 10.0, 10)];

    for (seed, min, max, count) in test_configs {
        let mut rng = Rng::from_seed(seed);
        let dist = Uniform::new(min, max);
        let values: Vec<f64> = (0..count).map(|_| dist.sample(&mut rng)).collect();

        let test_case = UniformDistTestCase {
            input: UniformDistInput {
                seed,
                min,
                max,
                count,
            },
            output: values,
        };

        let filename = format!("seed-{}-min-{}-max-{}.json", seed, min, max);
        write_json(&dist_dir.join(filename), &test_case);
    }
}

fn generate_additive_distribution_tests(tests_dir: &PathBuf) {
    let dist_dir = tests_dir.join("distributions").join("additive");
    fs::create_dir_all(&dist_dir).expect("Failed to create additive distribution test dir");

    let test_configs: Vec<(i64, f64, f64, usize)> =
        vec![(1729, 0.0, 1.0, 10), (123, 10.0, 2.0, 10)];

    for (seed, mean, std_dev, count) in test_configs {
        let mut rng = Rng::from_seed(seed);
        let dist = Additive::new(mean, std_dev);
        let values: Vec<f64> = (0..count).map(|_| dist.sample(&mut rng)).collect();

        let test_case = AdditiveDistTestCase {
            input: AdditiveDistInput {
                seed,
                mean,
                std_dev,
                count,
            },
            output: values,
        };

        let filename = format!("seed-{}-mean-{}-stddev-{}.json", seed, mean, std_dev);
        write_json(&dist_dir.join(filename), &test_case);
    }
}

fn generate_multiplic_distribution_tests(tests_dir: &PathBuf) {
    let dist_dir = tests_dir.join("distributions").join("multiplic");
    fs::create_dir_all(&dist_dir).expect("Failed to create multiplic distribution test dir");

    let test_configs: Vec<(i64, f64, f64, usize)> =
        vec![(1729, 0.0, 1.0, 10), (123, -1.0, 2.0, 10)];

    for (seed, log_mean, log_std_dev, count) in test_configs {
        let mut rng = Rng::from_seed(seed);
        let dist = Multiplic::new(log_mean, log_std_dev);
        let values: Vec<f64> = (0..count).map(|_| dist.sample(&mut rng)).collect();

        let test_case = MultiplicDistTestCase {
            input: MultiplicDistInput {
                seed,
                log_mean,
                log_std_dev,
                count,
            },
            output: values,
        };

        let filename = format!(
            "seed-{}-logmean-{}-logstddev-{}.json",
            seed, log_mean, log_std_dev
        );
        write_json(&dist_dir.join(filename), &test_case);
    }
}

fn generate_exp_distribution_tests(tests_dir: &PathBuf) {
    let dist_dir = tests_dir.join("distributions").join("exp");
    fs::create_dir_all(&dist_dir).expect("Failed to create exp distribution test dir");

    let test_configs: Vec<(i64, f64, usize)> = vec![(1729, 1.0, 10), (123, 2.0, 10)];

    for (seed, rate, count) in test_configs {
        let mut rng = Rng::from_seed(seed);
        let dist = Exp::new(rate);
        let values: Vec<f64> = (0..count).map(|_| dist.sample(&mut rng)).collect();

        let test_case = ExpDistTestCase {
            input: ExpDistInput { seed, rate, count },
            output: values,
        };

        let filename = format!("seed-{}-rate-{}.json", seed, rate);
        write_json(&dist_dir.join(filename), &test_case);
    }
}

fn generate_power_distribution_tests(tests_dir: &PathBuf) {
    let dist_dir = tests_dir.join("distributions").join("power");
    fs::create_dir_all(&dist_dir).expect("Failed to create power distribution test dir");

    let test_configs: Vec<(i64, f64, f64, usize)> = vec![(1729, 1.0, 2.0, 10), (123, 2.0, 3.0, 10)];

    for (seed, min, shape, count) in test_configs {
        let mut rng = Rng::from_seed(seed);
        let dist = Power::new(min, shape);
        let values: Vec<f64> = (0..count).map(|_| dist.sample(&mut rng)).collect();

        let test_case = PowerDistTestCase {
            input: PowerDistInput {
                seed,
                min,
                shape,
                count,
            },
            output: values,
        };

        let filename = format!("seed-{}-min-{}-shape-{}.json", seed, min, shape);
        write_json(&dist_dir.join(filename), &test_case);
    }
}

fn main() {
    let tests_dir = find_tests_dir();

    println!("Generating RNG test data in: {}", tests_dir.display());
    println!();

    println!("Generating uniform tests...");
    generate_uniform_tests(&tests_dir);
    println!();

    println!("Generating uniform_range tests...");
    generate_uniform_range_tests(&tests_dir);
    println!();

    println!("Generating uniform_f32 tests...");
    generate_uniform_f32_tests(&tests_dir);
    println!();

    println!("Generating uniform_int tests...");
    generate_uniform_int_tests(&tests_dir);
    println!();

    println!("Generating uniform_i32 tests...");
    generate_uniform_i32_tests(&tests_dir);
    println!();

    println!("Generating uniform_bool tests...");
    generate_uniform_bool_tests(&tests_dir);
    println!();

    println!("Generating string seed tests...");
    generate_string_seed_tests(&tests_dir);
    println!();

    println!("Generating shuffle tests...");
    generate_shuffle_tests(&tests_dir);
    println!();

    println!("Generating sample tests...");
    generate_sample_tests(&tests_dir);
    println!();

    println!("Generating resample tests...");
    generate_resample_tests(&tests_dir);
    println!();

    println!("Generating distribution tests...");
    generate_uniform_distribution_tests(&tests_dir);
    generate_additive_distribution_tests(&tests_dir);
    generate_multiplic_distribution_tests(&tests_dir);
    generate_exp_distribution_tests(&tests_dir);
    generate_power_distribution_tests(&tests_dir);
    println!();

    println!("Done! Test data generated successfully.");
}
