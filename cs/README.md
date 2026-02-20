# C#

Install from NuGet via .NET CLI:

```bash
dotnet add package Pragmastat --version 10.0.4
```

Install from NuGet via Package Manager Console:

```ps1
NuGet\Install-Package Pragmastat -Version 10.0.4
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v10.0.4/cs

Pragmastat on NuGet: https://www.nuget.org/packages/Pragmastat/

## Demo

```cs
using static System.Console;
using Pragmastat.Distributions;
using Pragmastat.Randomization;

namespace Pragmastat.Demo;

class Program
{
  static void Main()
  {
    // --- One-Sample ---

    var x = new Sample(
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
      11, 12, 13, 14, 15, 16, 17, 18, 19, 20);

    WriteLine(Toolkit.Center(x));             // 10.5
    WriteLine(Toolkit.CenterBounds(x, 0.05)); // [7.5;13.5]
    WriteLine(Toolkit.Spread(x));             // 6
    WriteLine(Toolkit.SpreadBounds(x, 0.05, "demo")); // [2;10]

    // --- Two-Sample ---

    x = new Sample(
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
      16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30);
    var y = new Sample(
      21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
      36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50);

    WriteLine(Toolkit.Shift(x, y));             // -20
    WriteLine(Toolkit.ShiftBounds(x, y, 0.05)); // [-25;-15]
    WriteLine(Toolkit.Ratio(x, y));             // 0.43669798282695127
    WriteLine(Toolkit.RatioBounds(x, y, 0.05)); // [0.31250000000000006;0.5599999999999999]
    WriteLine(Toolkit.Disparity(x, y));         // -2.2222222222222223
    WriteLine(Toolkit.DisparityBounds(x, y, 0.05, "demo")); // [-13;-0.8235294117647058]

    // --- Randomization ---

    var rng = new Rng("demo-uniform");
    WriteLine(rng.UniformDouble()); // 0.2640554428629759
    WriteLine(rng.UniformDouble()); // 0.9348534835582796

    rng = new Rng("demo-uniform-int");
    WriteLine(rng.UniformInt32(0, 100)); // 41

    rng = new Rng("demo-sample");
    WriteLine(string.Join(", ", rng.Sample([0.0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3))); // 3, 8, 9

    rng = new Rng("demo-resample");
    WriteLine(string.Join(", ", rng.Resample([1.0, 2, 3, 4, 5], 7))); // 3, 1, 3, 2, 4, 1, 2

    rng = new Rng("demo-shuffle");
    WriteLine(string.Join(", ", rng.Shuffle([1.0, 2, 3, 4, 5]))); // 4, 2, 3, 5, 1

    // --- Distributions ---

    rng = new Rng("demo-dist-additive");
    WriteLine(new Additive(0, 1).Sample(rng)); // 0.17410448679568188

    rng = new Rng("demo-dist-multiplic");
    WriteLine(new Multiplic(0, 1).Sample(rng)); // 1.1273244602673853

    rng = new Rng("demo-dist-exp");
    WriteLine(new Exp(1).Sample(rng)); // 0.6589065267276553

    rng = new Rng("demo-dist-power");
    WriteLine(new Power(1, 2).Sample(rng)); // 1.023677535537084

    rng = new Rng("demo-dist-uniform");
    WriteLine(new Uniform(0, 10).Sample(rng)); // 6.54043657816832
  }
}
```
