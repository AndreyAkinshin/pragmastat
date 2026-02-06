using System.Text.Json;
using Pragmastat.Internal;
using Xunit;

namespace Pragmastat.TestGenerator.Framework;

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

  /// <summary>
  /// Determines if a test case file contains an expected_error field,
  /// indicating it's an error test case rather than a normal test case.
  /// </summary>
  public static bool IsErrorTestCase(string suiteName, string testName, bool shared = false)
  {
    string testSuiteDirectory = GetTestSuiteDirectory(suiteName, shared);
    string filePath = Path.Combine(testSuiteDirectory, testName + ".json");
    string json = File.ReadAllText(filePath);
    using var doc = JsonDocument.Parse(json);
    return doc.RootElement.TryGetProperty("expected_error", out _);
  }
}
