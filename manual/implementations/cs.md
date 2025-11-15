<span id="cs"></span> <!-- [pdf] DELETE -->

## C\#

Install from NuGet via .NET CLI:

```bash
dotnet add package Pragmastat --version 4.0.0
```

Install from NuGet via Package Manager Console:

```ps1
NuGet\Install-Package Pragmastat -Version 4.0.0
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v4.0.0/cs

Pragmastat on NuGet: https://www.nuget.org/packages/Pragmastat/

Demo:

```cs
using static System.Console;
using Pragmastat.Functions;

namespace Pragmastat.Demo;

class Program
{
  static void Main()
  {
    var x = new Sample(0, 2, 4, 6, 8);
    WriteLine(x.Center()); // 4
    WriteLine((x + 10).Center()); // 14
    WriteLine((x * 3).Center()); // 12

    WriteLine(x.Spread()); // 4
    WriteLine((x + 10).Spread()); // 4
    WriteLine((x * 2).Spread()); // 8

    WriteLine(x.RelSpread()); // 1
    WriteLine((x * 5).RelSpread()); // 1

    var y = new Sample(10, 12, 14, 16, 18);
    WriteLine(Toolkit.Shift(x, y)); // -10
    WriteLine(Toolkit.Shift(x, x)); // 0
    WriteLine(Toolkit.Shift(x + 7, y + 3)); // -6
    WriteLine(Toolkit.Shift(x * 2, y * 2)); // -20
    WriteLine(Toolkit.Shift(y, x)); // 10

    x = new Sample(1, 2, 4, 8, 16);
    y = new Sample(2, 4, 8, 16, 32);
    WriteLine(Toolkit.Ratio(x, y)); // 0.5
    WriteLine(Toolkit.Ratio(x, x)); // 1
    WriteLine(Toolkit.Ratio(x * 2, y * 5)); // 0.2

    x = new Sample(0, 3, 6, 9, 12);
    y = new Sample(0, 2, 4, 6, 8);
    WriteLine(x.Spread()); // 6
    WriteLine(y.Spread()); // 4

    WriteLine(Toolkit.AvgSpread(x, y)); // 5
    WriteLine(Toolkit.AvgSpread(x, x)); // 6
    WriteLine(Toolkit.AvgSpread(x * 2, x * 3)); // 15
    WriteLine(Toolkit.AvgSpread(y, x)); // 5
    WriteLine(Toolkit.AvgSpread(x * 2, y * 2)); // 10

    WriteLine(Toolkit.Shift(x, y)); // 2
    WriteLine(Toolkit.AvgSpread(x, y)); // 5

    WriteLine(Toolkit.Disparity(x, y)); // 0.4
    WriteLine(Toolkit.Disparity(x + 5, y + 5)); // 0.4
    WriteLine(Toolkit.Disparity(x * 2, y * 2)); // 0.4
    WriteLine(Toolkit.Disparity(y, x)); // -0.4

    x = new Sample(
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
      16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30);
    y = new Sample(
      21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
      36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50);

    WriteLine(PairwiseMargin.Instance.Calc(30, 30, 1e-6)); // 276
    WriteLine(PairwiseMargin.Instance.Calc(30, 30, 1e-5)); // 328
    WriteLine(PairwiseMargin.Instance.Calc(30, 30, 1e-4)); // 390
    WriteLine(PairwiseMargin.Instance.Calc(30, 30, 1e-3)); // 464

    WriteLine(Toolkit.Shift(x, y)); // -20

    WriteLine(Toolkit.ShiftBounds(x, y, 1e-6)); // [-33, -7]
    WriteLine(Toolkit.ShiftBounds(x, y, 1e-5)); // [-32, -8]
    WriteLine(Toolkit.ShiftBounds(x, y, 1e-4)); // [-30, -10]
    WriteLine(Toolkit.ShiftBounds(x, y, 1e-3)); // [-28, -12]
  }
}
```