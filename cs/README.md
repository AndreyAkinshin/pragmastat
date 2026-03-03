# C#

Install from NuGet via .NET CLI:

```bash
dotnet add package Pragmastat --version 11.1.1
```

Install from NuGet via Package Manager Console:

```ps1
NuGet\Install-Package Pragmastat -Version 11.1.1
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v11.1.1/cs

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

    var x = new Sample(Enumerable.Range(1, 200).Select(i => (double)i).ToArray());

    WriteLine(x.Center());                    // 100.5
    WriteLine(x.CenterBounds());              // [86;115]
    WriteLine(x.Spread());                    // 59
    WriteLine(x.SpreadBounds(1e-3, "demo")); // [44;87]

    // --- Two-Sample ---

    x = new Sample(Enumerable.Range(1, 200).Select(i => (double)i).ToArray());
    var y = new Sample(Enumerable.Range(101, 200).Select(i => (double)i).ToArray());

    WriteLine(x.Shift(y));                        // -100
    WriteLine(x.ShiftBounds(y));                  // [-120;-80]
    WriteLine(x.Ratio(y));                        // 0.5008354224706334
    WriteLine(x.RatioBounds(y));                  // [0.4066666666666668;0.5958333333333332]
    WriteLine(x.Disparity(y));                    // -1.694915254237288
    WriteLine(x.DisparityBounds(y, 1e-3, "demo")); // [-3.1025641025641026;-0.8494623655913979]

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
