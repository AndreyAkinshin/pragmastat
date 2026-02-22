namespace Pragmastat.TestGenerator.Framework.AvgSpreadBounds;

public class AvgSpreadBoundsController(string name, double eps = 1e-9)
  : ReferenceTestController<AvgSpreadBoundsInput, AvgSpreadBoundsOutput>(shared: true)
{
  protected override string SuiteName { get; } = name;

  public override bool Assert(AvgSpreadBoundsOutput expected, AvgSpreadBoundsOutput actual)
  {
    return Math.Abs(expected.Lower - actual.Lower) < eps &&
           Math.Abs(expected.Upper - actual.Upper) < eps;
  }

  public override AvgSpreadBoundsOutput Run(AvgSpreadBoundsInput input)
  {
    var bounds = input.Seed != null
      ? Toolkit.AvgSpreadBounds(input.GetSampleX(), input.GetSampleY(), new Probability(input.Misrate), input.Seed)
      : Toolkit.AvgSpreadBounds(input.GetSampleX(), input.GetSampleY(), new Probability(input.Misrate));
    return new AvgSpreadBoundsOutput(bounds);
  }

}
