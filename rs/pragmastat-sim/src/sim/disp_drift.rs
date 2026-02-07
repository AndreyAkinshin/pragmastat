use super::drift::{format_drift_row, round_drift_row, DriftInput, DriftRow};
use super::{SimError, Simulation};
use crate::distributions::{self, DistributionEntry};
use crate::estimators;
use indexmap::IndexMap;
use pragmastat::Rng;
use std::collections::BTreeMap;

/// One-sample dispersion estimator function type.
type EstimatorFn = fn(&[f64]) -> f64;

fn lookup_estimator(name: &str) -> EstimatorFn {
    match name {
        "StdDev" => estimators::std_dev,
        "MAD" => estimators::mad,
        "Spread" => |v| pragmastat::spread(v).unwrap(),
        _ => panic!("Unknown dispersion estimator: {name}"),
    }
}

pub struct DispDriftSim {
    distributions: Vec<&'static DistributionEntry>,
    estimator_names: Vec<String>,
    sample_count: usize,
    base_seed: String,
}

impl DispDriftSim {
    pub fn new(
        distributions: Vec<&'static DistributionEntry>,
        estimator_names: Vec<String>,
        sample_count: usize,
        base_seed: String,
    ) -> Self {
        Self {
            distributions,
            estimator_names,
            sample_count,
            base_seed,
        }
    }
}

impl Simulation for DispDriftSim {
    type Input = DriftInput;
    type Row = DriftRow;

    fn name(&self) -> &'static str {
        "disp-drift"
    }

    fn create_inputs(
        &self,
        sample_sizes: &[usize],
        existing: &BTreeMap<String, DriftRow>,
        overwrite: bool,
    ) -> (Vec<DriftInput>, Vec<DriftRow>) {
        let mut inputs = Vec::new();
        let mut reused = Vec::new();

        for dist in &self.distributions {
            for &n in sample_sizes {
                let key = format!("{}-{}", dist.name, n);
                if !overwrite {
                    if let Some(row) = existing.get(&key) {
                        reused.push(row.clone());
                        continue;
                    }
                }
                inputs.push(DriftInput {
                    distribution_name: dist.name.to_string(),
                    estimator_names: self.estimator_names.clone(),
                    sample_count: self.sample_count,
                    sample_size: n,
                    base_seed: self.base_seed.clone(),
                });
            }
        }

        reused.sort();
        (inputs, reused)
    }

    fn simulate_row(
        &self,
        input: &DriftInput,
        progress: &dyn Fn(f64),
    ) -> Result<DriftRow, SimError> {
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

        let estimators: Vec<(&str, EstimatorFn)> = input
            .estimator_names
            .iter()
            .map(|name| (name.as_str(), lookup_estimator(name)))
            .collect();

        // Build sampling distributions
        let mut sampling: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        for name in &input.estimator_names {
            sampling.insert(name.clone(), Vec::with_capacity(input.sample_count));
        }

        for i in 0..input.sample_count {
            let sample = dist.samples(&mut rng, input.sample_size);
            for &(name, f) in &estimators {
                sampling.get_mut(name).unwrap().push(f(&sample));
            }
            progress((i + 1) as f64 / input.sample_count as f64);
        }

        // Compute drift: sqrt(n) * rel_spread(sampling)
        let n = input.sample_size as f64;
        let mut drifts = IndexMap::new();

        for name in &input.estimator_names {
            let values = &sampling[name];
            let rs = pragmastat::rel_spread(values).map_err(|e| SimError(format!("{e}")))?;
            drifts.insert(name.clone(), n.sqrt() * rs);
        }

        Ok(DriftRow {
            distribution: input.distribution_name.clone(),
            sample_size: input.sample_size,
            drifts: Some(drifts),
            error: None,
        })
    }

    fn create_error_row(&self, input: &DriftInput, error: &str) -> DriftRow {
        DriftRow {
            distribution: input.distribution_name.clone(),
            sample_size: input.sample_size,
            drifts: None,
            error: Some(error.to_string()),
        }
    }

    fn format_row(&self, row: &DriftRow) -> String {
        format_drift_row(row)
    }

    fn round_row(&self, row: DriftRow, digits: u32) -> DriftRow {
        round_drift_row(row, digits)
    }
}
