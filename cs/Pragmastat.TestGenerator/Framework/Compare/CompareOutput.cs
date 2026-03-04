using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.Compare;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class CompareOutput
{
  public ProjectionOutput[] Projections { get; set; } = [];

  public CompareOutput()
  {
  }

  public CompareOutput(IReadOnlyList<Projection> projections)
  {
    Projections = projections.Select(p => new ProjectionOutput(p)).ToArray();
  }
}
