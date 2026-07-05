namespace Pragmastat.TestGenerator.Framework.RatioBounds;

public class RatioBoundsController : ReferenceTestController<RatioBoundsInput, RatioBoundsOutput>
{
  private readonly double eps;
  private readonly Func<RatioBoundsInput, Bounds> compute;

  protected override string SuiteName { get; }

  public RatioBoundsController(string name, double eps = 1e-9, Func<RatioBoundsInput, Bounds>? compute = null)
    : base(ReferenceTestSuiteHelper.GetTestSuiteDirectory(name, shared: true))
  {
    SuiteName = name;
    this.eps = eps;
    this.compute = compute ?? (input =>
      Toolkit.RatioBounds(input.GetSampleX(), input.GetSampleY(), input.Misrate));
  }

  public override bool Assert(RatioBoundsOutput expected, RatioBoundsOutput actual)
  {
    return Math.Abs(expected.Lower - actual.Lower) < eps &&
           Math.Abs(expected.Upper - actual.Upper) < eps;
  }

  public override RatioBoundsOutput Run(RatioBoundsInput input)
  {
    return new RatioBoundsOutput(compute(input));
  }
}
