namespace Pragmastat.TestGenerator.Framework.ShiftBounds;

public class ShiftBoundsController : ReferenceTestController<ShiftBoundsInput, ShiftBoundsOutput>
{
  private readonly double eps;
  private readonly Func<ShiftBoundsInput, Bounds> compute;

  protected override string SuiteName { get; }

  public ShiftBoundsController(string name, double eps = 1e-9, Func<ShiftBoundsInput, Bounds>? compute = null)
    : base(ReferenceTestSuiteHelper.GetTestSuiteDirectory(name, shared: true))
  {
    SuiteName = name;
    this.eps = eps;
    this.compute = compute ?? (input =>
      Toolkit.ShiftBounds(input.GetSampleX(), input.GetSampleY(), input.Misrate));
  }

  public override bool Assert(ShiftBoundsOutput expected, ShiftBoundsOutput actual)
  {
    return Math.Abs(expected.Lower - actual.Lower) < eps &&
           Math.Abs(expected.Upper - actual.Upper) < eps;
  }

  public override ShiftBoundsOutput Run(ShiftBoundsInput input)
  {
    return new ShiftBoundsOutput(compute(input));
  }
}
