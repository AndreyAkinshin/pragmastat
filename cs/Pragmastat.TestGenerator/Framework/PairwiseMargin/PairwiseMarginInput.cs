namespace Pragmastat.TestGenerator.Framework.PairwiseMargin;

public class PairwiseMarginInput(int n, int m, double misrate)
{
  public int N { get; } = n;
  public int M { get; } = m;
  public double Misrate { get; } = misrate;
}
