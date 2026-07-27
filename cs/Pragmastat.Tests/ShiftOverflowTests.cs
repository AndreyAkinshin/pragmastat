using Pragmastat;

namespace Pragmastat.Tests;

// Regression: shift search bounds x[0]-y[n-1] and x[m-1]-y[0] can overflow to
// +-Infinity on extreme finite input, turning the midpoint into NaN and returning
// +-Infinity instead of the true finite shift.
public class ShiftOverflowTests
{
  [Fact]
  public void ShiftDoesNotOverflowOnExtremeFiniteInput()
  {
    const double max = double.MaxValue;
    // Bitwise: the claim is the exact finite shift, and the first case is a difference of
    // symmetric samples, where +0.0 and -0.0 are both reachable and == cannot tell them apart.
    BitwiseAssert.Equal(0.0, Toolkit.Shift(new[] { -max, max }, new[] { -max, max }, assumeSorted: true),
      "Shift([-max, max], [-max, max])");
    BitwiseAssert.Equal(max, Toolkit.Shift(new[] { 0.0, max }, new[] { -max, 0.0 }, assumeSorted: true),
      "Shift([0, max], [-max, 0])");
  }
}
