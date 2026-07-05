using System.Diagnostics;
using Pragmastat.Algorithms;

namespace Pragmastat.Tests.Algorithms;

/// <summary>
/// SpreadImpl.Estimate on UNSORTED input with assumeSorted=true must NOT spin forever. The
/// Shamos selection loop has a convergence guard (iteration cap of 256 + 4n plus active-set
/// stall detection, mirroring CenterImpl's guard); when the contract is violated (the
/// assumeSorted=true promise is false), the loop terminates deterministically with a plain
/// InvalidOperationException ("convergence failure") instead of wedging the process.
/// </summary>
public class SpreadImplConvergenceTests
{
  [Fact]
  public void Estimate_UnsortedWithAssumeSorted_DoesNotHang_AndRaisesConvergenceError()
  {
    // Adversarial DESCENDING-with-ties input. With assumeSorted=true the algorithm treats it as
    // ascending, so the pairwise differences a[j]-a[i] go negative and the sorted-matrix
    // partition invariants are violated; the selection loop never reaches the target rank and
    // cannot make consistent progress. Deterministic RNG keeps the test reproducible.
    var values = new double[] { 5, 5, 4, 4, 3, 3, 2, 2, 1, 1 };
    var rng = new Random(1);

    var stopwatch = Stopwatch.StartNew();
    var ex = Assert.Throws<InvalidOperationException>(() =>
      SpreadImpl.Estimate(values, rng, assumeSorted: true));
    stopwatch.Stop();

    Assert.Contains("Convergence failure", ex.Message);
    // Must terminate quickly (the cap is O(1) work per iteration for small n); a wedge would
    // never return. Generous bound to avoid CI flakiness while still catching a true hang.
    Assert.True(stopwatch.Elapsed.TotalSeconds < 30,
      $"SpreadImpl took too long to fail: {stopwatch.Elapsed.TotalSeconds:F1}s");
  }

  [Fact]
  public void Estimate_ValidSortedInput_ConvergesNormally()
  {
    // Sanity: the cap must never trigger on valid sorted input.
    var values = Enumerable.Range(1, 1000).Select(i => (double)i).ToArray();
    double result = SpreadImpl.Estimate(values, new Random(1), assumeSorted: true);
    // Shamos spread of 1..1000 is the median of the 499500 pairwise gaps. Gap d occurs
    // (1000 - d) times, so ranks 249750 and 249751 both land on d = 293.
    Assert.Equal(293.0, result, 1e-9);
  }
}
