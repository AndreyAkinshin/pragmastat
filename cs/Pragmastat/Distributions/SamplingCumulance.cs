using Pragmastat.Internal;
using Pragmastat.Randomization;

namespace Pragmastat.Distributions;

/// <summary>
/// The cumulance one draw consumes, for the two families whose <c>QuantileAt(0)</c> is not a
/// sample: Additive returns negative infinity there, Multiplic returns the zero its own support
/// excludes. The other three families' <c>QuantileAt(0)</c> is the finite value their support
/// already includes, so they draw <c>UniformDouble</c> directly and never call this.
/// </summary>
/// <remarks>
/// A drawn 0 is lifted to the smallest positive subnormal. <c>UniformDouble</c> never returns 1,
/// so the other endpoint needs no guard.
/// </remarks>
internal static class SamplingCumulance
{
  public static double Draw(Rng rng)
  {
    double u = rng.UniformDouble();
    return u == 0.0 ? Constants.SmallestPositiveSubnormal : u;
  }
}
