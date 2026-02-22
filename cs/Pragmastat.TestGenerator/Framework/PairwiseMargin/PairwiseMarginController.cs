namespace Pragmastat.TestGenerator.Framework.PairwiseMargin;

public class PairwiseMarginController(string name)
  : ReferenceTestController<PairwiseMarginInput, int>(shared: true)
{
  protected override string SuiteName { get; } = name;
  public override bool Assert(int expected, int actual) => expected == actual;

  public override int Run(PairwiseMarginInput input) =>
    Functions.PairwiseMargin.Instance.Calc(input.N, input.M, input.Misrate);

}

