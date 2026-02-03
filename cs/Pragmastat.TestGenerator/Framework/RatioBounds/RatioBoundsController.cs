namespace Pragmastat.TestGenerator.Framework.RatioBounds;

public class RatioBoundsController(string name, double eps = 1e-9)
  : ReferenceTestController<RatioBoundsInput, RatioBoundsOutput>(shared: true)
{
  protected override string SuiteName { get; } = name;

  public override bool Assert(RatioBoundsOutput expected, RatioBoundsOutput actual)
  {
    return Math.Abs(expected.Lower - actual.Lower) < eps &&
           Math.Abs(expected.Upper - actual.Upper) < eps;
  }

  public override RatioBoundsOutput Run(RatioBoundsInput input)
  {
    var bounds = Toolkit.RatioBounds(input.GetSampleX(), input.GetSampleY(), new Probability(input.Misrate));
    return new RatioBoundsOutput(bounds);
  }
}
