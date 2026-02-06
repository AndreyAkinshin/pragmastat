package dev.pragmastat

/**
 * SplitMix64 PRNG for seed expansion.
 */
internal class SplitMix64(private var state: ULong) {
    fun next(): ULong {
        state += 0x9e3779b97f4a7c15UL
        var z = state
        z = (z xor (z shr 30)) * 0xbf58476d1ce4e5b9UL
        z = (z xor (z shr 27)) * 0x94d049bb133111ebUL
        return z xor (z shr 31)
    }
}

/**
 * xoshiro256++ PRNG.
 * Reference: https://prng.di.unimi.it/xoshiro256plusplus.c
 *
 * This is the jump-free version of the algorithm. It passes BigCrush
 * and is used by .NET 6+, Julia, and Rust's rand crate.
 */
internal class Xoshiro256PlusPlus(seed: ULong) {
    private var s0: ULong
    private var s1: ULong
    private var s2: ULong
    private var s3: ULong

    init {
        val sm = SplitMix64(seed)
        s0 = sm.next()
        s1 = sm.next()
        s2 = sm.next()
        s3 = sm.next()
    }

    private fun rotl(x: ULong, k: Int): ULong = (x shl k) or (x shr (64 - k))

    fun nextU64(): ULong {
        val result = rotl(s0 + s3, 23) + s0

        val t = s1 shl 17

        s2 = s2 xor s0
        s3 = s3 xor s1
        s1 = s1 xor s2
        s0 = s0 xor s3

        s2 = s2 xor t
        s3 = rotl(s3, 45)

        return result
    }

    // ========================================================================
    // Floating Point Methods
    // ========================================================================

    fun uniform(): Double {
        // Use upper 53 bits for maximum precision
        return (nextU64() shr 11).toDouble() * (1.0 / (1UL shl 53).toDouble())
    }

    fun uniform(min: Double, max: Double): Double {
        if (min >= max) return min
        return min + (max - min) * uniform()
    }

    fun uniformFloat(): Float {
        // Use 24 bits for float mantissa precision
        return (nextU64() shr 40).toFloat() * (1.0f / (1UL shl 24).toFloat())
    }

    fun uniformFloat(min: Float, max: Float): Float {
        if (min >= max) return min
        return min + (max - min) * uniformFloat()
    }

    // ========================================================================
    // Signed Integer Methods
    // ========================================================================

    /**
     * Generate a uniform Long in [min, max).
     * @throws ArithmeticException if max - min overflows.
     */
    fun uniformLong(min: Long, max: Long): Long {
        if (min >= max) return min
        val range = Math.subtractExact(max, min).toULong()
        return min + (nextU64() % range).toLong()
    }

    fun uniformInt(min: Int, max: Int): Int {
        if (min >= max) return min
        val range = (max.toLong() - min.toLong()).toULong()
        return min + (nextU64() % range).toInt()
    }

    fun uniformShort(min: Short, max: Short): Short {
        if (min >= max) return min
        val range = (max.toInt() - min.toInt()).toULong()
        return (min + (nextU64() % range).toInt()).toShort()
    }

    fun uniformByte(min: Byte, max: Byte): Byte {
        if (min >= max) return min
        val range = (max.toInt() - min.toInt()).toULong()
        return (min + (nextU64() % range).toInt()).toByte()
    }

    // ========================================================================
    // Unsigned Integer Methods
    // ========================================================================

    fun uniformULong(min: ULong, max: ULong): ULong {
        if (min >= max) return min
        val range = max - min
        return min + nextU64() % range
    }

    fun uniformUInt(min: UInt, max: UInt): UInt {
        if (min >= max) return min
        val range = (max - min).toULong()
        return min + (nextU64() % range).toUInt()
    }

    fun uniformUShort(min: UShort, max: UShort): UShort {
        if (min >= max) return min
        val range = (max - min).toULong()
        return (min + (nextU64() % range).toUInt()).toUShort()
    }

    fun uniformUByte(min: UByte, max: UByte): UByte {
        if (min >= max) return min
        val range = (max - min).toULong()
        return (min + (nextU64() % range).toUInt()).toUByte()
    }

    // ========================================================================
    // Boolean Methods
    // ========================================================================

    fun uniformBool(): Boolean {
        return uniform() < 0.5
    }
}

/**
 * FNV-1a hash constants and implementation.
 */
internal object Fnv1a {
    private const val OFFSET_BASIS = 0xcbf29ce484222325UL
    private const val PRIME = 0x00000100000001b3UL

    /**
     * Compute FNV-1a 64-bit hash of a string.
     */
    fun hash(s: String): ULong {
        var hash = OFFSET_BASIS
        for (byte in s.encodeToByteArray()) {
            hash = hash xor byte.toUByte().toULong()
            hash *= PRIME
        }
        return hash
    }

    /**
     * Compute FNV-1a 64-bit hash of double values.
     */
    fun hashDoubles(values: List<Double>): ULong {
        var hash = OFFSET_BASIS
        for (v in values) {
            val bits = v.toRawBits().toULong()
            for (i in 0 until 8) {
                hash = hash xor ((bits shr (i * 8)) and 0xffUL)
                hash *= PRIME
            }
        }
        return hash
    }
}

/**
 * Derive a deterministic seed from input values using FNV-1a hash.
 * Ensures same input always produces same random sequence.
 */
internal fun deriveSeed(values: List<Double>): Long {
    return Fnv1a.hashDoubles(values).toLong()
}
