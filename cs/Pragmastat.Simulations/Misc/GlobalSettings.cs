using System.Text.Json;
using Pragmastat.Internal;

namespace Pragmastat.Simulations.Misc;

public static class GlobalSettings
{
  public static string GetSimulationRoot(bool publish)
  {
    string publishRoot = Path.Combine(SourceRepositoryLocator.RepositoryRoot, "sim");
    return publish
      ? publishRoot
      : Path.Combine(publishRoot, "cs");
  }

  public static readonly JsonSerializerOptions JsonOptions = new()
  {
    PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
    WriteIndented = true
  };
}
