namespace Pragmastat.TestGenerator.Framework.SpreadBounds;

public class SpreadBoundsController : ReferenceTestController<SpreadBoundsInput, SpreadBoundsOutput>
{
  private readonly double eps;
  private readonly Func<SpreadBoundsInput, Bounds> compute;

  protected override string SuiteName { get; }

  public SpreadBoundsController(string name, double eps = 1e-9, Func<SpreadBoundsInput, Bounds>? compute = null)
    : base(ReferenceTestSuiteHelper.GetTestSuiteDirectory(name, shared: true))
  {
    SuiteName = name;
    this.eps = eps;
    this.compute = compute ?? (input => input.Seed != null
      ? Toolkit.SpreadBounds(input.GetSample(), input.Misrate, input.Seed)
      : Toolkit.SpreadBounds(input.GetSample(), input.Misrate));
  }

  public override bool Assert(SpreadBoundsOutput expected, SpreadBoundsOutput actual)
  {
    return Math.Abs(expected.Lower - actual.Lower) < eps &&
           Math.Abs(expected.Upper - actual.Upper) < eps;
  }

  public override SpreadBoundsOutput Run(SpreadBoundsInput input)
  {
    return new SpreadBoundsOutput(compute(input));
  }
}
