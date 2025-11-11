namespace Pragmastat.TestGenerator.Framework.TwoSample;

public abstract class TwoSampleEstimatorTestBase : ReferenceTestBase<TwoSampleInput, double>
{
  protected abstract double Estimate(TwoSampleInput input);

  protected override ReferenceTestController<TwoSampleInput, double> CreateController() =>
    new TwoSampleEstimatorController(GetSuiteName(), Estimate);
}
