//! Distribution trait definition.

use crate::Rng;

/// A distribution: three views of one object, plus the ability to draw from it.
///
/// `density_at` is how tightly probability is packed near a value, `cumulative_at` is the
/// probability of not exceeding it, and `quantile_at` inverts that. Each family supplies all three
/// out of the toolkit's own primitives, so seven implementations return identical bits.
///
/// `sample` stays separate rather than being defined as `quantile_at(uniform_f64())`. Two of the
/// families draw by a route that is not inverse transform sampling, and changing that would change
/// every seeded stream in every language, which is a decision of its own and not this one.
///
/// # Why the names carry `_at`
///
/// A reconstruction is also a distribution, and it publishes `density`, `cumulative` and
/// `quantile` as *vector* queries that validate their input and return a `Result`. One type would
/// then carry both spellings of each name. Rust would resolve `d.quantile(..)` to the inherent
/// method and quietly hide the trait one, but four of the seven ports have no such rule, and a
/// method whose meaning depends on which of two identically named things the compiler picks is a
/// trap in any language. `_at` reads as what it is: the value *at* a point.
pub trait Distribution {
    /// Generate a single sample from this distribution.
    fn sample(&self, rng: &mut Rng) -> f64;

    /// Probability density at `x`. Zero outside the support.
    fn density_at(&self, x: f64) -> f64;

    /// Probability of not exceeding `x`. Zero below the support, one above it.
    fn cumulative_at(&self, x: f64) -> f64;

    /// The value below which a fraction `p` of the distribution lies.
    ///
    /// A `p` outside `(0, 1)` returns the endpoints of the support, which are infinities for the
    /// unbounded families.
    fn quantile_at(&self, p: f64) -> f64;

    /// Generate multiple samples from this distribution.
    fn samples(&self, rng: &mut Rng, count: usize) -> Vec<f64> {
        (0..count).map(|_| self.sample(rng)).collect()
    }
}
