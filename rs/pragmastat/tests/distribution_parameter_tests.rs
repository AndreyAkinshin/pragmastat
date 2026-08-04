//! The five parametric families refuse malformed parameters at construction. Every parameter has
//! to be finite, and every scale or shape parameter has to be positive.
//!
//! Two gaps this covers. Each family used to check only its scale parameter, so a non-finite
//! location constructed a family whose every draw is NaN. And in the other ports the predicate was
//! written negatively (`std_dev <= 0`), which is not the negation of `std_dev > 0` under IEEE 754:
//! `NaN <= 0` is false, so NaN walked past the guard. Rust's `assert!` already took the positive
//! form, so only the location parameters and the infinite scales were reachable here.

use pragmastat::{Additive, Distribution, Exp, Multiplic, Power, Rng, Uniform};

/// One refusal per case, as a `#[should_panic]` test named for what it refuses.
macro_rules! refuses {
    ($name:ident, $message:literal, $construct:expr) => {
        #[test]
        #[should_panic(expected = $message)]
        fn $name() {
            let _ = $construct;
        }
    };
}

refuses!(
    uniform_nan_min,
    "min must be finite",
    Uniform::new(f64::NAN, 1.0)
);
refuses!(
    uniform_infinite_min,
    "min must be finite",
    Uniform::new(f64::NEG_INFINITY, 1.0)
);
refuses!(
    uniform_nan_max,
    "max must be finite",
    Uniform::new(0.0, f64::NAN)
);
refuses!(
    uniform_infinite_max,
    "max must be finite",
    Uniform::new(0.0, f64::INFINITY)
);
refuses!(
    uniform_empty_interval,
    "min must be less than max",
    Uniform::new(1.0, 1.0)
);
refuses!(
    uniform_inverted_interval,
    "min must be less than max",
    Uniform::new(2.0, 1.0)
);

refuses!(
    additive_nan_mean,
    "mean must be finite",
    Additive::new(f64::NAN, 1.0)
);
refuses!(
    additive_infinite_mean,
    "mean must be finite",
    Additive::new(f64::INFINITY, 1.0)
);
refuses!(
    additive_nan_std_dev,
    "std_dev must be finite and positive",
    Additive::new(0.0, f64::NAN)
);
refuses!(
    additive_infinite_std_dev,
    "std_dev must be finite and positive",
    Additive::new(0.0, f64::INFINITY)
);
refuses!(
    additive_zero_std_dev,
    "std_dev must be finite and positive",
    Additive::new(0.0, 0.0)
);
refuses!(
    additive_negative_std_dev,
    "std_dev must be finite and positive",
    Additive::new(0.0, -1.0)
);

refuses!(
    multiplic_nan_log_mean,
    "log_mean must be finite",
    Multiplic::new(f64::NAN, 1.0)
);
refuses!(
    multiplic_infinite_log_mean,
    "log_mean must be finite",
    Multiplic::new(f64::INFINITY, 1.0)
);
refuses!(
    multiplic_nan_log_std_dev,
    "log_std_dev must be finite and positive",
    Multiplic::new(0.0, f64::NAN)
);
refuses!(
    multiplic_infinite_log_std_dev,
    "log_std_dev must be finite and positive",
    Multiplic::new(0.0, f64::INFINITY)
);
refuses!(
    multiplic_zero_log_std_dev,
    "log_std_dev must be finite and positive",
    Multiplic::new(0.0, 0.0)
);
refuses!(
    multiplic_negative_log_std_dev,
    "log_std_dev must be finite and positive",
    Multiplic::new(0.0, -1.0)
);

refuses!(
    exp_nan_rate,
    "rate must be finite and positive",
    Exp::new(f64::NAN)
);
refuses!(
    exp_infinite_rate,
    "rate must be finite and positive",
    Exp::new(f64::INFINITY)
);
refuses!(
    exp_zero_rate,
    "rate must be finite and positive",
    Exp::new(0.0)
);
refuses!(
    exp_negative_rate,
    "rate must be finite and positive",
    Exp::new(-1.0)
);

refuses!(
    power_nan_min,
    "min must be finite and positive",
    Power::new(f64::NAN, 3.0)
);
refuses!(
    power_infinite_min,
    "min must be finite and positive",
    Power::new(f64::INFINITY, 3.0)
);
refuses!(
    power_zero_min,
    "min must be finite and positive",
    Power::new(0.0, 3.0)
);
refuses!(
    power_negative_min,
    "min must be finite and positive",
    Power::new(-1.0, 3.0)
);
refuses!(
    power_nan_shape,
    "shape must be finite and positive",
    Power::new(1.0, f64::NAN)
);
refuses!(
    power_infinite_shape,
    "shape must be finite and positive",
    Power::new(1.0, f64::INFINITY)
);
refuses!(
    power_zero_shape,
    "shape must be finite and positive",
    Power::new(1.0, 0.0)
);
refuses!(
    power_negative_shape,
    "shape must be finite and positive",
    Power::new(1.0, -1.0)
);

#[test]
fn well_formed_families_still_construct_and_draw() {
    let mut rng = Rng::from_string("distribution-parameters");
    let families: Vec<Box<dyn Distribution>> = vec![
        Box::new(Uniform::new(-2.5, 7.5)),
        Box::new(Additive::new(10.0, 2.0)),
        Box::new(Multiplic::new(1.0, 0.5)),
        Box::new(Exp::new(3.0)),
        Box::new(Power::new(1.0, 2.0)),
    ];
    for family in &families {
        assert!(family.sample(&mut rng).is_finite());
    }
}
