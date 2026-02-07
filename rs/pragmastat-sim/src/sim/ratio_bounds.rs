use super::bounds::{format_bounds_row, parse_misrates, round_bounds_row, BoundsInput, BoundsRow};
use super::{SimError, Simulation};
use crate::distributions::{self, DistributionEntry};
use pragmastat::Rng;
use std::collections::BTreeMap;

pub struct RatioBoundsSim {
    distributions: Vec<&'static DistributionEntry>,
    sample_count: usize,
    misrates: Vec<f64>,
    base_seed: String,
}

impl RatioBoundsSim {
    pub fn new(
        distributions: Vec<&'static DistributionEntry>,
        sample_count: usize,
        misrates_str: &str,
        base_seed: String,
    ) -> Self {
        Self {
            distributions,
            sample_count,
            misrates: parse_misrates(misrates_str),
            base_seed,
        }
    }
}

impl Simulation for RatioBoundsSim {
    type Input = BoundsInput;
    type Row = BoundsRow;

    fn name(&self) -> &'static str {
        "ratio-bounds"
    }

    fn create_inputs(
        &self,
        sample_sizes: &[usize],
        existing: &BTreeMap<String, BoundsRow>,
        overwrite: bool,
    ) -> (Vec<BoundsInput>, Vec<BoundsRow>) {
        let mut inputs = Vec::new();
        let mut reused = Vec::new();

        for dist in &self.distributions {
            if !distributions::is_positive(dist.name) {
                continue;
            }
            for &n in sample_sizes {
                for &misrate in &self.misrates {
                    let key = format!("{}-{}-{}", dist.name, n, misrate);
                    if !overwrite {
                        if let Some(row) = existing.get(&key) {
                            reused.push(row.clone());
                            continue;
                        }
                    }
                    inputs.push(BoundsInput {
                        distribution_name: dist.name.to_string(),
                        sample_count: self.sample_count,
                        sample_size: n,
                        misrate,
                        base_seed: self.base_seed.clone(),
                    });
                }
            }
        }

        reused.sort();
        (inputs, reused)
    }

    fn simulate_row(
        &self,
        input: &BoundsInput,
        progress: &dyn Fn(f64),
    ) -> Result<BoundsRow, SimError> {
        let dist_entry =
            distributions::find_distributions(std::slice::from_ref(&input.distribution_name))
                .into_iter()
                .next()
                .expect("distribution not found");
        let dist = dist_entry.create();
        let mut rng = Rng::from_string(&format!(
            "{}-{}-{}",
            input.base_seed, input.distribution_name, input.sample_size
        ));

        let true_value = 1.0;
        let mut coverage = 0_usize;

        for i in 0..input.sample_count {
            let x: Vec<f64> = dist.samples(&mut rng, input.sample_size);
            let y: Vec<f64> = dist.samples(&mut rng, input.sample_size);

            let bounds = pragmastat::ratio_bounds(&x, &y, input.misrate)
                .map_err(|e| SimError(format!("{e}")))?;

            if bounds.lower <= true_value && true_value <= bounds.upper {
                coverage += 1;
            }

            if i % 1000 == 0 {
                progress((i + 1) as f64 / input.sample_count as f64);
            }
        }

        let observed = 1.0 - coverage as f64 / input.sample_count as f64;
        Ok(BoundsRow {
            distribution: input.distribution_name.clone(),
            sample_size: input.sample_size,
            requested_misrate: input.misrate,
            observed_misrate: Some(observed),
            error: None,
        })
    }

    fn create_error_row(&self, input: &BoundsInput, error: &str) -> BoundsRow {
        BoundsRow {
            distribution: input.distribution_name.clone(),
            sample_size: input.sample_size,
            requested_misrate: input.misrate,
            observed_misrate: None,
            error: Some(error.to_string()),
        }
    }

    fn format_row(&self, row: &BoundsRow) -> String {
        format_bounds_row(row)
    }

    fn round_row(&self, row: BoundsRow, digits: u32) -> BoundsRow {
        round_bounds_row(row, digits)
    }
}
