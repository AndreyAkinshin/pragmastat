namespace Pragmastat.ReferenceTests.ReferenceTesting.OneSample;

public abstract class OneSampleEstimatorTestBase : ReferenceTestBase<OneSampleInput, double>
{
    protected abstract double Estimate(OneSampleInput input);

    protected override ReferenceTestController<OneSampleInput, double> CreateController() =>
        new OneSampleEstimatorController(GetSuiteName(), Estimate);
}