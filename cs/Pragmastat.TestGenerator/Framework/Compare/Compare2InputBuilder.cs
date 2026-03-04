using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.Compare;

[PublicAPI]
public class Compare2InputBuilder : ReferenceTestCaseInputBuilder<Compare2Input>
{
  private const string DefaultSeed = "compare2-tests";

  public Compare2InputBuilder Add(string name, Sample x, Sample y, ThresholdInput[] thresholds, string? seed = null)
  {
    Add(name, new Compare2Input(x, y, thresholds, seed ?? DefaultSeed));
    return this;
  }
}
