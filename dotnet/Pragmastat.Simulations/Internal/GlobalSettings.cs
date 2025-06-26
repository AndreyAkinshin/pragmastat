using System.Text.Json;
using Pragmastat.Core.Internal;

namespace Pragmastat.Simulations.Internal;

public static class GlobalSettings
{
    public static string GetSimulationRoot(bool publish)
    {
        string publishRoot = Path.Combine(SourceRepositoryLocator.RepositoryRoot, "simulations");
        return publish
            ? publishRoot
            : Path.Combine(publishRoot, "dotnet");
    }

    public static readonly JsonSerializerOptions JsonOptions = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
        WriteIndented = true
    };
}