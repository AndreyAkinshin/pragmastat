<span id="dotnet"></span> <!-- [pdf] DELETE -->

## .NET

Source code of the latest version: https://github.com/AndreyAkinshin/pragmastat/tree/main/dotnet

NuGet package is not available yet.

The .NET implementation provides the complete statistical toolkit as a modern C# library.

The library offers two usage patterns: instance methods on `Sample` objects for one-sample estimators and static
  methods on the `Toolkit` class for two-sample estimators.
This design follows .NET conventions while maintaining mathematical clarity.

Demo in C#:

```cs
<!-- INCLUDE dotnet/Pragmastat.Demo/Program.cs -->
```