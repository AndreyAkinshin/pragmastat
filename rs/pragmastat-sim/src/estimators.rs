//! Simple statistical estimators not exposed by pragmastat's public API.

/// Arithmetic mean.
pub fn mean(values: &[f64]) -> f64 {
    let n = values.len();
    assert!(n > 0, "mean requires non-empty input");
    values.iter().sum::<f64>() / n as f64
}

/// Corrected sample standard deviation (Bessel's correction).
pub fn std_dev(values: &[f64]) -> f64 {
    let n = values.len();
    assert!(n > 1, "std_dev requires at least 2 values");
    let m = mean(values);
    let variance = values.iter().map(|&v| (v - m) * (v - m)).sum::<f64>() / (n - 1) as f64;
    variance.sqrt()
}

/// Median absolute deviation (MAD).
pub fn mad(values: &[f64]) -> f64 {
    let med = pragmastat::median(values).expect("MAD requires valid input");
    let mut abs_devs: Vec<f64> = values.iter().map(|&v| (v - med).abs()).collect();
    abs_devs.sort_by(|a, b| a.total_cmp(b));
    let n = abs_devs.len();
    if n.is_multiple_of(2) {
        (abs_devs[n / 2 - 1] + abs_devs[n / 2]) / 2.0
    } else {
        abs_devs[n / 2]
    }
}
