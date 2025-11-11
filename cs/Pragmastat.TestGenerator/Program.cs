using Pragmastat.Internal;
using Pragmastat.TestGenerator.TestCases;
using Spectre.Console;

AnsiConsole.Write(new Rule("[bold cyan]Reference Tests Generator[/]").RuleStyle("cyan"));
AnsiConsole.MarkupLine("");

// Delete existing test directories to recreate all files from scratch
var repositoryRoot = SourceRepositoryLocator.RepositoryRoot;
var sharedTestsDirectory = Path.Combine(repositoryRoot, "tests");
var csTestsDirectory = Path.Combine(repositoryRoot, "cs", "tests");

if (Directory.Exists(sharedTestsDirectory))
{
  AnsiConsole.MarkupLine($"[red]×[/] Deleting shared tests directory: [dim]{sharedTestsDirectory}[/]");
  Directory.Delete(sharedTestsDirectory, recursive: true);
}

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

AnsiConsole.MarkupLine("");
AnsiConsole.Write(new Rule("[bold green]Generation Completed Successfully![/]").RuleStyle("green"));
