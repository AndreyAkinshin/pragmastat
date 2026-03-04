using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.Compare;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class ProjectionOutput
{
  public double Estimate { get; set; }
  public double Lower { get; set; }
  public double Upper { get; set; }
  public string Verdict { get; set; } = "";

  public ProjectionOutput()
  {
  }

  public ProjectionOutput(Projection p)
  {
    Estimate = p.Estimate.NominalValue;
    Lower = p.Bounds.Lower;
    Upper = p.Bounds.Upper;
    Verdict = p.Verdict switch
    {
      ComparisonVerdict.Less => "less",
      ComparisonVerdict.Greater => "greater",
      ComparisonVerdict.Inconclusive => "inconclusive",
      _ => throw new ArgumentOutOfRangeException(nameof(p.Verdict), p.Verdict, null)
    };
  }
}
