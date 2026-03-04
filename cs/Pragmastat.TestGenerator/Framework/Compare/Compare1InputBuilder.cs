using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.Compare;

[PublicAPI]
public class Compare1InputBuilder : ReferenceTestCaseInputBuilder<Compare1Input>
{
  private const string DefaultSeed = "compare1-tests";

  public Compare1InputBuilder Add(string name, Sample x, ThresholdInput[] thresholds, string? seed = null)
  {
    Add(name, new Compare1Input(x, thresholds, seed ?? DefaultSeed));
    return this;
  }
}
