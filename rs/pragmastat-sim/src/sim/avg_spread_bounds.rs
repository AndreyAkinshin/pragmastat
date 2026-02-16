use super::bounds::{
    format_two_sample_bounds_row, min_achievable_misrate_avg_spread, parse_misrates,
    resolve_sample_count, round_two_sample_bounds_row, TwoSampleBoundsInput, TwoSampleBoundsRow,
};
use super::{SimError, Simulation};
use crate::distributions::{asymptotic_spread, find_distributions, DistributionEntry};
use pragmastat::Rng;
use std::collections::BTreeMap;

pub struct AvgSpreadBoundsSim {
    distributions: Vec<&'static DistributionEntry>,
    sample_count: Option<usize>,
    misrates: Vec<f64>,
    base_seed: String,
    /// Second dimension of sample sizes (y). Pairs are generated as (n, m) with n <= m.
    sizes_y: Vec<usize>,
}

impl AvgSpreadBoundsSim {
    pub fn new(
        distributions: Vec<&'static DistributionEntry>,
        sample_count: Option<usize>,
        misrates_str: &str,
        base_seed: String,
        sizes_y: Vec<usize>,
    ) -> Self {
        Self {
            distributions,
            sample_count,
            misrates: parse_misrates(misrates_str),
            base_seed,
            sizes_y,
        }
    }
}

impl Simulation for AvgSpreadBoundsSim {
    type Input = TwoSampleBoundsInput;
    type Row = TwoSampleBoundsRow;

    fn name(&self) -> &'static str {
        "avg-spread-bounds"
    }

    fn create_inputs(
        &self,
        sample_sizes: &[usize],
        existing: &BTreeMap<String, TwoSampleBoundsRow>,
        overwrite: bool,
    ) -> (Vec<TwoSampleBoundsInput>, Vec<TwoSampleBoundsRow>) {
        let mut inputs = Vec::new();
        let mut reused = Vec::new();

        for dist in &self.distributions {
            for &n in sample_sizes {
                for &m in &self.sizes_y {
                    if n > m {
                        continue;
                    }
                    for &misrate in &self.misrates {
                        let min_misrate = min_achievable_misrate_avg_spread(n, m);
                        if misrate < min_misrate {
                            continue;
                        }
                        let key = format!("{}-{}-{}-{}", dist.name, n, m, misrate);
                        if !overwrite {
                            if let Some(row) = existing.get(&key) {
                                reused.push(row.clone());
                                continue;
                            }
                        }
                        inputs.push(TwoSampleBoundsInput {
                            distribution_name: dist.name.to_string(),
                            sample_count: resolve_sample_count(self.sample_count, misrate),
                            sample_size_x: n,
                            sample_size_y: m,
                            misrate,
                            base_seed: self.base_seed.clone(),
                        });
                    }
                }
            }
        }

        reused.sort();
        (inputs, reused)
    }

    fn simulate_row(
        &self,
        input: &TwoSampleBoundsInput,
        progress: &dyn Fn(f64),
    ) -> Result<TwoSampleBoundsRow, SimError> {
        let dist_entry =
            find_distributions(std::slice::from_ref(&input.distribution_name))
                .into_iter()
                .next()
                .expect("distribution not found");
        let dist = dist_entry.create();
        let mut rng = Rng::from_string(&format!(
            "{}-{}-{}-{}",
            input.base_seed, input.distribution_name, input.sample_size_x, input.sample_size_y
        ));

        let true_value = asymptotic_spread(dist_entry);
        let mut coverage = 0_usize;

        for i in 0..input.sample_count {
            let x: Vec<f64> = dist.samples(&mut rng, input.sample_size_x);
            let y: Vec<f64> = dist.samples(&mut rng, input.sample_size_y);
            let bounds = pragmastat::estimators::avg_spread_bounds(&x, &y, input.misrate)
                .map_err(|e| SimError(format!("{e}")))?;

            if bounds.lower <= true_value && true_value <= bounds.upper {
                coverage += 1;
            }

            if i % 1000 == 0 {
                progress((i + 1) as f64 / input.sample_count as f64);
            }
        }

        let observed = 1.0 - coverage as f64 / input.sample_count as f64;
        Ok(TwoSampleBoundsRow {
            distribution: input.distribution_name.clone(),
            sample_size_x: input.sample_size_x,
            sample_size_y: input.sample_size_y,
            requested_misrate: input.misrate,
            sample_count: input.sample_count,
            observed_misrate: Some(observed),
            error: None,
        })
    }

    fn create_error_row(&self, input: &TwoSampleBoundsInput, error: &str) -> TwoSampleBoundsRow {
        TwoSampleBoundsRow {
            distribution: input.distribution_name.clone(),
            sample_size_x: input.sample_size_x,
            sample_size_y: input.sample_size_y,
            requested_misrate: input.misrate,
            sample_count: input.sample_count,
            observed_misrate: None,
            error: Some(error.to_string()),
        }
    }

    fn format_row(&self, row: &TwoSampleBoundsRow) -> String {
        format_two_sample_bounds_row(row)
    }

    fn round_row(&self, row: TwoSampleBoundsRow, digits: u32) -> TwoSampleBoundsRow {
        round_two_sample_bounds_row(row, digits)
    }
}
