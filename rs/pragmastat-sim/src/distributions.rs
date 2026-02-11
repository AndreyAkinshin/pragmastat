use pragmastat::{Additive, Distribution, Exp, Multiplic, Power, Rng, Uniform};

/// A named distribution with its asymptotic spread constant.
pub struct DistributionEntry {
    pub name: &'static str,
    factory: fn() -> Box<dyn Distribution + Send + Sync>,
    /// Known asymptotic spread, or None if it must be estimated empirically.
    asymptotic_spread: Option<f64>,
    /// True center of symmetry, or None if the distribution is not symmetric.
    pub center: Option<f64>,
}

impl DistributionEntry {
    pub fn create(&self) -> Box<dyn Distribution + Send + Sync> {
        (self.factory)()
    }
}

/// All available simulation distributions.
pub const DISTRIBUTIONS: &[DistributionEntry] = &[
    DistributionEntry {
        name: "Additive",
        factory: || Box::new(Additive::new(0.0, 1.0)),
        asymptotic_spread: Some(0.953_872_552_4),
        center: Some(0.0),
    },
    DistributionEntry {
        name: "Multiplic",
        factory: || Box::new(Multiplic::new(0.0, 1.0)),
        asymptotic_spread: None,
        center: None,
    },
    DistributionEntry {
        name: "Exp",
        factory: || Box::new(Exp::new(1.0)),
        asymptotic_spread: None,
        center: None,
    },
    DistributionEntry {
        name: "Power",
        factory: || Box::new(Power::new(1.0, 3.0)),
        asymptotic_spread: None,
        center: None,
    },
    DistributionEntry {
        name: "Uniform",
        factory: || Box::new(Uniform::new(0.0, 1.0)),
        asymptotic_spread: Some(1.0 - std::f64::consts::FRAC_1_SQRT_2),
        center: Some(0.5),
    },
];

/// Look up distribution entries by name (case-insensitive).
pub fn find_distributions(names: &[String]) -> Vec<&'static DistributionEntry> {
    names
        .iter()
        .filter_map(|name| {
            DISTRIBUTIONS
                .iter()
                .find(|d| d.name.eq_ignore_ascii_case(name))
        })
        .collect()
}

/// Get asymptotic spread for a distribution, estimating empirically if unknown.
pub fn asymptotic_spread(entry: &DistributionEntry) -> f64 {
    if let Some(val) = entry.asymptotic_spread {
        return val;
    }
    estimate_asymptotic_spread(entry)
}

fn estimate_asymptotic_spread(entry: &DistributionEntry) -> f64 {
    const SAMPLING_SIZE: usize = 10_000_000;
    let dist = entry.create();
    let mut rng = Rng::from_string("asymptotic-spread");
    let mut diffs = Vec::with_capacity(SAMPLING_SIZE);
    for _ in 0..SAMPLING_SIZE {
        let a = dist.sample(&mut rng);
        let b = dist.sample(&mut rng);
        diffs.push((a - b).abs());
    }
    crate::estimators::median(&diffs)
}

/// Returns true if the distribution is always positive (for ratio-bounds).
pub fn is_positive(name: &str) -> bool {
    name.eq_ignore_ascii_case("Multiplic")
        || name.eq_ignore_ascii_case("Exp")
        || name.eq_ignore_ascii_case("Uniform")
}
