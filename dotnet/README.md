# Pragmastat .NET

A .NET implementation of the Pragmastat statistical toolkit - robust estimators designed for real-world data analysis.

## Installation

```bash
dotnet add package Pragmastat
```

Or via Package Manager Console:

```powershell
Install-Package Pragmastat
```

## Quick Start

```csharp
using Pragmastat.Core;

var x = new Sample(1, 2, 3, 4, 5, 6, 273);
WriteLine(x.Center()); // 4
WriteLine(x.Spread()); // 3
WriteLine(x.RelSpread()); // 0.75

WriteLine(Toolkit.Shift(x, x - 10)); // 10
WriteLine(Toolkit.Ratio(x, x / 10)); // 10

x = new Sample(-3, -2, -1, 0, 1, 2, 3);
WriteLine(Toolkit.Disparity(x, x * 10)); // 0
WriteLine(Toolkit.Disparity(x, x - 10)); // 5
WriteLine(Toolkit.Disparity(x * 10, x * 10 - 100)); // 5
```

## Package Structure

The Pragmastat .NET package consists of two main components:

- **Pragmastat**: Main package with high-level API
- **Pragmastat.Core**: Core types and interfaces

## API Overview

### Two Ways to Use the API

**Option 1: Extension Methods**
```csharp
var center = data.Center();
var shift = sample1.Shift(sample2);
```

**Option 2: Static Toolkit Methods**
```csharp
var center = Toolkit.Center(data);
var shift = Toolkit.Shift(sample1, sample2);
```

### Available Estimators

**One-Sample Estimators:**
- `Center()` - Robust central tendency
- `Spread()` - Robust dispersion
- `RelSpread()` - Relative dispersion

**Two-Sample Estimators:**
- `Shift()` - Robust difference
- `Ratio()` - Robust ratio
- `AvgSpread()` - Pooled dispersion
- `Disparity()` - Robust effect size

## Platform Support

- **.NET Standard 2.0** - Compatible with .NET Framework 4.6.1+, .NET Core 2.0+
- **.NET 6.0+** - Modern .NET support with enhanced performance

## Documentation

For detailed information about the statistical properties, mathematical formulations, and theoretical background of the estimators, see the **[Pragmastat Manual](https://github.com/AndreyAkinshin/pragmastat)**.

## License

MIT License - see LICENSE file for details.
