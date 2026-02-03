using static System.Console;
using Pragmastat.Distributions;
using Pragmastat.Functions;
using Pragmastat.Randomization;

namespace Pragmastat.Demo;

class Program
{
  static void Main()
  {
    // --- Randomization ---

    var rng = new Rng(1729);
    WriteLine(rng.Uniform()); // 0.3943034703296536
    WriteLine(rng.Uniform()); // 0.5730893757071377

    rng = new Rng("experiment-1");
    WriteLine(rng.Uniform()); // 0.9535207726895857

    rng = new Rng(1729);
    WriteLine(string.Join(", ", rng.Sample([0.0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3))); // 6, 8, 9

    rng = new Rng(1729);
    WriteLine(string.Join(", ", rng.Shuffle([1.0, 2, 3, 4, 5]))); // 4, 2, 3, 5, 1

    // --- Distribution Sampling ---

    rng = new Rng(1729);
    IDistribution dist = new Uniform(0, 10);
    WriteLine(dist.Sample(rng)); // 3.9430347032965365

    rng = new Rng(1729);
    dist = new Additive(0, 1);
    WriteLine(dist.Sample(rng)); // -1.222932972163442

    rng = new Rng(1729);
    dist = new Exp(1);
    WriteLine(dist.Sample(rng)); // 0.5013761944646019

    rng = new Rng(1729);
    dist = new Power(1, 2);
    WriteLine(dist.Sample(rng)); // 1.284909255071668

    rng = new Rng(1729);
    dist = new Multiplic(0, 1);
    WriteLine(dist.Sample(rng)); // 0.2943655336550937

    // --- Single-Sample Statistics ---

    var x = new Sample(0, 2, 4, 6, 8);

    WriteLine(Toolkit.Median(x)); // 4
    WriteLine(x.Center()); // 4
    WriteLine(x.Spread()); // 4
    WriteLine((x + 10).Spread()); // 4
    WriteLine((x * 2).Spread()); // 8
    WriteLine(x.RelSpread()); // 1

    // --- Two-Sample Comparison ---

    x = new Sample(0, 3, 6, 9, 12);
    var y = new Sample(0, 2, 4, 6, 8);

    WriteLine(Toolkit.Shift(x, y)); // 2
    WriteLine(Toolkit.Shift(y, x)); // -2
    WriteLine(Toolkit.AvgSpread(x, y)); // 5
    WriteLine(Toolkit.Disparity(x, y)); // 0.4
    WriteLine(Toolkit.Disparity(y, x)); // -0.4

    x = new Sample(1, 2, 4, 8, 16);
    y = new Sample(2, 4, 8, 16, 32);
    WriteLine(Toolkit.Ratio(x, y)); // 0.5
    WriteLine(Toolkit.Ratio(y, x)); // 2

    // --- Confidence Bounds ---

    x = new Sample(
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
      16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30);
    y = new Sample(
      21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
      36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50);

    WriteLine(PairwiseMargin.Instance.Calc(30, 30, 1e-4)); // 390
    WriteLine(Toolkit.Shift(x, y)); // -20
    WriteLine(Toolkit.ShiftBounds(x, y, 1e-4)); // [-30, -10]

    x = new Sample(1, 2, 3, 4, 5);
    y = new Sample(2, 3, 4, 5, 6);
    WriteLine(Toolkit.RatioBounds(x, y, 0.05)); // [0.333..., 1.5]
  }
}
