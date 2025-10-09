namespace Pragmastat.ReferenceTests.ReferenceTesting.TwoSample;

public class TwoSampleEstimatorController(string name, Func<TwoSampleInput, double> estimate, double eps = 1e-9)
  : ReferenceTestController<TwoSampleInput, double>(shared: true)
{
  protected override string SuiteName { get; } = name;
  public override bool Assert(double expected, double actual) => Math.Abs(expected - actual) < eps;
  public override double Run(TwoSampleInput input) => estimate(input);
}
