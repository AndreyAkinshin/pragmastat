namespace Pragmastat.TestGenerator.Framework.DisparityBounds;

public class DisparityBoundsController(string name, double eps = 1e-9)
  : ReferenceTestController<DisparityBoundsInput, DisparityBoundsOutput>(shared: true)
{
  protected override string SuiteName { get; } = name;

  public override bool Assert(DisparityBoundsOutput expected, DisparityBoundsOutput actual)
  {
    return Math.Abs(expected.Lower - actual.Lower) < eps &&
           Math.Abs(expected.Upper - actual.Upper) < eps;
  }

  public override DisparityBoundsOutput Run(DisparityBoundsInput input)
  {
    var bounds = input.Seed != null
      ? Toolkit.DisparityBounds(input.GetSampleX(), input.GetSampleY(), new Probability(input.Misrate), input.Seed)
      : Toolkit.DisparityBounds(input.GetSampleX(), input.GetSampleY(), new Probability(input.Misrate));
    return new DisparityBoundsOutput(bounds);
  }

}
