//! xoshiro256++ PRNG
//! Reference: https://prng.di.unimi.it/xoshiro256plusplus.c
//!
//! This is the jump-free version of the algorithm. It passes BigCrush
//! and is used by .NET 6+, Julia, and Rust's rand crate.

use crate::splitmix64::SplitMix64;

pub(crate) struct Xoshiro256PlusPlus {
    state: [u64; 4],
}

impl Xoshiro256PlusPlus {
    /// Create a new generator from a 64-bit seed
    /// Uses SplitMix64 to expand the seed into the full state
    pub fn new(seed: u64) -> Self {
        let mut sm = SplitMix64::new(seed);
        Self {
            state: [sm.next(), sm.next(), sm.next(), sm.next()],
        }
    }

    /// Generate the next 64-bit random value
    pub fn next_u64(&mut self) -> u64 {
        let result = (self.state[0].wrapping_add(self.state[3]))
            .rotate_left(23)
            .wrapping_add(self.state[0]);

        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);

        result
    }

    /// Generate a uniform float in [0, 1)
    /// Uses the upper 53 bits for maximum precision
    #[inline]
    pub fn uniform(&mut self) -> f64 {
        // Standard method: use upper 53 bits for mantissa
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// Generate a uniform integer in [min, max)
    ///
    /// # Panics
    /// Panics if the range `max - min` overflows i64.
    #[inline]
    pub fn uniform_int(&mut self, min: i64, max: i64) -> i64 {
        if min >= max {
            return min;
        }
        let range =
            max.checked_sub(min)
                .expect("uniform_int: range overflow (max - min exceeds i64)") as u64;
        min + (self.next_u64() % range) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_sequence() {
        let mut rng1 = Xoshiro256PlusPlus::new(42);
        let mut rng2 = Xoshiro256PlusPlus::new(42);

        for _ in 0..100 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn uniform_range() {
        let mut rng = Xoshiro256PlusPlus::new(42);

        for _ in 0..1000 {
            let v = rng.uniform();
            assert!(v >= 0.0 && v < 1.0);
        }
    }

    #[test]
    fn uniform_int_range() {
        let mut rng = Xoshiro256PlusPlus::new(42);

        for _ in 0..1000 {
            let v = rng.uniform_int(10, 20);
            assert!(v >= 10 && v < 20);
        }
    }

    #[test]
    fn uniform_int_negative_range() {
        let mut rng = Xoshiro256PlusPlus::new(42);

        for _ in 0..1000 {
            let v = rng.uniform_int(-10, 10);
            assert!(v >= -10 && v < 10);
        }
    }
}
