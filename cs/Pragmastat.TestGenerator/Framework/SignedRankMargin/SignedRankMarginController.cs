namespace Pragmastat.TestGenerator.Framework.SignedRankMargin;

public class SignedRankMarginController(string name)
  : ReferenceTestController<SignedRankMarginInput, int>(shared: true)
{
  protected override string SuiteName { get; } = name;
  public override bool Assert(int expected, int actual) => expected == actual;

  public override int Run(SignedRankMarginInput input) =>
    Functions.SignedRankMargin.Instance.Calc(input.N, input.Misrate);

}
