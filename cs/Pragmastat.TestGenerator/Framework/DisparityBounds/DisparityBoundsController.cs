namespace Pragmastat.TestGenerator.Framework.DisparityBounds;

public class DisparityBoundsController : ReferenceTestController<DisparityBoundsInput, DisparityBoundsOutput>
{
  private readonly double eps;
  private readonly Func<DisparityBoundsInput, Bounds> compute;

  protected override string SuiteName { get; }

  public DisparityBoundsController(string name, double eps = 1e-9, Func<DisparityBoundsInput, Bounds>? compute = null)
    : base(ReferenceTestSuiteHelper.GetTestSuiteDirectory(name, shared: true))
  {
    SuiteName = name;
    this.eps = eps;
    this.compute = compute ?? (input => input.Seed != null
      ? Toolkit.DisparityBounds(input.GetSampleX(), input.GetSampleY(), input.Misrate, input.Seed)
      : Toolkit.DisparityBounds(input.GetSampleX(), input.GetSampleY(), input.Misrate));
  }

  public override bool Assert(DisparityBoundsOutput expected, DisparityBoundsOutput actual)
  {
    return Math.Abs(expected.Lower - actual.Lower) < eps &&
           Math.Abs(expected.Upper - actual.Upper) < eps;
  }

  public override DisparityBoundsOutput Run(DisparityBoundsInput input)
  {
    return new DisparityBoundsOutput(compute(input));
  }
}
