package dev.pragmastat

/**
 * A deterministic random number generator.
 *
 * Rng uses xoshiro256++ internally and guarantees identical output sequences
 * across all Pragmastat language implementations when initialized with the same seed.
 *
 * @example
 * ```kotlin
 * // Create from integer seed
 * val rng = Rng(1729)
 * val value = rng.uniform()
 *
 * // Create from string seed
 * val rng2 = Rng("experiment-1")
 *
 * // Shuffle a list
 * val shuffled = rng.shuffle(listOf(1.0, 2.0, 3.0, 4.0, 5.0))
 *
 * // Sample k elements
 * val sampled = rng.sample(listOf(0, 1, 2, 3, 4, 5, 6, 7, 8, 9), 3)
 * ```
 */
class Rng private constructor(private val inner: Xoshiro256PlusPlus) {

    /**
     * Create a new Rng with system entropy (non-deterministic).
     */
    constructor() : this(System.currentTimeMillis())

    /**
     * Create a new Rng from an integer seed.
     * The same seed always produces the same sequence of random numbers.
     */
    constructor(seed: Long) : this(Xoshiro256PlusPlus(seed.toULong()))

    /**
     * Create a new Rng from a string seed.
     * The string is hashed using FNV-1a to produce a numeric seed.
     */
    constructor(seed: String) : this(Xoshiro256PlusPlus(Fnv1a.hash(seed)))

    // ========================================================================
    // Floating Point Methods
    // ========================================================================

    /**
     * Generate a uniform random Double in [0, 1).
     * Uses 53 bits of precision for the mantissa.
     */
    fun uniform(): Double = inner.uniform()

    /**
     * Generate a uniform random Double in [min, max).
     * Returns min if min >= max.
     */
    fun uniform(min: Double, max: Double): Double = inner.uniform(min, max)

    /**
     * Generate a uniform random Float in [0, 1).
     * Uses 24 bits for Float mantissa precision.
     */
    fun uniformFloat(): Float = inner.uniformFloat()

    /**
     * Generate a uniform random Float in [min, max).
     * Returns min if min >= max.
     */
    fun uniformFloat(min: Float, max: Float): Float = inner.uniformFloat(min, max)

    // ========================================================================
    // Signed Integer Methods
    // ========================================================================

    /**
     * Generate a uniform random Long in [min, max).
     * Returns min if min >= max.
     *
     * Uses modulo reduction which introduces slight bias for ranges that don't
     * evenly divide 2^64. This bias is negligible for statistical simulations
     * but not suitable for cryptographic applications.
     */
    fun uniformLong(min: Long, max: Long): Long = inner.uniformLong(min, max)

    /**
     * Generate a uniform random Int in [min, max).
     * Returns min if min >= max.
     */
    fun uniformInt(min: Int, max: Int): Int = inner.uniformInt(min, max)

    /**
     * Generate a uniform random Short in [min, max).
     * Returns min if min >= max.
     */
    fun uniformShort(min: Short, max: Short): Short = inner.uniformShort(min, max)

    /**
     * Generate a uniform random Byte in [min, max).
     * Returns min if min >= max.
     */
    fun uniformByte(min: Byte, max: Byte): Byte = inner.uniformByte(min, max)

    // ========================================================================
    // Unsigned Integer Methods
    // ========================================================================

    /**
     * Generate a uniform random ULong in [min, max).
     * Returns min if min >= max.
     */
    fun uniformULong(min: ULong, max: ULong): ULong = inner.uniformULong(min, max)

    /**
     * Generate a uniform random UInt in [min, max).
     * Returns min if min >= max.
     */
    fun uniformUInt(min: UInt, max: UInt): UInt = inner.uniformUInt(min, max)

    /**
     * Generate a uniform random UShort in [min, max).
     * Returns min if min >= max.
     */
    fun uniformUShort(min: UShort, max: UShort): UShort = inner.uniformUShort(min, max)

    /**
     * Generate a uniform random UByte in [min, max).
     * Returns min if min >= max.
     */
    fun uniformUByte(min: UByte, max: UByte): UByte = inner.uniformUByte(min, max)

    // ========================================================================
    // Boolean Methods
    // ========================================================================

    /**
     * Generate a uniform random Boolean with P(true) = 0.5.
     */
    fun uniformBool(): Boolean = inner.uniformBool()

    // ========================================================================
    // Collection Methods
    // ========================================================================

    /**
     * Return a shuffled copy of the input list.
     * Uses the Fisher-Yates shuffle algorithm for uniform distribution.
     * The original list is not modified.
     */
    fun <T> shuffle(x: List<T>): List<T> {
        val result = x.toMutableList()
        val n = result.size

        // Fisher-Yates shuffle (backwards)
        for (i in n - 1 downTo 1) {
            val j = uniformLong(0, (i + 1).toLong()).toInt()
            val temp = result[i]
            result[i] = result[j]
            result[j] = temp
        }

        return result
    }

    /**
     * Sample k elements from the input list without replacement.
     * Uses selection sampling to maintain order of first appearance.
     * Returns all elements if k >= x.size.
     *
     * @throws IllegalArgumentException if k is negative.
     */
    fun <T> sample(x: List<T>, k: Int): List<T> {
        require(k >= 0) { "k must be non-negative" }
        val n = x.size
        if (k >= n) return x.toList()

        val result = mutableListOf<T>()
        var remaining = k

        for (i in 0 until n) {
            if (remaining == 0) break
            val available = n - i
            // Probability of selecting this item: remaining / available
            if (uniform() * available < remaining) {
                result.add(x[i])
                remaining--
            }
        }

        return result
    }
}

/**
 * Extension function: shuffle a list using the given Rng.
 */
fun <T> List<T>.shuffle(rng: Rng): List<T> = rng.shuffle(this)

/**
 * Extension function: sample k elements from a list using the given Rng.
 */
fun <T> List<T>.sample(k: Int, rng: Rng): List<T> = rng.sample(this, k)
