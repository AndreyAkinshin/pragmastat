namespace Pragmastat.Tests.Estimators;

/// <summary>
/// Forward guard: the n==2 center midpoint must be exact and order-symmetric. The current
/// 0.5*a + 0.5*b form is bit-exact regardless of operand order and never overflows for
/// extreme inputs; midpoint variants such as a + (b - a) * 0.5 are order-asymmetric (they
/// give -3.4000000000000004 for the reversed operand order here). assumeSorted=true is
/// required so the midpoint sees the raw (unsorted) order; the normalizing sort would
/// otherwise hide any asymmetry.
/// </summary>
public class CenterMidpointSymmetryTests
{
  [Fact]
  public void Center_N2_Midpoint_IsOrderSymmetric_Exact()
  {
    double forward = Toolkit.Center(new[] { -5.0, -1.8 }, assumeSorted: true);
    double reversed = Toolkit.Center(new[] { -1.8, -5.0 }, assumeSorted: true);

    // Exact bit equality (not approximate): both must be exactly -3.4.
    Assert.Equal(forward, reversed);
    Assert.Equal(-3.4, forward);
    Assert.Equal(-3.4, reversed);
  }
}
