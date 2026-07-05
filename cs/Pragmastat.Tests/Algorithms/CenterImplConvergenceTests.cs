using System.Diagnostics;
using Pragmastat.Algorithms;

namespace Pragmastat.Tests.Algorithms;

/// <summary>
/// CenterImpl.Estimate on UNSORTED input with assumeSorted=true must NOT spin forever. The
/// Monahan selection loop has a convergence guard (iteration cap of 256 + 4n plus active-set
/// stall detection); when the contract is violated (the assumeSorted=true promise is false),
/// the loop terminates deterministically with a plain InvalidOperationException
/// ("convergence failure") instead of wedging the process.
/// </summary>
public class CenterImplConvergenceTests
{
  [Fact]
  public void Estimate_UnsortedWithAssumeSorted_DoesNotHang_AndRaisesConvergenceError()
  {
    // Adversarial unsorted input. With assumeSorted=true the algorithm's sorted-matrix
    // invariants are violated, so the selection loop cannot make consistent progress.
    // Deterministic RNG keeps the test reproducible.
    var values = new double[] { 5, 1, 9, 2, 8, 3, 7, 4, 6, 0, 5, 1, 9, 2, 8, 3, 7, 4, 6, 0 };
    var rng = new Random(12345);

    var stopwatch = Stopwatch.StartNew();
    var ex = Assert.Throws<InvalidOperationException>(() =>
      CenterImpl.Estimate(values, rng, assumeSorted: true));
    stopwatch.Stop();

    Assert.Contains("Convergence failure", ex.Message);
    // Must terminate quickly (the cap is O(1) work per iteration for small n); a wedge would
    // never return. Generous bound to avoid CI flakiness while still catching a true hang.
    Assert.True(stopwatch.Elapsed.TotalSeconds < 30,
      $"CenterImpl took too long to fail: {stopwatch.Elapsed.TotalSeconds:F1}s");
  }

  [Fact]
  public void Estimate_ValidSortedInput_ConvergesNormally()
  {
    // Sanity: the cap must never trigger on valid sorted input.
    var values = Enumerable.Range(1, 1000).Select(i => (double)i).ToArray();
    double result = CenterImpl.Estimate(values, new Random(1), assumeSorted: true);
    Assert.Equal(500.5, result, 1e-9);
  }
}
