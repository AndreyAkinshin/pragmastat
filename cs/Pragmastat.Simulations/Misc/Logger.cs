using Spectre.Console;

namespace Pragmastat.Simulations.Misc;

public static class Logger
{
  public static void Error(string message)
  {
    AnsiConsole.MarkupLine($"[red]{message}[/]");
  }
}
