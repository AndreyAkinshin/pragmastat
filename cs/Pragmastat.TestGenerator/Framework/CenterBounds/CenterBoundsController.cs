namespace Pragmastat.TestGenerator.Framework.CenterBounds;

public class CenterBoundsController(string name, double eps = 1e-9)
  : ReferenceTestController<CenterBoundsInput, CenterBoundsOutput>(shared: true)
{
  protected override string SuiteName { get; } = name;

  public override bool Assert(CenterBoundsOutput expected, CenterBoundsOutput actual)
  {
    return Math.Abs(expected.Lower - actual.Lower) < eps &&
           Math.Abs(expected.Upper - actual.Upper) < eps;
  }

  public override CenterBoundsOutput Run(CenterBoundsInput input)
  {
    var bounds = Toolkit.CenterBounds(input.GetSample(), new Probability(input.Misrate));
    return new CenterBoundsOutput(bounds);
  }

}
