using System.Diagnostics;

namespace Pragmastat.ReferenceTests.ReferenceTesting;

public abstract class ReferenceTestBase<TInput, TOutput>
{
    private readonly Lazy<ReferenceTestController<TInput, TOutput>> lazyController;
    private ReferenceTestController<TInput, TOutput> Controller => lazyController.Value;

    protected abstract string GetSuiteName();
    protected abstract ReferenceTestController<TInput, TOutput> CreateController();
    protected abstract ReferenceTestCaseInputBuilder<TInput> GetInputBuilder();

    protected ReferenceTestBase()
    {
        lazyController = new Lazy<ReferenceTestController<TInput, TOutput>>(CreateController);
    }

    // Remove 'Skip' to generate initial test data
    //[Fact(Skip = "Run this tests explicitly for the initial test case generation")]
    [Fact]
    public void GenerateTests()
    {
        var inputs = GetInputBuilder().Build();
        var testData = Controller.GenerateData(inputs);
        Controller.Save(testData);
    }

    protected void PerformTest(string testName)
    {
        var testCase = Controller.LoadTestCase(testName);
        Trace.WriteLine("=== TestCase ===");
        Trace.WriteLine(Controller.Serialize(testCase));

        var actual = Controller.Run(testCase.Input);
        Trace.WriteLine("=== ActualOutput ===");
        Trace.WriteLine(Controller.Serialize(actual));

        Assert.True(Controller.Assert(testCase.Output, actual));
    }
}