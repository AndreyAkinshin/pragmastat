namespace Pragmastat.Tests.Estimators;

/// <summary>
/// Directly exercises the raw (native-array) API's assumeSorted=true branch. The
/// dual-path reference tests only ever pass assumeSorted=false; the =true branch is
/// reached only transitively via Sample. This adds a direct check.
///
/// For ORDER-INDEPENDENT estimators, sorting the input ascending and calling with
/// assumeSorted=true must equal the call on the unsorted input with assumeSorted=false.
///
/// For SHUFFLE-based bounds, assumeSorted only affects the internal sparity check, never
/// the shuffle, so on an already-SORTED array with a fixed seed the result must be
/// byte-identical for true vs false.
/// </summary>
public class AssumeSortedTests
{
  private const double Eps = 1e-9;
  private const string Seed = "pragmastat";
  private static readonly Probability Misrate = new(0.3);

  private static readonly double[] X = { 3, 1, 2, 5, 4, 8, 6, 7 };
  private static readonly double[] Y = { 9, 11, 10, 13, 12, 16, 14, 15 };

  private static double[] Sorted(double[] values)
  {
    var copy = (double[])values.Clone();
    Array.Sort(copy);
    return copy;
  }

  // --- Order-independent scalar estimators ---

  [Fact]
  public void Center_AssumeSortedRoundtrip()
  {
    double want = Toolkit.Center(X, assumeSorted: false);
    double got = Toolkit.Center(Sorted(X), assumeSorted: true);
    Assert.Equal(want, got, Eps);
  }

  [Fact]
  public void Spread_AssumeSortedRoundtrip()
  {
    double want = Toolkit.Spread(X, assumeSorted: false);
    double got = Toolkit.Spread(Sorted(X), assumeSorted: true);
    Assert.Equal(want, got, Eps);
  }

  [Fact]
  public void Shift_AssumeSortedRoundtrip()
  {
    double want = Toolkit.Shift(X, Y, assumeSorted: false);
    double got = Toolkit.Shift(Sorted(X), Sorted(Y), assumeSorted: true);
    Assert.Equal(want, got, Eps);
  }

  [Fact]
  public void Ratio_AssumeSortedRoundtrip()
  {
    double want = Toolkit.Ratio(X, Y, assumeSorted: false);
    double got = Toolkit.Ratio(Sorted(X), Sorted(Y), assumeSorted: true);
    Assert.Equal(want, got, Eps);
  }

  [Fact]
  public void Disparity_AssumeSortedRoundtrip()
  {
    double want = Toolkit.Disparity(X, Y, assumeSorted: false);
    double got = Toolkit.Disparity(Sorted(X), Sorted(Y), assumeSorted: true);
    Assert.Equal(want, got, Eps);
  }

  // --- Order-independent bounds estimators ---

  [Fact]
  public void CenterBounds_AssumeSortedRoundtrip()
  {
    var want = Toolkit.CenterBounds(X, Misrate, assumeSorted: false);
    var got = Toolkit.CenterBounds(Sorted(X), Misrate, assumeSorted: true);
    Assert.Equal(want.Lower, got.Lower, Eps);
    Assert.Equal(want.Upper, got.Upper, Eps);
  }

  [Fact]
  public void ShiftBounds_AssumeSortedRoundtrip()
  {
    var want = Toolkit.ShiftBounds(X, Y, Misrate, assumeSorted: false);
    var got = Toolkit.ShiftBounds(Sorted(X), Sorted(Y), Misrate, assumeSorted: true);
    Assert.Equal(want.Lower, got.Lower, Eps);
    Assert.Equal(want.Upper, got.Upper, Eps);
  }

  [Fact]
  public void RatioBounds_AssumeSortedRoundtrip()
  {
    var want = Toolkit.RatioBounds(X, Y, Misrate, assumeSorted: false);
    var got = Toolkit.RatioBounds(Sorted(X), Sorted(Y), Misrate, assumeSorted: true);
    Assert.Equal(want.Lower, got.Lower, Eps);
    Assert.Equal(want.Upper, got.Upper, Eps);
  }

  // --- Shuffle-based bounds: assumeSorted never changes the shuffle (which always runs on
  // the original order). On a genuinely SORTED array with a fixed seed, the shuffle order is
  // identical for both calls and the only difference is whether the (valid) sorted view is
  // reused, so the result must be byte-identical. ---

  [Fact]
  public void SpreadBounds_AssumeSortedIdenticalOnSorted()
  {
    var sortedX = Sorted(X);
    var want = Toolkit.SpreadBounds(sortedX, Misrate, Seed, assumeSorted: false);
    var got = Toolkit.SpreadBounds(sortedX, Misrate, Seed, assumeSorted: true);
    Assert.Equal(want.Lower, got.Lower);
    Assert.Equal(want.Upper, got.Upper);
  }

  [Fact]
  public void DisparityBounds_AssumeSortedIdenticalOnSorted()
  {
    var sortedX = Sorted(X);
    var sortedY = Sorted(Y);
    var want = Toolkit.DisparityBounds(sortedX, sortedY, Misrate, Seed, assumeSorted: false);
    var got = Toolkit.DisparityBounds(sortedX, sortedY, Misrate, Seed, assumeSorted: true);
    Assert.Equal(want.Lower, got.Lower);
    Assert.Equal(want.Upper, got.Upper);
  }

  // --- spreadBounds assumeSorted contract. assumeSorted=true is INERT only on SORTED input
  // (see SpreadBounds_AssumeSortedIdenticalOnSorted above): only the shuffle part is genuinely
  // order-independent. On UNSORTED input, assumeSorted=true is UNDEFINED BEHAVIOR, exactly like
  // every other estimator: the sparity (spread>0) check runs SpreadImpl(x, assumeSorted), and on
  // unsorted x with assumeSorted=true that feeds unsorted data to a sorted-only kernel (may hit
  // the iteration cap and ERROR, or pass by luck). So there is NO inert-on-UNSORTED test here. ---
}
