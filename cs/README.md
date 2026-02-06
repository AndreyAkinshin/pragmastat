# C#

Install from NuGet via .NET CLI:

```bash
dotnet add package Pragmastat --version 6.0.1
```

Install from NuGet via Package Manager Console:

```ps1
NuGet\Install-Package Pragmastat -Version 6.0.1
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v6.0.1/cs

Pragmastat on NuGet: https://www.nuget.org/packages/Pragmastat/

## Demo

```cs
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

    var rng = new Rng("demo-uniform");
    WriteLine(rng.Uniform()); // 0.2640554428629759
    WriteLine(rng.Uniform()); // 0.9348534835582796

    rng = new Rng("demo-sample");
    WriteLine(string.Join(", ", rng.Sample([0.0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3))); // 3, 8, 9

    rng = new Rng("demo-shuffle");
    WriteLine(string.Join(", ", rng.Shuffle([1.0, 2, 3, 4, 5]))); // 4, 2, 3, 5, 1

    rng = new Rng("demo-resample");
    WriteLine(string.Join(", ", rng.Resample([1.0, 2, 3, 4, 5], 7))); // 5, 1, 1, 3, 3, 4, 5

    // --- Distribution Sampling ---

    rng = new Rng("demo-dist-uniform");
    IDistribution dist = new Uniform(0, 10);
    WriteLine(dist.Sample(rng)); // 6.54043657816832

    rng = new Rng("demo-dist-additive");
    dist = new Additive(0, 1);
    WriteLine(dist.Sample(rng)); // 0.17410448679568188

    rng = new Rng("demo-dist-exp");
    dist = new Exp(1);
    WriteLine(dist.Sample(rng)); // 0.6589065267276553

    rng = new Rng("demo-dist-power");
    dist = new Power(1, 2);
    WriteLine(dist.Sample(rng)); // 1.023677535537084

    rng = new Rng("demo-dist-multiplic");
    dist = new Multiplic(0, 1);
    WriteLine(dist.Sample(rng)); // 1.1273244602673853

    // --- Single-Sample Statistics ---

    var x = new Sample(1, 3, 5, 7, 9);

    WriteLine(Toolkit.Median(x)); // 5
    WriteLine(x.Center()); // 5
    WriteLine(x.Spread()); // 4
    WriteLine((x + 10).Spread()); // 4
    WriteLine((x * 2).Spread()); // 8
    WriteLine(x.RelSpread()); // 0.8

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

    // --- One-Sample Bounds ---

    x = new Sample(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

    WriteLine(SignedRankMargin.Instance.Calc(10, 0.05)); // 18
    WriteLine(Toolkit.Center(x)); // 5.5
    WriteLine(Toolkit.CenterBounds(x, 0.05)); // [3.5, 7.5]
    WriteLine(Toolkit.MedianBounds(x, 0.05)); // [2, 9]
    WriteLine(Toolkit.CenterBoundsApprox(x, 0.05)); // [3.5, 7.5] (approximate)

    // --- Two-Sample Bounds ---

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
```
