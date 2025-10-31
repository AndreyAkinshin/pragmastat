namespace Pragmastat.ReferenceTests.Generator.Framework.OneSample;

public class OneSampleEstimatorController(string name, Func<OneSampleInput, double> estimate, double eps = 1e-9)
  : ReferenceTestController<OneSampleInput, double>(shared: true)
{
  protected override string SuiteName { get; } = name;
  public override bool Assert(double expected, double actual) => Math.Abs(expected - actual) < eps;
  public override double Run(OneSampleInput input) => estimate(input);
}
