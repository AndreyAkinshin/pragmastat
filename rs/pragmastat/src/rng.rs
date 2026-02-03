//! Deterministic random number generator for cross-language reproducibility
//!
//! The `Rng` struct provides a deterministic PRNG based on xoshiro256++ that
//! produces identical sequences across all Pragmastat language implementations.

#![allow(deprecated)]

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
/// // Create from string seed
/// let mut rng = Rng::from_string("demo-uniform");
/// let value = rng.uniform();
///
/// // Shuffle a vector
/// let mut rng = Rng::from_string("demo-shuffle");
/// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let shuffled = rng.shuffle(&data);
///
/// // Sample k elements
/// let mut rng = Rng::from_string("demo-sample");
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

    // ========================================================================
    // Floating Point Methods
    // ========================================================================

    /// Generate a uniform random f64 in [0, 1)
    ///
    /// Uses 53 bits of precision for the mantissa.
    ///
    /// # Examples
    ///
    /// ```
    /// use pragmastat::Rng;
    ///
    /// let mut rng = Rng::from_string("demo-uniform");
    /// let value = rng.uniform();
    /// assert!(value >= 0.0 && value < 1.0);
    /// ```
    #[inline]
    pub fn uniform(&mut self) -> f64 {
        self.inner.uniform()
    }

    /// Generate a uniform random f64 in [min, max)
    ///
    /// Returns `min` if `min >= max`.
    ///
    /// # Examples
    ///
    /// ```
    /// use pragmastat::Rng;
    ///
    /// let mut rng = Rng::from_string("demo-uniform");
    /// let value = rng.uniform_range(-5.0, 5.0);
    /// assert!(value >= -5.0 && value < 5.0);
    /// ```
    #[inline]
    pub fn uniform_range(&mut self, min: f64, max: f64) -> f64 {
        self.inner.uniform_range(min, max)
    }

    /// Generate a uniform random f32 in [0, 1)
    ///
    /// Uses 24 bits for f32 mantissa precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use pragmastat::Rng;
    ///
    /// let mut rng = Rng::from_string("demo-uniform");
    /// let value = rng.uniform_f32();
    /// assert!(value >= 0.0 && value < 1.0);
    /// ```
    #[inline]
    pub fn uniform_f32(&mut self) -> f32 {
        self.inner.uniform_f32()
    }

    /// Generate a uniform random f32 in [min, max)
    ///
    /// Returns `min` if `min >= max`.
    ///
    /// # Examples
    ///
    /// ```
    /// use pragmastat::Rng;
    ///
    /// let mut rng = Rng::from_string("demo-uniform");
    /// let value = rng.uniform_f32_range(-5.0, 5.0);
    /// assert!(value >= -5.0 && value < 5.0);
    /// ```
    #[inline]
    pub fn uniform_f32_range(&mut self, min: f32, max: f32) -> f32 {
        self.inner.uniform_f32_range(min, max)
    }

    // ========================================================================
    // Signed Integer Methods
    // ========================================================================

    /// Generate a uniform random i64 in [min, max)
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
    /// let mut rng = Rng::from_string("demo-uniform");
    /// let value = rng.uniform_i64(0, 100);
    /// assert!(value >= 0 && value < 100);
    /// ```
    #[inline]
    pub fn uniform_i64(&mut self, min: i64, max: i64) -> i64 {
        self.inner.uniform_i64(min, max)
    }

    /// Generate a uniform random i32 in [min, max)
    ///
    /// Returns `min` if `min >= max`.
    #[inline]
    pub fn uniform_i32(&mut self, min: i32, max: i32) -> i32 {
        self.inner.uniform_i32(min, max)
    }

    /// Generate a uniform random i16 in [min, max)
    ///
    /// Returns `min` if `min >= max`.
    #[inline]
    pub fn uniform_i16(&mut self, min: i16, max: i16) -> i16 {
        self.inner.uniform_i16(min, max)
    }

    /// Generate a uniform random i8 in [min, max)
    ///
    /// Returns `min` if `min >= max`.
    #[inline]
    pub fn uniform_i8(&mut self, min: i8, max: i8) -> i8 {
        self.inner.uniform_i8(min, max)
    }

    /// Generate a uniform random isize in [min, max)
    ///
    /// Returns `min` if `min >= max`.
    #[inline]
    pub fn uniform_isize(&mut self, min: isize, max: isize) -> isize {
        self.inner.uniform_isize(min, max)
    }

    // ========================================================================
    // Unsigned Integer Methods
    // ========================================================================

    /// Generate a uniform random u64 in [min, max)
    ///
    /// Returns `min` if `min >= max`.
    #[inline]
    pub fn uniform_u64(&mut self, min: u64, max: u64) -> u64 {
        self.inner.uniform_u64(min, max)
    }

    /// Generate a uniform random u32 in [min, max)
    ///
    /// Returns `min` if `min >= max`.
    #[inline]
    pub fn uniform_u32(&mut self, min: u32, max: u32) -> u32 {
        self.inner.uniform_u32(min, max)
    }

    /// Generate a uniform random u16 in [min, max)
    ///
    /// Returns `min` if `min >= max`.
    #[inline]
    pub fn uniform_u16(&mut self, min: u16, max: u16) -> u16 {
        self.inner.uniform_u16(min, max)
    }

    /// Generate a uniform random u8 in [min, max)
    ///
    /// Returns `min` if `min >= max`.
    #[inline]
    pub fn uniform_u8(&mut self, min: u8, max: u8) -> u8 {
        self.inner.uniform_u8(min, max)
    }

    /// Generate a uniform random usize in [min, max)
    ///
    /// Returns `min` if `min >= max`.
    #[inline]
    pub fn uniform_usize(&mut self, min: usize, max: usize) -> usize {
        self.inner.uniform_usize(min, max)
    }

    // ========================================================================
    // Boolean Methods
    // ========================================================================

    /// Generate a uniform random boolean with P(true) = 0.5
    ///
    /// # Examples
    ///
    /// ```
    /// use pragmastat::Rng;
    ///
    /// let mut rng = Rng::from_string("demo-uniform");
    /// let coin_flip = rng.uniform_bool();
    /// ```
    #[inline]
    pub fn uniform_bool(&mut self) -> bool {
        self.inner.uniform_bool()
    }

    // ========================================================================
    // Deprecated Methods
    // ========================================================================

    /// Generate a uniform random integer in [min, max)
    ///
    /// Returns `min` if `min >= max`.
    #[deprecated(since = "5.2.0", note = "use uniform_i64 instead")]
    #[inline]
    pub fn uniform_int(&mut self, min: i64, max: i64) -> i64 {
        self.inner.uniform_i64(min, max)
    }

    // ========================================================================
    // Collection Methods
    // ========================================================================

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
    /// let mut rng = Rng::from_string("demo-shuffle");
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
            let j = self.uniform_i64(0, (i + 1) as i64) as usize;
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
    /// let mut rng = Rng::from_string("demo-sample");
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
    fn uniform_in_range() {
        let mut rng = Rng::from_string("test-uniform");

        for _ in 0..10000 {
            let v = rng.uniform();
            assert!(v >= 0.0 && v < 1.0);
        }
    }

    #[test]
    fn uniform_range_bounds() {
        let mut rng = Rng::from_string("test-uniform-range");

        for _ in 0..10000 {
            let v = rng.uniform_range(-10.0, 10.0);
            assert!(v >= -10.0 && v < 10.0);
        }
    }

    #[test]
    fn uniform_f32_in_range() {
        let mut rng = Rng::from_string("test-uniform-f32");

        for _ in 0..10000 {
            let v = rng.uniform_f32();
            assert!(v >= 0.0 && v < 1.0);
        }
    }

    #[test]
    fn uniform_f32_range_bounds() {
        let mut rng = Rng::from_string("test-uniform-f32-range");

        for _ in 0..10000 {
            let v = rng.uniform_f32_range(-10.0, 10.0);
            assert!(v >= -10.0 && v < 10.0);
        }
    }

    #[test]
    fn uniform_i64_bounds() {
        let mut rng = Rng::from_string("test-uniform-i64");

        for _ in 0..10000 {
            let v = rng.uniform_i64(0, 100);
            assert!(v >= 0 && v < 100);
        }
    }

    #[test]
    fn uniform_i32_bounds() {
        let mut rng = Rng::from_string("test-uniform-i32");

        for _ in 0..10000 {
            let v = rng.uniform_i32(-500, 500);
            assert!(v >= -500 && v < 500);
        }
    }

    #[test]
    fn uniform_i16_bounds() {
        let mut rng = Rng::from_string("test-uniform-i16");

        for _ in 0..10000 {
            let v = rng.uniform_i16(-100, 100);
            assert!(v >= -100 && v < 100);
        }
    }

    #[test]
    fn uniform_i8_bounds() {
        let mut rng = Rng::from_string("test-uniform-i8");

        for _ in 0..10000 {
            let v = rng.uniform_i8(-50, 50);
            assert!(v >= -50 && v < 50);
        }
    }

    #[test]
    fn uniform_u64_bounds() {
        let mut rng = Rng::from_string("test-uniform-u64");

        for _ in 0..10000 {
            let v = rng.uniform_u64(10, 1000);
            assert!(v >= 10 && v < 1000);
        }
    }

    #[test]
    fn uniform_u32_bounds() {
        let mut rng = Rng::from_string("test-uniform-u32");

        for _ in 0..10000 {
            let v = rng.uniform_u32(10, 1000);
            assert!(v >= 10 && v < 1000);
        }
    }

    #[test]
    fn uniform_u16_bounds() {
        let mut rng = Rng::from_string("test-uniform-u16");

        for _ in 0..10000 {
            let v = rng.uniform_u16(10, 100);
            assert!(v >= 10 && v < 100);
        }
    }

    #[test]
    fn uniform_u8_bounds() {
        let mut rng = Rng::from_string("test-uniform-u8");

        for _ in 0..10000 {
            let v = rng.uniform_u8(10, 100);
            assert!(v >= 10 && v < 100);
        }
    }

    #[test]
    fn uniform_bool_distribution() {
        let mut rng = Rng::from_string("test-uniform-bool");
        let count: usize = (0..10000).filter(|_| rng.uniform_bool()).count();
        // Should be approximately 50% true
        assert!(count > 4500 && count < 5500);
    }

    #[test]
    fn shuffle_preserves_elements() {
        let mut rng = Rng::from_string("test-shuffle");
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
        let mut rng = Rng::from_string("test-sample");
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
        let mut rng = Rng::from_string("test-sample-edge");
        let data: Vec<f64> = vec![1.0, 2.0, 3.0];
        let sampled = rng.sample(&data, 10);

        assert_eq!(sampled, data);
    }
}
