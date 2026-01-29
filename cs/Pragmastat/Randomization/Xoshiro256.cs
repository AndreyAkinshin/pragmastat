using System.Runtime.CompilerServices;
using System.Text;

namespace Pragmastat.Randomization;

/// <summary>
/// SplitMix64 PRNG for seed expansion.
/// </summary>
internal sealed class SplitMix64
{
  private ulong _state;

  public SplitMix64(ulong seed)
  {
    _state = seed;
  }

  [MethodImpl(MethodImplOptions.AggressiveInlining)]
  public ulong Next()
  {
    _state += 0x9e3779b97f4a7c15UL;
    ulong z = _state;
    z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9UL;
    z = (z ^ (z >> 27)) * 0x94d049bb133111ebUL;
    return z ^ (z >> 31);
  }
}

/// <summary>
/// xoshiro256++ PRNG.
/// Reference: https://prng.di.unimi.it/xoshiro256plusplus.c
/// </summary>
/// <remarks>
/// This is the jump-free version of the algorithm. It passes BigCrush
/// and is used by .NET 6+, Julia, and Rust's rand crate.
/// </remarks>
internal sealed class Xoshiro256PlusPlus
{
  private ulong _s0, _s1, _s2, _s3;

  public Xoshiro256PlusPlus(ulong seed)
  {
    var sm = new SplitMix64(seed);
    _s0 = sm.Next();
    _s1 = sm.Next();
    _s2 = sm.Next();
    _s3 = sm.Next();
  }

  [MethodImpl(MethodImplOptions.AggressiveInlining)]
  private static ulong RotateLeft(ulong x, int k)
  {
    return (x << k) | (x >> (64 - k));
  }

  [MethodImpl(MethodImplOptions.AggressiveInlining)]
  public ulong NextU64()
  {
    ulong result = RotateLeft(_s0 + _s3, 23) + _s0;

    ulong t = _s1 << 17;

    _s2 ^= _s0;
    _s3 ^= _s1;
    _s1 ^= _s2;
    _s0 ^= _s3;

    _s2 ^= t;
    _s3 = RotateLeft(_s3, 45);

    return result;
  }

  [MethodImpl(MethodImplOptions.AggressiveInlining)]
  public double Uniform()
  {
    // Use upper 53 bits for maximum precision
    return (NextU64() >> 11) * (1.0 / (1UL << 53));
  }

  /// <exception cref="OverflowException">Thrown if max - min overflows.</exception>
  [MethodImpl(MethodImplOptions.AggressiveInlining)]
  public long UniformInt(long min, long max)
  {
    if (min >= max)
      return min;
    ulong range = checked((ulong)(max - min));
    return min + (long)(NextU64() % range);
  }
}

/// <summary>
/// FNV-1a hash implementation.
/// </summary>
internal static class Fnv1a
{
  private const ulong OffsetBasis = 0xcbf29ce484222325UL;
  private const ulong Prime = 0x00000100000001b3UL;

  /// <summary>
  /// Compute FNV-1a 64-bit hash of a string.
  /// </summary>
  public static ulong Hash(string s)
  {
    ulong hash = OffsetBasis;
    byte[] bytes = Encoding.UTF8.GetBytes(s);
    foreach (byte b in bytes)
    {
      hash ^= b;
      hash *= Prime;
    }
    return hash;
  }
}
