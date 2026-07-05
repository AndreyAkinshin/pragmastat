using System.Text.Json;
using Pragmastat.Exceptions;
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

  /// <summary>Entry-point labels for the dual-path reference tests.</summary>
  public const string EntryPointRaw = "raw";
  public const string EntryPointSample = "sample";

  /// <summary>
  /// Produces theory data that runs every fixture through BOTH the raw native-array entry point
  /// (with assumeSorted=false) and the Sample entry point. This catches Sample-adapter bugs that a
  /// raw-only loop would miss.
  /// </summary>
  public static TheoryData<string, string> GetDualPathTheoryData(string suiteName, bool shared = false)
  {
    var testCaseNames = LoadTestNames(suiteName, shared);

    var data = new TheoryData<string, string>();
    foreach (string value in testCaseNames)
    {
      data.Add(value, EntryPointRaw);
      data.Add(value, EntryPointSample);
    }
    return data;
  }

  /// <summary>
  /// Asserts that a thrown <see cref="AssumptionException"/> matches the fixture's expected error.
  /// The id is always checked. The subject is checked except on the Sample entry point for
  /// sample-construction validity errors whose fixture attributes them to subject "y":
  /// Sample construction cannot know the argument position, so a y-argument validity error
  /// surfaces from construction with the fixed subject "x". The raw entry point always validates
  /// positionally, so its subject is asserted in full.
  /// </summary>
  public static void AssertErrorMatches(ExpectedError expected, AssumptionException ex, string entryPoint)
  {
    Assert.Equal(expected.Id, ex.Violation.IdString);

    bool skipSubject = entryPoint == EntryPointSample &&
                       expected.Id == "validity" &&
                       expected.Subject == "y";
    if (!skipSubject)
      Assert.Equal(expected.Subject, ex.Violation.Subject.ToString().ToLower());
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
