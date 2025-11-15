using System.Diagnostics;
using Xunit;

namespace Pragmastat.TestGenerator.Framework;

public abstract class ReferenceTestBase<TInput, TOutput>
{
  private readonly Lazy<ReferenceTestController<TInput, TOutput>> lazyController;
  private ReferenceTestController<TInput, TOutput> Controller => lazyController.Value;

  /// <summary>
  /// Timeout for each individual test execution (5 seconds as per documentation)
  /// </summary>
  protected virtual TimeSpan TestTimeout { get; } = TimeSpan.FromSeconds(5);

  protected abstract string GetSuiteName();
  protected abstract ReferenceTestController<TInput, TOutput> CreateController();
  protected abstract ReferenceTestCaseInputBuilder<TInput> GetInputBuilder();

  protected ReferenceTestBase()
  {
    lazyController = new Lazy<ReferenceTestController<TInput, TOutput>>(CreateController);
  }

  protected void PerformTest(string testName)
  {
    var testCase = Controller.LoadTestCase(testName);
    Trace.WriteLine("=== TestCase ===");
    Trace.WriteLine(Controller.Serialize(testCase));

    var actual = RunWithTimeout(testCase.Input, testName);
    Trace.WriteLine("=== ActualOutput ===");
    Trace.WriteLine(Controller.Serialize(actual));

    Assert.True(Controller.Assert(testCase.Output, actual));
  }

  /// <summary>
  /// Runs the test with a timeout. Throws TimeoutException if test exceeds the timeout.
  /// </summary>
  private TOutput RunWithTimeout(TInput input, string testName)
  {
    var task = Task.Run(() => Controller.Run(input));
    if (task.Wait(TestTimeout))
    {
      return task.Result;
    }

    throw new TimeoutException(
      $"Test '{testName}' in suite '{GetSuiteName()}' exceeded timeout of {TestTimeout.TotalSeconds} seconds. " +
      $"Performance tests must complete in under 5 seconds as per documentation.");
  }
}
