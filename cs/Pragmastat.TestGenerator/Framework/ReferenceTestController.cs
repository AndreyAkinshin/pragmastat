using System.Diagnostics.CodeAnalysis;
using System.Runtime.Serialization;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace Pragmastat.TestGenerator.Framework;

public abstract class ReferenceTestController<TInput, TOutput>
{
  private readonly string testSuiteDirectory;

  /// <summary>
  /// Timeout for each individual test execution (5 seconds as per documentation)
  /// </summary>
  protected virtual TimeSpan TestTimeout { get; } = TimeSpan.FromSeconds(5);

  [SuppressMessage("ReSharper", "VirtualMemberCallInConstructor")]
  protected ReferenceTestController(string? testDataDirectory = null, bool shared = false)
  {
    testSuiteDirectory = testDataDirectory ?? ReferenceTestSuiteHelper.GetTestSuiteDirectory(SuiteName, shared);
  }

  protected abstract string SuiteName { get; }
  public abstract bool Assert(TOutput expected, TOutput actual);
  public abstract TOutput Run(TInput input);

  public IReadOnlyDictionary<string, ReferenceTestCase<TInput, TOutput>> GenerateData(
    IReadOnlyDictionary<string, TInput> inputs)
  {
    var result = new Dictionary<string, ReferenceTestCase<TInput, TOutput>>();
    foreach ((string testName, var input) in inputs)
    {
      var output = RunWithTimeout(input, testName);
      result[testName] = new ReferenceTestCase<TInput, TOutput>(input, output);
    }

    return result;
  }

  /// <summary>
  /// Runs the test with a timeout. Throws TimeoutException if test exceeds the timeout.
  /// </summary>
  private TOutput RunWithTimeout(TInput input, string testName)
  {
    var task = Task.Run(() => Run(input));
    if (task.Wait(TestTimeout))
    {
      return task.Result;
    }

    throw new TimeoutException(
      $"Test '{testName}' in suite '{SuiteName}' exceeded timeout of {TestTimeout.TotalSeconds} seconds. " +
      $"Performance tests must complete in under 5 seconds as per documentation.");
  }

  public ReferenceTestCase<TInput, TOutput> LoadTestCase(string testName)
  {
    string filePath = Path.Combine(testSuiteDirectory, testName + ".json");
    string testCaseJson = File.ReadAllText(filePath);
    return Deserialize<ReferenceTestCase<TInput, TOutput>>(testCaseJson);
  }

  public void Save(IReadOnlyDictionary<string, ReferenceTestCase<TInput, TOutput>> data)
  {
    if (!Directory.Exists(testSuiteDirectory))
      Directory.CreateDirectory(testSuiteDirectory);
    foreach ((string testName, var testCase) in data)
    {
      string filePath = Path.Combine(testSuiteDirectory, testName + ".json");
      string json = Serialize(testCase);
      File.WriteAllText(filePath, json);
    }
  }


  private readonly JsonSerializerOptions jsonOptions = new()
  {
    DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
    PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
    WriteIndented = true
  };

  public string Serialize<T>(T value) => JsonSerializer.Serialize(value, jsonOptions);

  private T Deserialize<T>(string value) => JsonSerializer.Deserialize<T>(value, jsonOptions)
                                            ?? throw new SerializationException($"Failed to deserialize:\n{value}");

  protected static bool AreEqual(double[]? x, double[]? y, double eps)
  {
    if (x == null && y == null)
      return true;
    if (x == null || y == null)
      return false;
    if (x.Length != y.Length)
      return false;

    for (int i = 0; i < x.Length; i++)
      if (Math.Abs(x[i] - y[i]) > eps)
        return false;

    return true;
  }
}
