//! SplitMix64 PRNG for seed expansion
//! Reference: https://prng.di.unimi.it/splitmix64.c

pub(crate) struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_sequence() {
        let mut rng = SplitMix64::new(42);
        let values: Vec<u64> = (0..5).map(|_| rng.next()).collect();

        // Same seed should always produce same sequence
        let mut rng2 = SplitMix64::new(42);
        let values2: Vec<u64> = (0..5).map(|_| rng2.next()).collect();

        assert_eq!(values, values2);
    }

    #[test]
    fn different_seeds_different_sequences() {
        let mut rng1 = SplitMix64::new(1);
        let mut rng2 = SplitMix64::new(2);

        assert_ne!(rng1.next(), rng2.next());
    }
}
