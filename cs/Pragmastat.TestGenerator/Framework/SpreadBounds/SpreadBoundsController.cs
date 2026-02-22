namespace Pragmastat.TestGenerator.Framework.SpreadBounds;

public class SpreadBoundsController(string name, double eps = 1e-9)
  : ReferenceTestController<SpreadBoundsInput, SpreadBoundsOutput>(shared: true)
{
  protected override string SuiteName { get; } = name;

  public override bool Assert(SpreadBoundsOutput expected, SpreadBoundsOutput actual)
  {
    return Math.Abs(expected.Lower - actual.Lower) < eps &&
           Math.Abs(expected.Upper - actual.Upper) < eps;
  }

  public override SpreadBoundsOutput Run(SpreadBoundsInput input)
  {
    var bounds = input.Seed != null
      ? Toolkit.SpreadBounds(input.GetSample(), new Probability(input.Misrate), input.Seed)
      : Toolkit.SpreadBounds(input.GetSample(), new Probability(input.Misrate));
    return new SpreadBoundsOutput(bounds);
  }

}
