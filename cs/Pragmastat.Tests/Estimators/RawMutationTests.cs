namespace Pragmastat.Tests.Estimators;

/// <summary>
/// Regression guard: the public raw (native-array) API must not mutate the caller's array.
/// With assumeSorted=false the kernels sort a copy internally; with assumeSorted=true the
/// internal sort is skipped and the kernels may read the caller's array directly (aliasing),
/// so any in-place write would corrupt caller data. In both modes the input array must be
/// byte-for-byte unchanged after each call.
/// </summary>
public class RawMutationTests
{
  private static readonly Probability Misrate = new(0.3);

  [Fact]
  public void RawPointEstimators_DoNotMutateInput()
  {
    var x = new double[] { 3, 1, 2, 5, 4, 8, 6, 7 };
    var y = new double[] { 30, 10, 20, 50, 40, 80, 60, 70 };
    var origX = (double[])x.Clone();
    var origY = (double[])y.Clone();

    Toolkit.Center(x, assumeSorted: false);
    Toolkit.Spread(x, assumeSorted: false);
    Toolkit.Shift(x, y, assumeSorted: false);
    Toolkit.Ratio(x, y, assumeSorted: false);
    Toolkit.Disparity(x, y, assumeSorted: false);

    Assert.Equal(origX, x);
    Assert.Equal(origY, y);
  }

  [Fact]
  public void RawBoundsEstimators_DoNotMutateInput()
  {
    var x = new double[] { 3, 1, 2, 5, 4, 8, 6, 7 };
    var y = new double[] { 30, 10, 20, 50, 40, 80, 60, 70 };
    var origX = (double[])x.Clone();
    var origY = (double[])y.Clone();

    Toolkit.CenterBounds(x, Misrate, assumeSorted: false);
    Toolkit.SpreadBounds(x, Misrate, assumeSorted: false);
    Toolkit.ShiftBounds(x, y, Misrate, assumeSorted: false);
    Toolkit.RatioBounds(x, y, Misrate, assumeSorted: false);
    Toolkit.DisparityBounds(x, y, Misrate, assumeSorted: false);

    Assert.Equal(origX, x);
    Assert.Equal(origY, y);
  }

  // --- Aliasing branch: assumeSorted=true on genuinely SORTED input. The kernels see the
  // caller's array without a defensive sort-copy, so this is where an in-place write would
  // actually leak out. ---

  [Fact]
  public void RawPointEstimators_AssumeSorted_DoNotMutateInput()
  {
    var x = new double[] { 1, 2, 3, 4, 5, 6, 7, 8 };
    var y = new double[] { 10, 20, 30, 40, 50, 60, 70, 80 };
    var origX = (double[])x.Clone();
    var origY = (double[])y.Clone();

    Toolkit.Center(x, assumeSorted: true);
    Toolkit.Spread(x, assumeSorted: true);
    Toolkit.Shift(x, y, assumeSorted: true);
    Toolkit.Ratio(x, y, assumeSorted: true);
    Toolkit.Disparity(x, y, assumeSorted: true);

    Assert.Equal(origX, x);
    Assert.Equal(origY, y);
  }

  [Fact]
  public void RawBoundsEstimators_AssumeSorted_DoNotMutateInput()
  {
    var x = new double[] { 1, 2, 3, 4, 5, 6, 7, 8 };
    var y = new double[] { 10, 20, 30, 40, 50, 60, 70, 80 };
    var origX = (double[])x.Clone();
    var origY = (double[])y.Clone();

    Toolkit.CenterBounds(x, Misrate, assumeSorted: true);
    Toolkit.SpreadBounds(x, Misrate, assumeSorted: true);
    Toolkit.ShiftBounds(x, y, Misrate, assumeSorted: true);
    Toolkit.RatioBounds(x, y, Misrate, assumeSorted: true);
    Toolkit.DisparityBounds(x, y, Misrate, assumeSorted: true);

    Assert.Equal(origX, x);
    Assert.Equal(origY, y);
  }
}
