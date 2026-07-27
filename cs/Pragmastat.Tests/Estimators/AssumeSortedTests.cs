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
///
/// Every comparison here is BITWISE, not tolerant. Both sides are the same kernel on the
/// same values, reached by two routes: assumeSorted=false sorts a copy and hands the kernel
/// exactly the array assumeSorted=true hands it directly. No arithmetic separates the two,
/// so there is nothing for a rounding difference to come from. Either the kernel selects the
/// same value and the payloads match to the last bit, or one route does something the other
/// does not, which is a defect. An epsilon there measures nothing and hides that defect.
/// Ratio and RatioBounds keep the exact comparison for the same reason: both sides take the
/// same log -> shift -> exp route (log is monotonic, so sorting before or after it produces
/// the identical array), so the transcendental calls see identical arguments.
/// </summary>
public class AssumeSortedTests
{
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

  private static void AssertSameBounds(string context, Bounds want, Bounds got)
  {
    BitwiseAssert.Equal(want.Lower, got.Lower, $"{context}, lower");
    BitwiseAssert.Equal(want.Upper, got.Upper, $"{context}, upper");
  }

  // --- Order-independent scalar estimators ---

  [Fact]
  public void Center_AssumeSortedRoundtrip()
  {
    double want = Toolkit.Center(X, assumeSorted: false);
    double got = Toolkit.Center(Sorted(X), assumeSorted: true);
    BitwiseAssert.Equal(want, got, "Center(x, assumeSorted: false) vs Center(sorted x, assumeSorted: true)");
  }

  [Fact]
  public void Spread_AssumeSortedRoundtrip()
  {
    double want = Toolkit.Spread(X, assumeSorted: false);
    double got = Toolkit.Spread(Sorted(X), assumeSorted: true);
    BitwiseAssert.Equal(want, got, "Spread(x, assumeSorted: false) vs Spread(sorted x, assumeSorted: true)");
  }

  [Fact]
  public void Shift_AssumeSortedRoundtrip()
  {
    double want = Toolkit.Shift(X, Y, assumeSorted: false);
    double got = Toolkit.Shift(Sorted(X), Sorted(Y), assumeSorted: true);
    BitwiseAssert.Equal(want, got,
      "Shift(x, y, assumeSorted: false) vs Shift(sorted x, sorted y, assumeSorted: true)");
  }

  [Fact]
  public void Ratio_AssumeSortedRoundtrip()
  {
    double want = Toolkit.Ratio(X, Y, assumeSorted: false);
    double got = Toolkit.Ratio(Sorted(X), Sorted(Y), assumeSorted: true);
    BitwiseAssert.Equal(want, got,
      "Ratio(x, y, assumeSorted: false) vs Ratio(sorted x, sorted y, assumeSorted: true)");
  }

  [Fact]
  public void Disparity_AssumeSortedRoundtrip()
  {
    double want = Toolkit.Disparity(X, Y, assumeSorted: false);
    double got = Toolkit.Disparity(Sorted(X), Sorted(Y), assumeSorted: true);
    BitwiseAssert.Equal(want, got,
      "Disparity(x, y, assumeSorted: false) vs Disparity(sorted x, sorted y, assumeSorted: true)");
  }

  // --- Order-independent bounds estimators ---

  [Fact]
  public void CenterBounds_AssumeSortedRoundtrip()
  {
    var want = Toolkit.CenterBounds(X, Misrate, assumeSorted: false);
    var got = Toolkit.CenterBounds(Sorted(X), Misrate, assumeSorted: true);
    AssertSameBounds("CenterBounds(x, assumeSorted: false) vs CenterBounds(sorted x, assumeSorted: true)", want, got);
  }

  [Fact]
  public void ShiftBounds_AssumeSortedRoundtrip()
  {
    var want = Toolkit.ShiftBounds(X, Y, Misrate, assumeSorted: false);
    var got = Toolkit.ShiftBounds(Sorted(X), Sorted(Y), Misrate, assumeSorted: true);
    AssertSameBounds(
      "ShiftBounds(x, y, assumeSorted: false) vs ShiftBounds(sorted x, sorted y, assumeSorted: true)", want, got);
  }

  [Fact]
  public void RatioBounds_AssumeSortedRoundtrip()
  {
    var want = Toolkit.RatioBounds(X, Y, Misrate, assumeSorted: false);
    var got = Toolkit.RatioBounds(Sorted(X), Sorted(Y), Misrate, assumeSorted: true);
    AssertSameBounds(
      "RatioBounds(x, y, assumeSorted: false) vs RatioBounds(sorted x, sorted y, assumeSorted: true)", want, got);
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
    AssertSameBounds("SpreadBounds(sorted x, assumeSorted: false vs true)", want, got);
  }

  [Fact]
  public void DisparityBounds_AssumeSortedIdenticalOnSorted()
  {
    var sortedX = Sorted(X);
    var sortedY = Sorted(Y);
    var want = Toolkit.DisparityBounds(sortedX, sortedY, Misrate, Seed, assumeSorted: false);
    var got = Toolkit.DisparityBounds(sortedX, sortedY, Misrate, Seed, assumeSorted: true);
    AssertSameBounds("DisparityBounds(sorted x, sorted y, assumeSorted: false vs true)", want, got);
  }

  // --- spreadBounds assumeSorted contract. assumeSorted=true is INERT only on SORTED input
  // (see SpreadBounds_AssumeSortedIdenticalOnSorted above): only the shuffle part is genuinely
  // order-independent. On UNSORTED input, assumeSorted=true is UNDEFINED BEHAVIOR, exactly like
  // every other estimator: the sparity (spread>0) check runs SpreadImpl(x, assumeSorted), and on
  // unsorted x with assumeSorted=true that feeds unsorted data to a sorted-only kernel (may hit
  // the iteration cap and ERROR, or pass by luck). So there is NO inert-on-UNSORTED test here. ---
}
