using Pragmastat.Core.Functions;
using Pragmastat.Distributions;
using Pragmastat.ReferenceTests.ReferenceTesting;
using Pragmastat.ReferenceTests.ReferenceTesting.SingleDoubleValue;

namespace Pragmastat.ReferenceTests;

public class ApproximationTests : SingleDoubleValueTestBase
{
    private const string SuiteName = "approximations";
    protected override string GetSuiteName() => SuiteName;

    public ApproximationTests()
    {
        double[] milliles = Uniform(0, 1, 1001, 1);
        double[] normalMilliles = milliles.Select(p => AdditiveDistribution.Standard.Quantile(p)).ToArray();
        RegisterFunction("acm209", AcmAlgorithm209.Gauss, normalMilliles);
        RegisterFunction("erf", AbramowitzStegunErf.Value, normalMilliles);
        RegisterFunction("erf_inverse", ErfInverse.Value, Uniform(-1, 1, 1001, 1));
    }

    private static double[] Uniform(double l, double r, int count, int trim = 0)
    {
        return Enumerable.Range(0, count)
            .Select(i => 1.0 * (l * (count - 1 - i) + r * i) / (count - 1))
            .Skip(trim)
            .SkipLast(trim)
            .ToArray();
    }

    public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName);

    [Theory]
    [MemberData(nameof(TestDataNames))]
    public void ApproximationTest(string testName) => PerformTest(testName);
}