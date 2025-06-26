using System.Reflection;

namespace Pragmastat.Core.Internal;

/// <summary>
/// Development-only helper class for tests and simulations
/// </summary>
internal static class SourceRepositoryLocator
{
    private static readonly Lazy<string> LazyRepositoryRoot = new(() => GetRepositoryRoot());
    public static string RepositoryRoot => LazyRepositoryRoot.Value;

    private static string GetRepositoryRoot(string rootMarkerName = "build.cmd")
    {
        var dir = Path.GetDirectoryName(Assembly.GetCallingAssembly().Location);
        while (dir != null)
        {
            var rootMarkerPath = Path.Combine(dir, rootMarkerName);
            if (File.Exists(rootMarkerPath))
                return dir;

            dir = Path.GetDirectoryName(dir);
        }

        throw new DirectoryNotFoundException(
            $"Could not find '{rootMarkerName}' directory in parent path of {Assembly.GetCallingAssembly().Location}");
    }
}