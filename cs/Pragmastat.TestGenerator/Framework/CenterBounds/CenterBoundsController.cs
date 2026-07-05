namespace Pragmastat.TestGenerator.Framework.CenterBounds;

public class CenterBoundsController : ReferenceTestController<CenterBoundsInput, CenterBoundsOutput>
{
  private readonly double eps;
  private readonly Func<CenterBoundsInput, Bounds> compute;

  protected override string SuiteName { get; }

  public CenterBoundsController(string name, double eps = 1e-9, Func<CenterBoundsInput, Bounds>? compute = null)
    : base(ReferenceTestSuiteHelper.GetTestSuiteDirectory(name, shared: true))
  {
    SuiteName = name;
    this.eps = eps;
    this.compute = compute ?? (input => Toolkit.CenterBounds(input.GetSample(), input.Misrate));
  }

  public override bool Assert(CenterBoundsOutput expected, CenterBoundsOutput actual)
  {
    return Math.Abs(expected.Lower - actual.Lower) < eps &&
           Math.Abs(expected.Upper - actual.Upper) < eps;
  }

  public override CenterBoundsOutput Run(CenterBoundsInput input)
  {
    return new CenterBoundsOutput(compute(input));
  }
}
