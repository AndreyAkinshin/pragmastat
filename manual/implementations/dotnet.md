---
title: Pragmastat / .NET
url: dotnet
---

The .NET implementation provides the complete statistical toolkit as a modern C# library.

The library offers two usage patterns: instance methods on `Sample` objects for one-sample estimators and static
  methods on the `Toolkit` class for two-sample estimators.
This design follows .NET conventions while maintaining mathematical clarity.

Demo:

```cs
<!-- INCLUDE dotnet/Pragmastat.Demo/Program.cs -->
```