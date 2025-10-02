# Pragmastat

A .NET implementation of 'Pragmastat: Pragmatic Statistical Toolkit' - robust summary estimators designed for real-world data analysis.
Online manual: https://pragmastat.dev

## Installation

```bash
dotnet add package Pragmastat
```

Or via Package Manager Console:

```powershell
Install-Package Pragmastat
```

## Demo

```csharp
using Pragmastat;

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
```

## Platform Support

- **.NET Standard 2.0** - Compatible with .NET Framework 4.6.1+, .NET Core 2.0+
- **.NET 6.0+** - Modern .NET support with enhanced performance

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
