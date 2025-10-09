using Pragmastat.Internal;

namespace Pragmastat.ReferenceTests.ReferenceTesting;

public static class ReferenceTestSuiteHelper
{
    public static string GetTestSuiteDirectory(string suiteName, bool shared = false)
    {
        return shared
            ? Path.Combine(SourceRepositoryLocator.RepositoryRoot, "tests", suiteName)
            : Path.Combine(SourceRepositoryLocator.RepositoryRoot, "cs", "tests", suiteName);
    }

    private static string[] LoadTestNames(string suiteName, bool shared = false)
    {
        var testSuiteDirectory = GetTestSuiteDirectory(suiteName, shared);
        var fileNames = Directory.GetFiles(testSuiteDirectory, "*.json", SearchOption.TopDirectoryOnly);
        return fileNames
            .Select(Path.GetFileNameWithoutExtension)
            .ToArray()!;
    }

    public static TheoryData<string> GetTheoryData(string suiteName, bool shared = false)
    {
        var testCastNames = LoadTestNames(suiteName, shared);

        var data = new TheoryData<string>();
        foreach (string value in testCastNames)
            data.Add(value);
        return data;
    }
}