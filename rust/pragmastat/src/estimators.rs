//! Statistical estimators for one-sample and two-sample analysis

/// Calculates the median of a sorted slice
fn median_sorted(sorted: &[f64]) -> Result<f64, &'static str> {
    let n = sorted.len();
    if n == 0 {
        return Err("Input slice cannot be empty");
    }
    if n % 2 == 0 {
        Ok((sorted[n / 2 - 1] + sorted[n / 2]) / 2.0)
    } else {
        Ok(sorted[n / 2])
    }
}

/// Calculates the median of a slice
fn median(values: &[f64]) -> Result<f64, &'static str> {
    if values.is_empty() {
        return Err("Input slice cannot be empty");
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    median_sorted(&sorted)
}

/// Estimates the central value of the data (Center)
///
/// Calculates the median of all pairwise averages (x[i] + x[j])/2.
/// More robust than the mean and more efficient than the median.
pub fn center(x: &[f64]) -> Result<f64, &'static str> {
    let n = x.len();
    if n == 0 {
        return Err("Input slice cannot be empty");
    }

    let mut pairwise_averages = Vec::new();
    for i in 0..n {
        for j in i..n {
            pairwise_averages.push((x[i] + x[j]) / 2.0);
        }
    }

    median(&pairwise_averages)
}

/// Estimates data dispersion (Spread)
///
/// Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
/// More robust than standard deviation and more efficient than MAD.
pub fn spread(x: &[f64]) -> Result<f64, &'static str> {
    let n = x.len();
    if n == 0 {
        return Err("Input slice cannot be empty");
    }
    if n == 1 {
        return Ok(0.0);
    }

    let mut pairwise_diffs = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            pairwise_diffs.push((x[i] - x[j]).abs());
        }
    }

    median(&pairwise_diffs)
}

/// Measures the relative dispersion of a sample (Volatility)
///
/// Calculates the ratio of Spread to absolute Center.
/// Robust alternative to the coefficient of variation.
pub fn volatility(x: &[f64]) -> Result<f64, &'static str> {
    let center_val = center(x)?;
    if center_val == 0.0 {
        return Err("Volatility is undefined when Center equals zero");
    }
    let spread_val = spread(x)?;
    Ok(spread_val / center_val.abs())
}

/// Measures precision: the distance between two estimations of independent random samples (Precision)
///
/// Calculated as 2 * Spread / sqrt(n). The interval center ± precision forms a range
/// that probably contains the true center value.
pub fn precision(x: &[f64]) -> Result<f64, &'static str> {
    let n = x.len();
    if n == 0 {
        return Err("Input slice cannot be empty");
    }
    let spread_val = spread(x)?;
    Ok(2.0 * spread_val / (n as f64).sqrt())
}

/// Measures the typical difference between elements of x and y (MedShift)
///
/// Calculates the median of all pairwise differences (x[i] - y[j]).
/// Positive values mean x is typically larger, negative means y is typically larger.
pub fn med_shift(x: &[f64], y: &[f64]) -> Result<f64, &'static str> {
    if x.is_empty() || y.is_empty() {
        return Err("Input slices cannot be empty");
    }

    let mut pairwise_shifts = Vec::new();
    for &xi in x {
        for &yj in y {
            pairwise_shifts.push(xi - yj);
        }
    }

    median(&pairwise_shifts)
}

/// Measures how many times larger x is compared to y (MedRatio)
///
/// Calculates the median of all pairwise ratios (x[i] / y[j]).
/// For example, med_ratio = 1.2 means x is typically 20% larger than y.
pub fn med_ratio(x: &[f64], y: &[f64]) -> Result<f64, &'static str> {
    if x.is_empty() || y.is_empty() {
        return Err("Input slices cannot be empty");
    }

    // Check that all y values are strictly positive
    if y.iter().any(|&val| val <= 0.0) {
        return Err("All values in y must be strictly positive");
    }

    let mut pairwise_ratios = Vec::new();
    for &xi in x {
        for &yj in y {
            pairwise_ratios.push(xi / yj);
        }
    }

    median(&pairwise_ratios)
}

/// Measures the typical variability when considering both samples together (MedSpread)
///
/// Computes the weighted average of individual spreads: (n*Spread(x) + m*Spread(y))/(n+m).
pub fn med_spread(x: &[f64], y: &[f64]) -> Result<f64, &'static str> {
    if x.is_empty() || y.is_empty() {
        return Err("Input slices cannot be empty");
    }

    let n = x.len();
    let m = y.len();
    let spread_x = spread(x)?;
    let spread_y = spread(y)?;

    Ok((n as f64 * spread_x + m as f64 * spread_y) / (n + m) as f64)
}

/// Measures effect size: a normalized absolute difference between x and y (MedDisparity)
///
/// Calculated as MedShift / MedSpread. Robust alternative to Cohen's d.
/// Returns infinity if med_spread is zero.
pub fn med_disparity(x: &[f64], y: &[f64]) -> Result<f64, &'static str> {
    let med_shift_val = med_shift(x, y)?;
    let med_spread_val = med_spread(x, y)?;
    if med_spread_val == 0.0 {
        return Ok(f64::INFINITY);
    }
    Ok(med_shift_val / med_spread_val)
}
