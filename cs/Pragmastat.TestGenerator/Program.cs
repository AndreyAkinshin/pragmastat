using Pragmastat.Internal;
using Pragmastat.TestGenerator.TestCases;
using Spectre.Console;

AnsiConsole.Write(new Rule("[bold cyan]Reference Tests Generator[/]").RuleStyle("cyan"));
AnsiConsole.MarkupLine("");

// Recreate only the suites this generator owns (see tests/manifest.json).
// The shared tests/ directory also holds suites owned by the Rust generator
// (rng, shuffle, sample, resample, distributions), hand-maintained suites
// (sample-construction, unit-propagation), and the README/manifest metadata —
// none of which may be touched here. Deleting a suite directory (rather than
// overwriting files) removes cases orphaned by renames.
var repositoryRoot = SourceRepositoryLocator.RepositoryRoot;

string[] ownedSharedSuites =
[
  "center", "center-bounds", "spread", "spread-bounds",
  "shift", "shift-bounds", "ratio", "ratio-bounds",
  "disparity", "disparity-bounds", "avg-spread", "avg-spread-bounds",
  "compare1", "compare2", "pairwise-margin", "signed-rank-margin",
];

foreach (string suite in ownedSharedSuites)
{
  var suiteDirectory = Path.Combine(repositoryRoot, "tests", suite);
  if (Directory.Exists(suiteDirectory))
  {
    AnsiConsole.MarkupLine($"[red]×[/] Deleting owned suite directory: [dim]{suiteDirectory}[/]");
    Directory.Delete(suiteDirectory, recursive: true);
  }
}

// The cs/tests directory is exclusively owned by this generator.
var csTestsDirectory = Path.Combine(repositoryRoot, "cs", "tests");
if (Directory.Exists(csTestsDirectory))
{
  AnsiConsole.MarkupLine($"[red]×[/] Deleting C# tests directory: [dim]{csTestsDirectory}[/]");
  Directory.Delete(csTestsDirectory, recursive: true);
}

AnsiConsole.MarkupLine("");

OneSampleTestCases.Generate();
TwoSampleTestCases.Generate();
DistributionTestCases.Generate();
ApproximationTestCases.Generate();
PairwiseMarginTestCases.Generate();
ShiftBoundsTestCases.Generate();
RatioBoundsTestCases.Generate();
SignedRankMarginTestCases.Generate();
CenterBoundsTestCases.Generate();
SpreadBoundsTestCases.Generate();
AvgSpreadBoundsTestCases.Generate();
DisparityBoundsTestCases.Generate();
Compare1TestCases.Generate();
Compare2TestCases.Generate();
AnsiConsole.MarkupLine("");
AnsiConsole.Write(new Rule("[bold green]Generation Completed Successfully![/]").RuleStyle("green"));
