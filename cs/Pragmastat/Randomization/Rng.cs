using System;
using System.Collections.Generic;

#pragma warning disable CS0618 // Type or member is obsolete

namespace Pragmastat.Randomization;

/// <summary>
/// A deterministic random number generator.
/// </summary>
/// <remarks>
/// Rng uses xoshiro256++ internally and guarantees identical output sequences
/// across all Pragmastat language implementations when initialized with the same seed.
/// </remarks>
public sealed class Rng
{
  private readonly Xoshiro256PlusPlus _inner;

  /// <summary>
  /// Create a new Rng with system entropy (non-deterministic).
  /// </summary>
  public Rng()
      : this(DateTime.UtcNow.Ticks)
  {
  }

  /// <summary>
  /// Create a new Rng from an integer seed.
  /// The same seed always produces the same sequence of random numbers.
  /// </summary>
  /// <param name="seed">The seed value.</param>
  public Rng(long seed)
  {
    _inner = new Xoshiro256PlusPlus((ulong)seed);
  }

  /// <summary>
  /// Create a new Rng from a string seed.
  /// The string is hashed using FNV-1a to produce a numeric seed.
  /// </summary>
  /// <param name="seed">The string seed.</param>
  /// <exception cref="ArgumentNullException">Thrown if seed is null.</exception>
  public Rng(string seed)
  {
    if (seed == null)
      throw new ArgumentNullException(nameof(seed));
    _inner = new Xoshiro256PlusPlus(Fnv1a.Hash(seed));
  }

  // ========================================================================
  // Floating Point Methods
  // ========================================================================

  /// <summary>
  /// Generate a uniform random double in [0, 1).
  /// Uses 53 bits of precision for the mantissa.
  /// </summary>
  /// <returns>A random value in [0, 1).</returns>
  public double Uniform()
  {
    return _inner.Uniform();
  }

  /// <summary>
  /// Generate a uniform random double in [min, max).
  /// </summary>
  /// <param name="min">Minimum value (inclusive).</param>
  /// <param name="max">Maximum value (exclusive).</param>
  /// <returns>A random value in [min, max). Returns min if min >= max.</returns>
  public double Uniform(double min, double max)
  {
    return _inner.Uniform(min, max);
  }

  /// <summary>
  /// Generate a uniform random float in [0, 1).
  /// Uses 24 bits for float mantissa precision.
  /// </summary>
  /// <returns>A random value in [0, 1).</returns>
  public float UniformSingle()
  {
    return _inner.UniformSingle();
  }

  /// <summary>
  /// Generate a uniform random float in [min, max).
  /// </summary>
  /// <param name="min">Minimum value (inclusive).</param>
  /// <param name="max">Maximum value (exclusive).</param>
  /// <returns>A random value in [min, max). Returns min if min >= max.</returns>
  public float UniformSingle(float min, float max)
  {
    return _inner.UniformSingle(min, max);
  }

  // ========================================================================
  // Signed Integer Methods
  // ========================================================================

  /// <summary>
  /// Generate a uniform random long in [min, max).
  /// </summary>
  /// <remarks>
  /// Uses modulo reduction which introduces slight bias for ranges that don't
  /// evenly divide 2^64. This bias is negligible for statistical simulations
  /// but not suitable for cryptographic applications.
  /// </remarks>
  /// <param name="min">Minimum value (inclusive).</param>
  /// <param name="max">Maximum value (exclusive).</param>
  /// <returns>A random long in [min, max). Returns min if min >= max.</returns>
  public long UniformInt64(long min, long max)
  {
    return _inner.UniformInt64(min, max);
  }

  /// <summary>
  /// Generate a uniform random int in [min, max).
  /// </summary>
  /// <param name="min">Minimum value (inclusive).</param>
  /// <param name="max">Maximum value (exclusive).</param>
  /// <returns>A random int in [min, max). Returns min if min >= max.</returns>
  public int UniformInt32(int min, int max)
  {
    return _inner.UniformInt32(min, max);
  }

  /// <summary>
  /// Generate a uniform random short in [min, max).
  /// </summary>
  /// <param name="min">Minimum value (inclusive).</param>
  /// <param name="max">Maximum value (exclusive).</param>
  /// <returns>A random short in [min, max). Returns min if min >= max.</returns>
  public short UniformInt16(short min, short max)
  {
    return _inner.UniformInt16(min, max);
  }

  /// <summary>
  /// Generate a uniform random sbyte in [min, max).
  /// </summary>
  /// <param name="min">Minimum value (inclusive).</param>
  /// <param name="max">Maximum value (exclusive).</param>
  /// <returns>A random sbyte in [min, max). Returns min if min >= max.</returns>
  [CLSCompliant(false)]
  public sbyte UniformInt8(sbyte min, sbyte max)
  {
    return _inner.UniformInt8(min, max);
  }

  // ========================================================================
  // Unsigned Integer Methods
  // ========================================================================

  /// <summary>
  /// Generate a uniform random ulong in [min, max).
  /// </summary>
  /// <param name="min">Minimum value (inclusive).</param>
  /// <param name="max">Maximum value (exclusive).</param>
  /// <returns>A random ulong in [min, max). Returns min if min >= max.</returns>
  [CLSCompliant(false)]
  public ulong UniformUInt64(ulong min, ulong max)
  {
    return _inner.UniformUInt64(min, max);
  }

  /// <summary>
  /// Generate a uniform random uint in [min, max).
  /// </summary>
  /// <param name="min">Minimum value (inclusive).</param>
  /// <param name="max">Maximum value (exclusive).</param>
  /// <returns>A random uint in [min, max). Returns min if min >= max.</returns>
  [CLSCompliant(false)]
  public uint UniformUInt32(uint min, uint max)
  {
    return _inner.UniformUInt32(min, max);
  }

  /// <summary>
  /// Generate a uniform random ushort in [min, max).
  /// </summary>
  /// <param name="min">Minimum value (inclusive).</param>
  /// <param name="max">Maximum value (exclusive).</param>
  /// <returns>A random ushort in [min, max). Returns min if min >= max.</returns>
  [CLSCompliant(false)]
  public ushort UniformUInt16(ushort min, ushort max)
  {
    return _inner.UniformUInt16(min, max);
  }

  /// <summary>
  /// Generate a uniform random byte in [min, max).
  /// </summary>
  /// <param name="min">Minimum value (inclusive).</param>
  /// <param name="max">Maximum value (exclusive).</param>
  /// <returns>A random byte in [min, max). Returns min if min >= max.</returns>
  public byte UniformByte(byte min, byte max)
  {
    return _inner.UniformByte(min, max);
  }

  // ========================================================================
  // Boolean Methods
  // ========================================================================

  /// <summary>
  /// Generate a uniform random boolean with P(true) = 0.5.
  /// </summary>
  /// <returns>A random boolean.</returns>
  public bool UniformBool()
  {
    return _inner.UniformBool();
  }

  // ========================================================================
  // Deprecated Methods
  // ========================================================================

  /// <summary>
  /// Generate a uniform random integer in [min, max).
  /// </summary>
  /// <remarks>
  /// Uses modulo reduction which introduces slight bias for ranges that don't
  /// evenly divide 2^64. This bias is negligible for statistical simulations
  /// but not suitable for cryptographic applications.
  /// </remarks>
  /// <param name="min">Minimum value (inclusive).</param>
  /// <param name="max">Maximum value (exclusive).</param>
  /// <returns>A random integer in [min, max). Returns min if min >= max.</returns>
  [Obsolete("Use UniformInt64 instead")]
  public long UniformInt(long min, long max)
  {
    return _inner.UniformInt64(min, max);
  }

  // ========================================================================
  // Collection Methods
  // ========================================================================

  /// <summary>
  /// Return a shuffled copy of the input list.
  /// Uses the Fisher-Yates shuffle algorithm for uniform distribution.
  /// The original list is not modified.
  /// </summary>
  /// <typeparam name="T">Element type.</typeparam>
  /// <param name="x">Input list to shuffle.</param>
  /// <returns>Shuffled copy of the input.</returns>
  public List<T> Shuffle<T>(IReadOnlyList<T> x)
  {
    var result = new List<T>(x);
    int n = result.Count;

    // Fisher-Yates shuffle (backwards)
    for (int i = n - 1; i > 0; i--)
    {
      int j = (int)UniformInt64(0, i + 1);
      (result[i], result[j]) = (result[j], result[i]);
    }

    return result;
  }

  /// <summary>
  /// Sample k elements from the input list without replacement.
  /// Uses selection sampling to maintain order of first appearance.
  /// Returns all elements if k >= x.Count.
  /// </summary>
  /// <typeparam name="T">Element type.</typeparam>
  /// <param name="x">Input list to sample from.</param>
  /// <param name="k">Number of elements to sample. Must be non-negative.</param>
  /// <returns>List of k sampled elements.</returns>
  /// <exception cref="ArgumentOutOfRangeException">Thrown if k is negative.</exception>
  public List<T> Sample<T>(IReadOnlyList<T> x, int k)
  {
    if (k < 0)
      throw new ArgumentOutOfRangeException(nameof(k), k, "k must be non-negative");

    int n = x.Count;
    if (k >= n)
    {
      return new List<T>(x);
    }

    var result = new List<T>(k);
    int remaining = k;

    for (int i = 0; i < n && remaining > 0; i++)
    {
      int available = n - i;
      // Probability of selecting this item: remaining / available
      if (Uniform() * available < remaining)
      {
        result.Add(x[i]);
        remaining--;
      }
    }

    return result;
  }

  /// <summary>
  /// Return a shuffled copy of the sample values.
  /// </summary>
  /// <param name="sample">Input sample to shuffle.</param>
  /// <returns>New sample with shuffled values.</returns>
  public Sample Shuffle(Sample sample)
  {
    if (sample.IsWeighted)
      throw new NotSupportedException("Weighted samples are not supported by Rng.Shuffle");
    var shuffled = Shuffle(sample.Values);
    return new Sample(shuffled, sample.Unit);
  }

  /// <summary>
  /// Sample k elements from the sample values without replacement.
  /// </summary>
  /// <param name="sample">Input sample to sample from.</param>
  /// <param name="k">Number of elements to sample. Must be non-negative.</param>
  /// <returns>New sample with k sampled values.</returns>
  /// <exception cref="ArgumentOutOfRangeException">Thrown if k is negative.</exception>
  public Sample Sample(Sample sample, int k)
  {
    if (k < 0)
      throw new ArgumentOutOfRangeException(nameof(k), k, "k must be non-negative");
    if (sample.IsWeighted)
      throw new NotSupportedException("Weighted samples are not supported by Rng.Sample");
    int n = sample.Size;
    if (k >= n)
    {
      return sample;
    }

    var values = new List<double>(k);
    int remaining = k;

    for (int i = 0; i < n && remaining > 0; i++)
    {
      int available = n - i;
      if (Uniform() * available < remaining)
      {
        values.Add(sample.Values[i]);
        remaining--;
      }
    }

    return new Sample(values, sample.Unit);
  }

  // ========================================================================
  // Bootstrap (With-Replacement) Methods
  // ========================================================================

  /// <summary>
  /// Resample k elements from the input list with replacement (bootstrap sampling).
  /// </summary>
  /// <typeparam name="T">Element type.</typeparam>
  /// <param name="x">Input list to sample from.</param>
  /// <param name="k">Number of elements to sample. Must be non-negative.</param>
  /// <returns>List of k sampled elements (may contain duplicates).</returns>
  /// <exception cref="ArgumentOutOfRangeException">Thrown if k is negative.</exception>
  /// <exception cref="ArgumentException">Thrown if input list is empty.</exception>
  public List<T> Resample<T>(IReadOnlyList<T> x, int k)
  {
    if (k < 0)
      throw new ArgumentOutOfRangeException(nameof(k), k, "k must be non-negative");
    if (x.Count == 0)
      throw new ArgumentException("Cannot resample from empty list", nameof(x));

    var result = new List<T>(k);
    for (int i = 0; i < k; i++)
      result.Add(x[UniformInt32(0, x.Count)]);
    return result;
  }

  /// <summary>
  /// Resample k elements from the sample values with replacement (bootstrap sampling).
  /// </summary>
  /// <param name="sample">Input sample to resample from.</param>
  /// <param name="k">Number of elements to sample. Must be non-negative.</param>
  /// <returns>New sample with k resampled values (may contain duplicates).</returns>
  /// <exception cref="ArgumentOutOfRangeException">Thrown if k is negative.</exception>
  public Sample Resample(Sample sample, int k)
  {
    if (k < 0)
      throw new ArgumentOutOfRangeException(nameof(k), k, "k must be non-negative");
    if (sample.IsWeighted)
      throw new NotSupportedException("Weighted samples are not supported by Rng.Resample");
    if (sample.Size == 0)
      throw new ArgumentException("Cannot resample from empty sample", nameof(sample));

    var values = new List<double>(k);
    for (int i = 0; i < k; i++)
      values.Add(sample.Values[UniformInt32(0, sample.Size)]);
    return new Sample(values, sample.Unit);
  }
}
