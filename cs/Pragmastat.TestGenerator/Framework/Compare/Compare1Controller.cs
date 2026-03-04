namespace Pragmastat.TestGenerator.Framework.Compare;

public class Compare1Controller(string name, double eps = 1e-9)
  : ReferenceTestController<Compare1Input, CompareOutput>(shared: true)
{
  protected override string SuiteName { get; } = name;

  public override CompareOutput Run(Compare1Input input)
  {
    var projections = input.Seed != null
      ? Toolkit.Compare1(input.GetSampleX(), input.GetThresholds(), input.Seed)
      : Toolkit.Compare1(input.GetSampleX(), input.GetThresholds());
    return new CompareOutput(projections);
  }

  public override bool Assert(CompareOutput expected, CompareOutput actual)
  {
    if (expected.Projections.Length != actual.Projections.Length) return false;
    for (int i = 0; i < expected.Projections.Length; i++)
    {
      var e = expected.Projections[i];
      var a = actual.Projections[i];
      if (Math.Abs(e.Estimate - a.Estimate) >= eps) return false;
      if (Math.Abs(e.Lower - a.Lower) >= eps) return false;
      if (Math.Abs(e.Upper - a.Upper) >= eps) return false;
      if (e.Verdict != a.Verdict) return false;
    }

    return true;
  }
}
