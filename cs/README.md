# Pragmastat

This is a C\# implementation of 'Pragmastat: Pragmatic Statistical Toolkit', which presents a toolkit of statistical procedures that provide reliable results across diverse real-world distributions, with ready-to-use implementations and detailed explanations.

- PDF manual for this version: [pragmastat-v3.2.1.pdf](https://github.com/AndreyAkinshin/pragmastat/releases/download/v3.2.1/pragmastat-v3.2.1.pdf)
- Markdown manual for this version: [pragmastat-v3.2.1.md](https://github.com/AndreyAkinshin/pragmastat/releases/download/v3.2.1/pragmastat-v3.2.1.md)
- Source code for this version: [pragmastat/cs/v3.2.1](https://github.com/AndreyAkinshin/pragmastat/tree/v3.2.1/cs)
- Latest online manual: https://pragmastat.dev
- Manual DOI: [10.5281/zenodo.17236778](https://doi.org/10.5281/zenodo.17236778)

## Installation

Install from NuGet via .NET CLI:

```bash
dotnet add package Pragmastat --version 3.2.1
```

Install from NuGet via Package Manager Console:

```ps1
NuGet\Install-Package Pragmastat -Version 3.2.1
```

## Demo

```cs
using static System.Console;

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
  }
}
```

## The MIT License

Copyright (c) 2025 Andrey Akinshin

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
