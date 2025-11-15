namespace Pragmastat.TestGenerator.Framework.ShiftBounds;

public class ShiftBoundsController(string name, double eps = 1e-9)
  : ReferenceTestController<ShiftBoundsInput, ShiftBoundsOutput>(shared: true)
{
  protected override string SuiteName { get; } = name;

  public override bool Assert(ShiftBoundsOutput expected, ShiftBoundsOutput actual)
  {
    return Math.Abs(expected.Lower - actual.Lower) < eps &&
           Math.Abs(expected.Upper - actual.Upper) < eps;
  }

  public override ShiftBoundsOutput Run(ShiftBoundsInput input)
  {
    var bounds = Toolkit.ShiftBounds(input.GetSampleX(), input.GetSampleY(), new Probability(input.Misrate));
    return new ShiftBoundsOutput(bounds);
  }
}

