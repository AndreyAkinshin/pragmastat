//! FNV-1a hash algorithm (64-bit)
//! Reference: http://www.isthe.com/chongo/tech/comp/fnv/

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x00000100000001b3;

/// Compute FNV-1a 64-bit hash of a string
pub(crate) fn fnv1a_hash(s: &str) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;
    for byte in s.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// Compute FNV-1a 64-bit hash of an f64 slice, returning it as i64 for use as an RNG seed.
pub(crate) fn hash_f64_slice(values: &[f64]) -> i64 {
    let mut hash = FNV_OFFSET_BASIS;
    for v in values {
        let bits = v.to_bits();
        for i in 0..8 {
            hash ^= (bits >> (i * 8)) & 0xff;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
    }
    hash as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        // Empty string should return offset basis
        assert_eq!(fnv1a_hash(""), FNV_OFFSET_BASIS);
    }

    #[test]
    fn known_values() {
        // These are well-known FNV-1a test vectors
        assert_eq!(fnv1a_hash("a"), 0xaf63dc4c8601ec8c);
        assert_eq!(fnv1a_hash("ab"), 0x089c4407b545986a);
        assert_eq!(fnv1a_hash("abc"), 0xe71fa2190541574b);
    }

    #[test]
    fn deterministic() {
        let hash1 = fnv1a_hash("test");
        let hash2 = fnv1a_hash("test");
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn different_strings_different_hashes() {
        assert_ne!(fnv1a_hash("hello"), fnv1a_hash("world"));
    }
}
