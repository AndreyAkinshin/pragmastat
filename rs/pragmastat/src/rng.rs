//! Deterministic random number generator for cross-language reproducibility
//!
//! The `Rng` struct provides a deterministic PRNG based on xoshiro256++ that
//! produces identical sequences across all Pragmastat language implementations.

use crate::fnv1a::fnv1a_hash;
use crate::xoshiro256::Xoshiro256PlusPlus;

/// A deterministic random number generator.
///
/// `Rng` uses xoshiro256++ internally and guarantees identical output sequences
/// across all Pragmastat language implementations when initialized with the same seed.
///
/// # Examples
///
/// ```
/// use pragmastat::Rng;
///
/// // Create from integer seed
/// let mut rng = Rng::from_seed(1729);
/// let value = rng.uniform();
///
/// // Create from string seed
/// let mut rng = Rng::from_string("experiment-1");
///
/// // Shuffle a vector
/// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let shuffled = rng.shuffle(&data);
///
/// // Sample k elements
/// let sampled = rng.sample(&data, 3);
/// ```
pub struct Rng {
    inner: Xoshiro256PlusPlus,
}

impl Default for Rng {
    fn default() -> Self {
        Self::new()
    }
}

impl Rng {
    /// Create a new Rng with system entropy
    ///
    /// Note: This is non-deterministic and should only be used when
    /// reproducibility is not required.
    pub fn new() -> Self {
        // Use system time as entropy source
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        Self::from_seed(seed as i64)
    }

    /// Create a new Rng from an integer seed
    ///
    /// The same seed always produces the same sequence of random numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use pragmastat::Rng;
    ///
    /// let mut rng = Rng::from_seed(1729);
    /// let v1 = rng.uniform();
    ///
    /// // Same seed produces same sequence
    /// let mut rng2 = Rng::from_seed(1729);
    /// let v2 = rng2.uniform();
    /// assert_eq!(v1, v2);
    /// ```
    pub fn from_seed(seed: i64) -> Self {
        Self {
            inner: Xoshiro256PlusPlus::new(seed as u64),
        }
    }

    /// Create a new Rng from a string seed
    ///
    /// The string is hashed using FNV-1a to produce a numeric seed.
    /// Useful for naming experiments or configurations.
    ///
    /// # Examples
    ///
    /// ```
    /// use pragmastat::Rng;
    ///
    /// let mut rng = Rng::from_string("experiment-alpha");
    /// ```
    pub fn from_string(seed: &str) -> Self {
        let hash = fnv1a_hash(seed);
        Self {
            inner: Xoshiro256PlusPlus::new(hash),
        }
    }

    /// Generate a uniform random float in [0, 1)
    ///
    /// Uses 53 bits of precision for the mantissa.
    ///
    /// # Examples
    ///
    /// ```
    /// use pragmastat::Rng;
    ///
    /// let mut rng = Rng::from_seed(1729);
    /// let value = rng.uniform();
    /// assert!(value >= 0.0 && value < 1.0);
    /// ```
    #[inline]
    pub fn uniform(&mut self) -> f64 {
        self.inner.uniform()
    }

    /// Generate a uniform random integer in [min, max)
    ///
    /// Returns `min` if `min >= max`.
    ///
    /// # Note on Distribution
    ///
    /// Uses modulo reduction which introduces slight bias for ranges that don't
    /// evenly divide 2^64. This bias is negligible for statistical simulations
    /// but not suitable for cryptographic applications.
    ///
    /// # Examples
    ///
    /// ```
    /// use pragmastat::Rng;
    ///
    /// let mut rng = Rng::from_seed(1729);
    /// let value = rng.uniform_int(0, 100);
    /// assert!(value >= 0 && value < 100);
    /// ```
    #[inline]
    pub fn uniform_int(&mut self, min: i64, max: i64) -> i64 {
        self.inner.uniform_int(min, max)
    }

    /// Return a shuffled copy of the input slice
    ///
    /// Uses the Fisher-Yates shuffle algorithm for uniform distribution.
    /// The original slice is not modified.
    ///
    /// # Examples
    ///
    /// ```
    /// use pragmastat::Rng;
    ///
    /// let mut rng = Rng::from_seed(1729);
    /// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let shuffled = rng.shuffle(&data);
    ///
    /// // Original unchanged
    /// assert_eq!(data, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    /// // Shuffled has same elements
    /// assert_eq!(shuffled.len(), data.len());
    /// ```
    pub fn shuffle<T: Clone>(&mut self, x: &[T]) -> Vec<T> {
        let mut result: Vec<T> = x.to_vec();
        let n = result.len();

        // Fisher-Yates shuffle (inside-out variant, backwards)
        for i in (1..n).rev() {
            let j = self.uniform_int(0, (i + 1) as i64) as usize;
            result.swap(i, j);
        }

        result
    }

    /// Sample k elements from the input slice without replacement
    ///
    /// Uses selection sampling to maintain order of first appearance.
    /// Returns up to `k` elements; if `k >= x.len()`, returns all elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use pragmastat::Rng;
    ///
    /// let mut rng = Rng::from_seed(1729);
    /// let data = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    /// let sampled = rng.sample(&data, 3);
    ///
    /// assert_eq!(sampled.len(), 3);
    /// ```
    pub fn sample<T: Clone>(&mut self, x: &[T], k: usize) -> Vec<T> {
        let n = x.len();
        if k >= n {
            return x.to_vec();
        }

        let mut result = Vec::with_capacity(k);
        let mut remaining = k;

        for (i, item) in x.iter().enumerate() {
            let available = n - i;
            // Probability of selecting this item: remaining / available
            if (self.uniform() * available as f64) < remaining as f64 {
                result.push(item.clone());
                remaining -= 1;
                if remaining == 0 {
                    break;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_seed_deterministic() {
        let mut rng1 = Rng::from_seed(1729);
        let mut rng2 = Rng::from_seed(1729);

        for _ in 0..100 {
            assert_eq!(rng1.uniform(), rng2.uniform());
        }
    }

    #[test]
    fn from_string_deterministic() {
        let mut rng1 = Rng::from_string("test");
        let mut rng2 = Rng::from_string("test");

        for _ in 0..100 {
            assert_eq!(rng1.uniform(), rng2.uniform());
        }
    }

    #[test]
    fn uniform_range() {
        let mut rng = Rng::from_seed(1729);

        for _ in 0..10000 {
            let v = rng.uniform();
            assert!(v >= 0.0 && v < 1.0);
        }
    }

    #[test]
    fn uniform_int_range() {
        let mut rng = Rng::from_seed(1729);

        for _ in 0..10000 {
            let v = rng.uniform_int(0, 100);
            assert!(v >= 0 && v < 100);
        }
    }

    #[test]
    fn shuffle_preserves_elements() {
        let mut rng = Rng::from_seed(1729);
        let data: Vec<i32> = (0..10).collect();
        let shuffled = rng.shuffle(&data);

        assert_eq!(shuffled.len(), data.len());
        let mut sorted = shuffled.clone();
        sorted.sort();
        assert_eq!(sorted, data);
    }

    #[test]
    fn shuffle_deterministic() {
        let data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let mut rng1 = Rng::from_seed(1729);
        let shuffled1 = rng1.shuffle(&data);

        let mut rng2 = Rng::from_seed(1729);
        let shuffled2 = rng2.shuffle(&data);

        assert_eq!(shuffled1, shuffled2);
    }

    #[test]
    fn sample_correct_size() {
        let mut rng = Rng::from_seed(1729);
        let data: Vec<f64> = (0..10).map(|i| i as f64).collect();
        let sampled = rng.sample(&data, 3);

        assert_eq!(sampled.len(), 3);
    }

    #[test]
    fn sample_deterministic() {
        let data: Vec<f64> = (0..10).map(|i| i as f64).collect();

        let mut rng1 = Rng::from_seed(1729);
        let sampled1 = rng1.sample(&data, 3);

        let mut rng2 = Rng::from_seed(1729);
        let sampled2 = rng2.sample(&data, 3);

        assert_eq!(sampled1, sampled2);
    }

    #[test]
    fn sample_k_greater_than_n() {
        let mut rng = Rng::from_seed(1729);
        let data: Vec<f64> = vec![1.0, 2.0, 3.0];
        let sampled = rng.sample(&data, 10);

        assert_eq!(sampled, data);
    }
}
