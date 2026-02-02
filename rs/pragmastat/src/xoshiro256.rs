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

    /// Generate a uniform f64 in [0, 1)
    /// Uses the upper 53 bits for maximum precision
    #[inline]
    pub fn uniform(&mut self) -> f64 {
        // Standard method: use upper 53 bits for mantissa
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// Generate a uniform f64 in [min, max)
    #[inline]
    pub fn uniform_range(&mut self, min: f64, max: f64) -> f64 {
        if min >= max {
            return min;
        }
        min + (max - min) * self.uniform()
    }

    /// Generate a uniform f32 in [0, 1)
    /// Uses 24 bits for f32 mantissa precision
    #[inline]
    pub fn uniform_f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 * (1.0f32 / (1u64 << 24) as f32)
    }

    /// Generate a uniform f32 in [min, max)
    #[inline]
    pub fn uniform_f32_range(&mut self, min: f32, max: f32) -> f32 {
        if min >= max {
            return min;
        }
        min + (max - min) * self.uniform_f32()
    }

    /// Generate a uniform i64 in [min, max)
    ///
    /// # Panics
    /// Panics if the range `max - min` overflows i64.
    #[inline]
    pub fn uniform_i64(&mut self, min: i64, max: i64) -> i64 {
        if min >= max {
            return min;
        }
        let range =
            max.checked_sub(min)
                .expect("uniform_i64: range overflow (max - min exceeds i64)") as u64;
        min + (self.next_u64() % range) as i64
    }

    /// Generate a uniform i32 in [min, max)
    #[inline]
    pub fn uniform_i32(&mut self, min: i32, max: i32) -> i32 {
        if min >= max {
            return min;
        }
        let range = (max as i64 - min as i64) as u64;
        min + (self.next_u64() % range) as i32
    }

    /// Generate a uniform i16 in [min, max)
    #[inline]
    pub fn uniform_i16(&mut self, min: i16, max: i16) -> i16 {
        if min >= max {
            return min;
        }
        let range = (max as i32 - min as i32) as u64;
        min + (self.next_u64() % range) as i16
    }

    /// Generate a uniform i8 in [min, max)
    #[inline]
    pub fn uniform_i8(&mut self, min: i8, max: i8) -> i8 {
        if min >= max {
            return min;
        }
        let range = (max as i16 - min as i16) as u64;
        min + (self.next_u64() % range) as i8
    }

    /// Generate a uniform isize in [min, max)
    #[inline]
    pub fn uniform_isize(&mut self, min: isize, max: isize) -> isize {
        if min >= max {
            return min;
        }
        let range = (max as i128 - min as i128) as u64;
        min + (self.next_u64() % range) as isize
    }

    /// Generate a uniform u64 in [min, max)
    #[inline]
    pub fn uniform_u64(&mut self, min: u64, max: u64) -> u64 {
        if min >= max {
            return min;
        }
        let range = max - min;
        min + self.next_u64() % range
    }

    /// Generate a uniform u32 in [min, max)
    #[inline]
    pub fn uniform_u32(&mut self, min: u32, max: u32) -> u32 {
        if min >= max {
            return min;
        }
        let range = (max - min) as u64;
        min + (self.next_u64() % range) as u32
    }

    /// Generate a uniform u16 in [min, max)
    #[inline]
    pub fn uniform_u16(&mut self, min: u16, max: u16) -> u16 {
        if min >= max {
            return min;
        }
        let range = (max - min) as u64;
        min + (self.next_u64() % range) as u16
    }

    /// Generate a uniform u8 in [min, max)
    #[inline]
    pub fn uniform_u8(&mut self, min: u8, max: u8) -> u8 {
        if min >= max {
            return min;
        }
        let range = (max - min) as u64;
        min + (self.next_u64() % range) as u8
    }

    /// Generate a uniform usize in [min, max)
    #[inline]
    pub fn uniform_usize(&mut self, min: usize, max: usize) -> usize {
        if min >= max {
            return min;
        }
        let range = (max - min) as u64;
        min + (self.next_u64() % range) as usize
    }

    /// Generate a uniform boolean with P(true) = 0.5
    #[inline]
    pub fn uniform_bool(&mut self) -> bool {
        self.uniform() < 0.5
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
    fn uniform_in_range() {
        let mut rng = Xoshiro256PlusPlus::new(42);

        for _ in 0..1000 {
            let v = rng.uniform();
            assert!(v >= 0.0 && v < 1.0);
        }
    }

    #[test]
    fn uniform_range_bounds() {
        let mut rng = Xoshiro256PlusPlus::new(42);

        for _ in 0..1000 {
            let v = rng.uniform_range(-5.0, 5.0);
            assert!(v >= -5.0 && v < 5.0);
        }
    }

    #[test]
    fn uniform_f32_in_range() {
        let mut rng = Xoshiro256PlusPlus::new(42);

        for _ in 0..1000 {
            let v = rng.uniform_f32();
            assert!(v >= 0.0 && v < 1.0);
        }
    }

    #[test]
    fn uniform_f32_range_bounds() {
        let mut rng = Xoshiro256PlusPlus::new(42);

        for _ in 0..1000 {
            let v = rng.uniform_f32_range(-5.0, 5.0);
            assert!(v >= -5.0 && v < 5.0);
        }
    }

    #[test]
    fn uniform_i64_bounds() {
        let mut rng = Xoshiro256PlusPlus::new(42);

        for _ in 0..1000 {
            let v = rng.uniform_i64(10, 20);
            assert!(v >= 10 && v < 20);
        }
    }

    #[test]
    fn uniform_i64_negative() {
        let mut rng = Xoshiro256PlusPlus::new(42);

        for _ in 0..1000 {
            let v = rng.uniform_i64(-10, 10);
            assert!(v >= -10 && v < 10);
        }
    }

    #[test]
    fn uniform_i32_bounds() {
        let mut rng = Xoshiro256PlusPlus::new(42);

        for _ in 0..1000 {
            let v = rng.uniform_i32(-100, 100);
            assert!(v >= -100 && v < 100);
        }
    }

    #[test]
    fn uniform_i16_bounds() {
        let mut rng = Xoshiro256PlusPlus::new(42);

        for _ in 0..1000 {
            let v = rng.uniform_i16(-100, 100);
            assert!(v >= -100 && v < 100);
        }
    }

    #[test]
    fn uniform_i8_bounds() {
        let mut rng = Xoshiro256PlusPlus::new(42);

        for _ in 0..1000 {
            let v = rng.uniform_i8(-50, 50);
            assert!(v >= -50 && v < 50);
        }
    }

    #[test]
    fn uniform_u64_bounds() {
        let mut rng = Xoshiro256PlusPlus::new(42);

        for _ in 0..1000 {
            let v = rng.uniform_u64(10, 100);
            assert!(v >= 10 && v < 100);
        }
    }

    #[test]
    fn uniform_u32_bounds() {
        let mut rng = Xoshiro256PlusPlus::new(42);

        for _ in 0..1000 {
            let v = rng.uniform_u32(10, 100);
            assert!(v >= 10 && v < 100);
        }
    }

    #[test]
    fn uniform_u16_bounds() {
        let mut rng = Xoshiro256PlusPlus::new(42);

        for _ in 0..1000 {
            let v = rng.uniform_u16(10, 100);
            assert!(v >= 10 && v < 100);
        }
    }

    #[test]
    fn uniform_u8_bounds() {
        let mut rng = Xoshiro256PlusPlus::new(42);

        for _ in 0..1000 {
            let v = rng.uniform_u8(10, 100);
            assert!(v >= 10 && v < 100);
        }
    }

    #[test]
    fn uniform_bool_distribution() {
        let mut rng = Xoshiro256PlusPlus::new(42);
        let count: usize = (0..10000).filter(|_| rng.uniform_bool()).count();
        // Should be approximately 50% true
        assert!(count > 4500 && count < 5500);
    }
}
