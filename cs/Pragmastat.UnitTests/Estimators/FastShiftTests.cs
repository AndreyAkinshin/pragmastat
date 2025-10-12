using System.Diagnostics;
using Pragmastat.Algorithms;
using Pragmastat.Distributions;
using Pragmastat.Distributions.Randomization;

namespace Pragmastat.UnitTests.Estimators;

public class FastShiftTests
{
  private const double Tolerance = 1e-9;

  [Fact]
  public void PairwiseDiffQuantile_SmallArrays_MatchesNaive()
  {
    var random = AdditiveDistribution.Standard.Random(1729);

    for (int m = 1; m <= 20; m++)
    for (int n = 1; n <= 20; n++)
    for (int iteration = 0; iteration < 5; iteration++)
    {
      var x = random.Next(m);
      var y = random.Next(n);
      var p = new[] { 0.0, 0.25, 0.5, 0.75, 1.0 };

      var actual = FastShift.Estimate(x, y, p);
      var expected = NaiveQuantiles(x, y, p);

      Assert.Equal(expected.Length, actual.Length);
      for (int i = 0; i < expected.Length; i++)
        Assert.Equal(expected[i], actual[i], Tolerance);
    }
  }

  [Fact]
  public void PairwiseDiffQuantile_MediumArrays_MatchesNaive()
  {
    var random = AdditiveDistribution.Standard.Random(42);

    for (int size = 20; size <= 100; size += 10)
    for (int iteration = 0; iteration < 3; iteration++)
    {
      var x = random.Next(size);
      var y = random.Next(size / 2);
      var p = new[] { 0.1, 0.5, 0.9 };

      var actual = FastShift.Estimate(x, y, p);
      var expected = NaiveQuantiles(x, y, p);

      Assert.Equal(expected.Length, actual.Length);
      for (int i = 0; i < expected.Length; i++)
        Assert.Equal(expected[i], actual[i], Tolerance);
    }
  }

  [Fact]
  public void PairwiseDiffQuantile_DifferentDistributions_AllQuantiles()
  {
    var seed = 2024;
    var distributions = new[]
    {
      AdditiveDistribution.Standard,
      new AdditiveDistribution(5.0, 2.0),
      new AdditiveDistribution(-10.0, 1.0),
      new AdditiveDistribution(0.0, 0.1)
    };

    var probabilities = new[] { 0.0, 0.05, 0.1, 0.25, 0.5, 0.75, 0.9, 0.95, 1.0 };

    foreach (var dist in distributions)
    {
      var random = dist.Random(seed++);
      for (int trial = 0; trial < 10; trial++)
      {
        var x = random.Next(15);
        var y = random.Next(10);

        var actual = FastShift.Estimate(x, y, probabilities);
        var expected = NaiveQuantiles(x, y, probabilities);

        Assert.Equal(expected.Length, actual.Length);
        for (int i = 0; i < expected.Length; i++)
          Assert.Equal(expected[i], actual[i], Tolerance);
      }
    }
  }

  [Fact]
  public void PairwiseDiffQuantile_UnsortedInput_MatchesSorted()
  {
    var random = AdditiveDistribution.Standard.Random(999);

    for (int trial = 0; trial < 50; trial++)
    {
      var xRaw = random.Next(20);
      var yRaw = random.Next(15);
      var p = new[] { 0.25, 0.5, 0.75 };

      var xSorted = xRaw.OrderBy(v => v).ToArray();
      var ySorted = yRaw.OrderBy(v => v).ToArray();

      var xShuffled = xRaw.OrderBy(_ => random.Next()).ToArray();
      var yShuffled = yRaw.OrderBy(_ => random.Next()).ToArray();

      var resultUnsorted = FastShift.Estimate(xShuffled, yShuffled, p, assumeSorted: false);
      var resultSorted = FastShift.Estimate(xSorted, ySorted, p, assumeSorted: true);

      Assert.Equal(resultSorted.Length, resultUnsorted.Length);
      for (int i = 0; i < resultSorted.Length; i++)
        Assert.Equal(resultSorted[i], resultUnsorted[i], Tolerance);
    }
  }

  [Fact]
  public void PairwiseDiffQuantile_SingleElement_ReturnsConstant()
  {
    var random = AdditiveDistribution.Standard.Random(123);

    for (int trial = 0; trial < 20; trial++)
    {
      var x = new[] { random.Next() };
      var y = new[] { random.Next() };
      var p = new[] { 0.0, 0.25, 0.5, 0.75, 1.0 };

      var result = FastShift.Estimate(x, y, p);
      var expected = x[0] - y[0];

      foreach (var q in result)
        Assert.Equal(expected, q, Tolerance);
    }
  }

  [Fact]
  public void PairwiseDiffQuantile_IdenticalArrays_MedianIsZero()
  {
    var random = AdditiveDistribution.Standard.Random(456);

    for (int size = 1; size <= 30; size++)
    for (int trial = 0; trial < 3; trial++)
    {
      var x = random.Next(size);
      var p = new[] { 0.5 };

      var result = FastShift.Estimate(x, x, p);

      Assert.Equal(0.0, result[0], Tolerance);
    }
  }

  [Fact]
  public void PairwiseDiffQuantile_AsymmetricSizes_CorrectResults()
  {
    var random = AdditiveDistribution.Standard.Random(789);

    var configs = new[]
    {
      (m: 1, n: 100),
      (m: 100, n: 1),
      (m: 10, n: 50),
      (m: 50, n: 10),
      (m: 5, n: 200)
    };

    foreach (var (m, n) in configs)
    {
      var x = random.Next(m);
      var y = random.Next(n);
      var p = new[] { 0.0, 0.5, 1.0 };

      var actual = FastShift.Estimate(x, y, p);
      var expected = NaiveQuantiles(x, y, p);

      Assert.Equal(expected.Length, actual.Length);
      for (int i = 0; i < expected.Length; i++)
        Assert.Equal(expected[i], actual[i], Tolerance);
    }
  }

  [Fact]
  public void PairwiseDiffQuantile_ExtremeQuantiles_MatchMinMax()
  {
    var random = AdditiveDistribution.Standard.Random(321);

    for (int trial = 0; trial < 30; trial++)
    {
      var x = random.Next(10 + trial);
      var y = random.Next(8 + trial / 2);
      var p = new[] { 0.0, 1.0 };

      var result = FastShift.Estimate(x, y, p);

      double min = double.PositiveInfinity;
      double max = double.NegativeInfinity;
      foreach (var xi in x)
      foreach (var yj in y)
      {
        double diff = xi - yj;
        if (diff < min) min = diff;
        if (diff > max) max = diff;
      }

      Assert.Equal(min, result[0], Tolerance);
      Assert.Equal(max, result[1], Tolerance);
    }
  }

  [Fact]
  public void PairwiseDiffQuantile_ManyProbabilities_MonotonicIncreasing()
  {
    var random = AdditiveDistribution.Standard.Random(654);

    for (int trial = 0; trial < 20; trial++)
    {
      var x = random.Next(25);
      var y = random.Next(20);

      var p = Enumerable.Range(0, 21).Select(i => i / 20.0).ToArray();

      var result = FastShift.Estimate(x, y, p);

      for (int i = 1; i < result.Length; i++)
        Assert.True(result[i] >= result[i - 1] - Tolerance,
          $"Quantiles must be non-decreasing: result[{i}]={result[i]} < result[{i-1}]={result[i-1]}");
    }
  }

  [Fact]
  public void PairwiseDiffQuantile_NegativeValues_HandledCorrectly()
  {
    var random = new AdditiveDistribution(-50.0, 10.0).Random(111);

    for (int trial = 0; trial < 20; trial++)
    {
      var x = random.Next(15);
      var y = random.Next(12);
      var p = new[] { 0.25, 0.5, 0.75 };

      var actual = FastShift.Estimate(x, y, p);
      var expected = NaiveQuantiles(x, y, p);

      for (int i = 0; i < expected.Length; i++)
        Assert.Equal(expected[i], actual[i], Tolerance);
    }
  }

  [Fact]
  public void PairwiseDiffQuantile_DuplicateValues_HandledCorrectly()
  {
    var random = AdditiveDistribution.Standard.Random(222);

    for (int trial = 0; trial < 10; trial++)
    {
      var x = Enumerable.Range(0, 12).Select(_ => Math.Round(random.Next() * 5) / 5.0).ToArray();
      var y = Enumerable.Range(0, 10).Select(_ => Math.Round(random.Next() * 5) / 5.0).ToArray();
      var p = new[] { 0.0, 0.5, 1.0 };

      var actual = FastShift.Estimate(x, y, p);
      var expected = NaiveQuantiles(x, y, p);

      for (int i = 0; i < expected.Length; i++)
        Assert.Equal(expected[i], actual[i], Tolerance);
    }
  }

  [Fact]
  public void PairwiseDiffQuantile_VerySmallValues_NumericalStability()
  {
    var random = new AdditiveDistribution(0.0, 1e-8).Random(333);

    for (int trial = 0; trial < 10; trial++)
    {
      var x = random.Next(10);
      var y = random.Next(10);
      var p = new[] { 0.5 };

      var result = FastShift.Estimate(x, y, p);

      Assert.False(double.IsNaN(result[0]));
      Assert.False(double.IsInfinity(result[0]));
    }
  }

  [Fact]
  public void PairwiseDiffQuantile_LargeValues_NumericalStability()
  {
    var random = new AdditiveDistribution(1e6, 1e5).Random(444);

    for (int trial = 0; trial < 10; trial++)
    {
      var x = random.Next(10);
      var y = random.Next(10);
      var p = new[] { 0.5 };

      var result = FastShift.Estimate(x, y, p);

      Assert.False(double.IsNaN(result[0]));
      Assert.False(double.IsInfinity(result[0]));
    }
  }

  [Fact]
  public void PairwiseDiffQuantile_ZeroSpread_AllSame()
  {
    var x = Enumerable.Repeat(5.0, 10).ToArray();
    var y = Enumerable.Repeat(2.0, 8).ToArray();
    var p = new[] { 0.0, 0.25, 0.5, 0.75, 1.0 };

    var result = FastShift.Estimate(x, y, p);

    foreach (var q in result)
      Assert.Equal(3.0, q, Tolerance);
  }

  [Fact]
  public void PairwiseDiffQuantile_LargeArrays_PerformanceTest()
  {
    var random = AdditiveDistribution.Standard.Random(1729);
    var x = random.Next(500);
    var y = random.Next(500);
    var p = new[] { 0.5 };

    var stopwatch = Stopwatch.StartNew();
    var result = FastShift.Estimate(x, y, p);
    stopwatch.Stop();

    Trace.WriteLine($"500x500 arrays: {stopwatch.ElapsedMilliseconds}ms");
    Assert.True(stopwatch.Elapsed.TotalSeconds < 5);
    Assert.Single(result);
  }

  [Fact]
  public void PairwiseDiffQuantile_VeryLargeArrays_PerformanceTest()
  {
    var random = AdditiveDistribution.Standard.Random(9999);
    var x = random.Next(1000);
    var y = random.Next(1000);
    var p = new[] { 0.5 };

    var stopwatch = Stopwatch.StartNew();
    var result = FastShift.Estimate(x, y, p);
    stopwatch.Stop();

    Trace.WriteLine($"1000x1000 arrays (1M pairs): {stopwatch.ElapsedMilliseconds}ms");
    Assert.True(stopwatch.Elapsed.TotalSeconds < 10);
    Assert.Single(result);
  }

  [Fact]
  public void PairwiseDiffQuantile_ManyQuantiles_PerformanceTest()
  {
    var random = AdditiveDistribution.Standard.Random(7777);
    var x = random.Next(200);
    var y = random.Next(200);
    var p = Enumerable.Range(0, 21).Select(i => i / 20.0).ToArray();

    var stopwatch = Stopwatch.StartNew();
    var result = FastShift.Estimate(x, y, p);
    stopwatch.Stop();

    Trace.WriteLine($"200x200 arrays, 21 quantiles: {stopwatch.ElapsedMilliseconds}ms");
    Assert.True(stopwatch.Elapsed.TotalSeconds < 5);
    Assert.Equal(21, result.Length);
  }

  [Fact]
  public void PairwiseDiffQuantile_NullInputs_ThrowsException()
  {
    var x = new double[] { 1, 2 };
    var y = new double[] { 3, 4 };
    var p = new double[] { 0.5 };

    Assert.Throws<ArgumentNullException>(() => FastShift.Estimate(null!, y, p));
    Assert.Throws<ArgumentNullException>(() => FastShift.Estimate(x, null!, p));
    Assert.Throws<ArgumentNullException>(() => FastShift.Estimate(x, y, null!));
  }

  [Fact]
  public void PairwiseDiffQuantile_EmptyArrays_ThrowsException()
  {
    var empty = new double[] { };
    var valid = new double[] { 1, 2 };
    var p = new double[] { 0.5 };

    Assert.Throws<ArgumentException>(() => FastShift.Estimate(empty, valid, p));
    Assert.Throws<ArgumentException>(() => FastShift.Estimate(valid, empty, p));
  }

  [Fact]
  public void PairwiseDiffQuantile_InvalidProbabilities_ThrowsException()
  {
    var x = new double[] { 1, 2 };
    var y = new double[] { 3, 4 };

    Assert.Throws<ArgumentOutOfRangeException>(() =>
      FastShift.Estimate(x, y, new[] { -0.1 }));
    Assert.Throws<ArgumentOutOfRangeException>(() =>
      FastShift.Estimate(x, y, new[] { 1.1 }));
    Assert.Throws<ArgumentOutOfRangeException>(() =>
      FastShift.Estimate(x, y, new[] { double.NaN }));
  }

  [Fact]
  public void PairwiseDiffQuantile_NaNInData_ThrowsException()
  {
    var xWithNaN = new double[] { 1, double.NaN };
    var yWithNaN = new double[] { 3, double.NaN };
    var valid = new double[] { 1, 2 };
    var p = new double[] { 0.5 };

    Assert.Throws<InvalidOperationException>(() =>
      FastShift.Estimate(xWithNaN, valid, p));
    Assert.Throws<InvalidOperationException>(() =>
      FastShift.Estimate(valid, yWithNaN, p));
  }

  [Fact]
  public void PairwiseDiffQuantile_EmptyProbabilities_ReturnsEmpty()
  {
    var x = new double[] { 1, 2 };
    var y = new double[] { 3, 4 };
    var p = new double[] { };

    var result = FastShift.Estimate(x, y, p);

    Assert.Empty(result);
  }

  [Fact]
  public void PairwiseDiffQuantile_ShiftInvariance_XShift()
  {
    var random = AdditiveDistribution.Standard.Random(555);

    for (int trial = 0; trial < 10; trial++)
    {
      var x = random.Next(15);
      var y = random.Next(12);
      var p = new[] { 0.25, 0.5, 0.75 };
      var shift = random.Next() * 10;

      var result1 = FastShift.Estimate(x, y, p);
      var xShifted = x.Select(v => v + shift).ToArray();
      var result2 = FastShift.Estimate(xShifted, y, p);

      for (int i = 0; i < result1.Length; i++)
        Assert.Equal(result1[i] + shift, result2[i], Tolerance);
    }
  }

  [Fact]
  public void PairwiseDiffQuantile_ShiftInvariance_YShift()
  {
    var random = AdditiveDistribution.Standard.Random(666);

    for (int trial = 0; trial < 10; trial++)
    {
      var x = random.Next(15);
      var y = random.Next(12);
      var p = new[] { 0.25, 0.5, 0.75 };
      var shift = random.Next() * 10;

      var result1 = FastShift.Estimate(x, y, p);
      var yShifted = y.Select(v => v + shift).ToArray();
      var result2 = FastShift.Estimate(x, yShifted, p);

      for (int i = 0; i < result1.Length; i++)
        Assert.Equal(result1[i] - shift, result2[i], Tolerance);
    }
  }

  [Fact]
  public void PairwiseDiffQuantile_ScaleInvariance()
  {
    var random = AdditiveDistribution.Standard.Random(777);

    for (int trial = 0; trial < 10; trial++)
    {
      var x = random.Next(15);
      var y = random.Next(12);
      var p = new[] { 0.5 };
      var scale = 2.0;

      var result1 = FastShift.Estimate(x, y, p);
      var xScaled = x.Select(v => v * scale).ToArray();
      var yScaled = y.Select(v => v * scale).ToArray();
      var result2 = FastShift.Estimate(xScaled, yScaled, p);

      for (int i = 0; i < result1.Length; i++)
        Assert.Equal(result1[i] * scale, result2[i], 1e-6);
    }
  }

  private double[] NaiveQuantiles(double[] x, double[] y, double[] p)
  {
    var diffs = new List<double>();
    foreach (var xi in x)
      foreach (var yj in y)
        diffs.Add(xi - yj);

    diffs.Sort();

    var result = new double[p.Length];
    for (int i = 0; i < p.Length; i++)
    {
      int n = diffs.Count;
      double h = 1.0 + (n - 1) * p[i];
      long lo = (long)Math.Floor(h);
      long hi = (long)Math.Ceiling(h);
      double gamma = h - lo;

      lo = Math.Max(1, Math.Min(n, lo));
      hi = Math.Max(1, Math.Min(n, hi));

      double a = diffs[(int)(lo - 1)];
      double b = diffs[(int)(hi - 1)];

      result[i] = gamma == 0.0 ? a : (1.0 - gamma) * a + gamma * b;
    }

    return result;
  }
}
