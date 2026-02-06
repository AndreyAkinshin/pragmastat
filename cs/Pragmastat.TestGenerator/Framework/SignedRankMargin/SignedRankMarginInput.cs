namespace Pragmastat.TestGenerator.Framework.SignedRankMargin;

public class SignedRankMarginInput(int n, double misrate)
{
  public int N { get; } = n;
  public double Misrate { get; } = misrate;
}
