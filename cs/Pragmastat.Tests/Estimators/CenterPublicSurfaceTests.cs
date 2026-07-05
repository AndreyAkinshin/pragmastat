namespace Pragmastat.Tests.Estimators;

/// <summary>
/// NIT public-surface coverage for the raw Center API:
/// (1) Toolkit.Center(unsorted, assumeSorted:true) surfaces the kernel's convergence guard
///     (an InvalidOperationException) at the public surface, not only at the kernel level.
/// (2) assumeSorted:true must not mutate the caller's array (RawMutationTests only cover
///     assumeSorted:false).
/// </summary>
public class CenterPublicSurfaceTests
{
  [Fact]
  public void Center_UnsortedWithAssumeSorted_ThrowsConvergence()
  {
    // Adversarial unsorted input: with assumeSorted:true the Monahan selection loop cannot make
    // consistent progress and terminates deterministically via the convergence guard
    // (iteration cap + stall detection).
    var values = new double[] { 5, 1, 9, 2, 8, 3, 7, 4, 6, 0, 5, 1, 9, 2, 8, 3, 7, 4, 6, 0 };
    var ex = Assert.Throws<InvalidOperationException>(() => Toolkit.Center(values, assumeSorted: true));
    Assert.Contains("Convergence failure", ex.Message);
  }

  [Fact]
  public void Center_AssumeSortedTrue_DoesNotMutateInput()
  {
    var x = new double[] { 1, 2, 3, 4, 5, 6, 7, 8 };
    var orig = (double[])x.Clone();
    Toolkit.Center(x, assumeSorted: true);
    Assert.Equal(orig, x);
  }
}
