using Pragmastat.Randomization;

namespace Pragmastat.Tests.Estimators;

public class EstimatorInvarianceTests
{
  private const long Seed = 1729;
  private static readonly int[] SampleSizes = [2, 3, 4, 5, 6, 7, 8, 9, 10];
  private const double Tolerance = 1e-9;

  private static Sample UniformSample(Rng rng, int n)
  {
    var values = new double[n];
    for (int i = 0; i < n; i++)
      values[i] = rng.UniformDouble();
    return new Sample(values);
  }

  private static void PerformTestOne(Func<Sample, double> expr1, Func<Sample, double> expr2)
  {
    var rng = new Rng(Seed);
    foreach (int n in SampleSizes)
    {
      var x = UniformSample(rng, n);
      double result1 = expr1(x);
      double result2 = expr2(x);
      Assert.True(Math.Abs(result1 - result2) < Tolerance,
        $"Failed for n={n}: {result1} != {result2}");
    }
  }

  private static void PerformTestTwo(Func<Sample, Sample, double> expr1, Func<Sample, Sample, double> expr2)
  {
    var rng = new Rng(Seed);
    foreach (int n in SampleSizes)
    {
      var x = UniformSample(rng, n);
      var y = UniformSample(rng, n);
      double result1 = expr1(x, y);
      double result2 = expr2(x, y);
      Assert.True(Math.Abs(result1 - result2) < Tolerance,
        $"Failed for n={n}: {result1} != {result2}");
    }
  }

  // Center tests

  [Fact]
  public void CenterShift()
  {
    PerformTestOne(x => Toolkit.Center(x + 2), x => Toolkit.Center(x) + 2);
  }

  [Fact]
  public void CenterScale()
  {
    PerformTestOne(x => Toolkit.Center(x * 2), x => 2 * Toolkit.Center(x));
  }

  [Fact]
  public void CenterNegate()
  {
    PerformTestOne(x => Toolkit.Center(x * -1), x => -1 * Toolkit.Center(x));
  }

  // Spread tests

  [Fact]
  public void SpreadShift()
  {
    PerformTestOne(x => Toolkit.Spread(x + 2), x => Toolkit.Spread(x));
  }

  [Fact]
  public void SpreadScale()
  {
    PerformTestOne(x => Toolkit.Spread(x * 2), x => 2 * Toolkit.Spread(x));
  }

  [Fact]
  public void SpreadNegate()
  {
    PerformTestOne(x => Toolkit.Spread(x * -1), x => Toolkit.Spread(x));
  }

  // RelSpread tests

  [Fact]
#pragma warning disable CS0618 // Obsolete
  public void RelSpreadScale()
  {
    PerformTestOne(x => Toolkit.RelSpread(x * 2), x => Toolkit.RelSpread(x));
  }
#pragma warning restore CS0618

  // Shift tests

  [Fact]
  public void ShiftShift()
  {
    PerformTestTwo((x, y) => Toolkit.Shift(x + 3, y + 2), (x, y) => Toolkit.Shift(x, y) + 1);
  }

  [Fact]
  public void ShiftScale()
  {
    PerformTestTwo((x, y) => Toolkit.Shift(x * 2, y * 2), (x, y) => 2 * Toolkit.Shift(x, y));
  }

  [Fact]
  public void ShiftAntisymmetry()
  {
    PerformTestTwo((x, y) => Toolkit.Shift(x, y), (x, y) => -1 * Toolkit.Shift(y, x));
  }

  // Ratio tests

  [Fact]
  public void RatioScale()
  {
    PerformTestTwo((x, y) => Toolkit.Ratio(x * 2, y * 3), (x, y) => (2.0 / 3) * Toolkit.Ratio(x, y));
  }

  // AvgSpread tests

  [Fact]
  public void AvgSpreadEqual()
  {
    PerformTestOne(x => Toolkit.AvgSpread(x, x), x => Toolkit.Spread(x));
  }

  [Fact]
  public void AvgSpreadSymmetry()
  {
    PerformTestTwo((x, y) => Toolkit.AvgSpread(x, y), (x, y) => Toolkit.AvgSpread(y, x));
  }

  [Fact]
  public void AvgSpreadAverage()
  {
    PerformTestOne(x => Toolkit.AvgSpread(x, x * 5), x => 3 * Toolkit.Spread(x));
  }

  [Fact]
  public void AvgSpreadScale()
  {
    PerformTestTwo((x, y) => Toolkit.AvgSpread(x * -2, y * -2), (x, y) => 2 * Toolkit.AvgSpread(x, y));
  }

  // Disparity tests

  [Fact]
  public void DisparityShift()
  {
    PerformTestTwo((x, y) => Toolkit.Disparity(x + 2, y + 2), (x, y) => Toolkit.Disparity(x, y));
  }

  [Fact]
  public void DisparityScale()
  {
    PerformTestTwo((x, y) => Toolkit.Disparity(x * 2, y * 2), (x, y) => Toolkit.Disparity(x, y));
  }

  [Fact]
  public void DisparityScaleNeg()
  {
    PerformTestTwo((x, y) => Toolkit.Disparity(x * -2, y * -2), (x, y) => -1 * Toolkit.Disparity(x, y));
  }

  [Fact]
  public void DisparityAntisymmetry()
  {
    PerformTestTwo((x, y) => Toolkit.Disparity(x, y), (x, y) => -1 * Toolkit.Disparity(y, x));
  }
}
