namespace Pragmastat.Internal;

internal static class Constants
{
  /// <summary>
  /// The Euler–Mascheroni constant (or just the Euler's constant)
  /// </summary>
  public const double EulerMascheroni = 0.57721566490153286060651209008240243104215933593992;

  /// <summary>
  /// Natural logarithm of two
  /// </summary>
  public const double Log2 = 0.69314718055994530941;

  /// <summary>
  /// Square root of two
  /// </summary>
  public const double Sqrt2 = 1.4142135623730950488016887;

  /// <summary>
  /// Square root of 2π
  /// </summary>
  public const double Sqrt2Pi = 2.506628274631000241612355239340;

  /// <summary>
  /// Smallest positive subnormal (denormalized) IEEE 754 double-precision value.
  /// </summary>
  /// <remarks>
  /// <para>
  /// Value: 2^(-1074) ≈ 4.94e-324, represented as 5e-324 for cross-language consistency.
  /// </para>
  /// <para>
  /// This is the smallest positive value representable in IEEE 754 binary64 format.
  /// Unlike machine epsilon (which is the smallest ε where 1+ε ≠ 1), this is the
  /// absolute smallest positive number before underflow to zero.
  /// </para>
  /// <para>
  /// Used by <see cref="Distributions.SamplingCumulance"/> to lift a drawn cumulance off zero,
  /// where the unbounded families answer with an infinity. All language implementations use this
  /// same value to ensure cross-language determinism in distribution sampling.
  /// </para>
  /// </remarks>
  public const double SmallestPositiveSubnormal = 5e-324;
}
