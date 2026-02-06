using JetBrains.Annotations;
using Pragmastat.Exceptions;

namespace Pragmastat.Functions;

/// <summary>
/// SignedRankMargin function for one-sample bounds.
/// One-sample analog of PairwiseMargin using Wilcoxon signed-rank distribution.
/// </summary>
/// <param name="threshold">Maximum n for exact computation; larger n uses approximation</param>
public class SignedRankMargin(int threshold = SignedRankMargin.MaxExactSize)
{
  public static readonly SignedRankMargin Instance = new();

  private const int MaxExactSize = 63;

  [PublicAPI]
  public int Calc(int n, double misrate)
  {
    if (n <= 0)
      throw AssumptionException.Domain(Subject.X);
    if (double.IsNaN(misrate) || misrate < 0 || misrate > 1)
      throw AssumptionException.Domain(Subject.Misrate);

    double minMisrate = MinAchievableMisrate.OneSample(n);
    if (misrate < minMisrate)
      throw AssumptionException.Domain(Subject.Misrate);

    return n <= threshold
      ? CalcExact(n, misrate)
      : CalcApprox(n, misrate);
  }

  internal int CalcExact(int n, double misrate)
  {
    int raw = CalcExactRaw(n, misrate / 2);
    return checked(raw * 2);
  }

  internal int CalcApprox(int n, double misrate)
  {
    long raw = CalcApproxRaw(n, misrate / 2);
    long margin = raw * 2;
    if (margin > int.MaxValue)
      throw new OverflowException($"Signed-rank margin exceeds supported range for n={n}");
    return (int)margin;
  }

  /// <summary>
  /// Compute one-sided margin using exact Wilcoxon signed-rank distribution.
  /// Uses dynamic programming to compute the CDF.
  /// </summary>
  private static int CalcExactRaw(int n, double p)
  {
    ulong total = 1UL << n;
    long maxW = (long)n * (n + 1) / 2;

    var count = new ulong[maxW + 1];
    count[0] = 1;

    for (int i = 1; i <= n; i++)
    {
      for (long w = Min(maxW, (long)i * (i + 1) / 2); w >= i; w--)
        count[w] += count[w - i];
    }

    ulong cumulative = 0;
    for (int w = 0; w <= maxW; w++)
    {
      cumulative += count[w];
      double cdf = (double)cumulative / total;
      if (cdf >= p)
        return w;
    }

    return (int)maxW;
  }

  /// <summary>
  /// Compute one-sided margin using Edgeworth approximation for large n.
  /// </summary>
  private static long CalcApproxRaw(int n, double misrate)
  {
    long maxW = (long)n * (n + 1) / 2;
    long a = 0;
    long b = maxW;

    while (a < b - 1)
    {
      long c = (a + b) / 2;
      double cdf = EdgeworthCdf(n, c);
      if (cdf < misrate)
        a = c;
      else
        b = c;
    }

    return EdgeworthCdf(n, b) < misrate ? b : a;
  }

  /// <summary>
  /// Edgeworth expansion for Wilcoxon signed-rank distribution CDF.
  /// </summary>
  private static double EdgeworthCdf(int n, long w)
  {
    double mu = (double)n * (n + 1) / 4.0;
    double sigma2 = n * (n + 1.0) * (2 * n + 1) / 24.0;
    double sigma = Sqrt(sigma2);

    // +0.5 continuity correction: computing P(W ≤ w) for a left-tail discrete CDF
    double z = (w - mu + 0.5) / sigma;
    double phi = Exp(-z * z / 2) / Sqrt(2 * PI);
    double Phi = AcmAlgorithm209.Gauss(z);

    double mu4 = CentralMoment4(n);
    double kappa4 = mu4 - 3 * sigma2 * sigma2;

    double e3 = kappa4 / (24 * sigma2 * sigma2);

    double z2 = z * z;
    double z3 = z2 * z;
    double f3 = -phi * (z3 - 3 * z);

    double edgeworth = Phi + e3 * f3;
    return Min(Max(edgeworth, 0), 1);
  }

  /// <summary>
  /// Compute 4th central moment of signed-rank distribution.
  /// E[(W - mu)^4] where W is the Wilcoxon signed-rank statistic.
  /// </summary>
  private static double CentralMoment4(int n)
  {
    double nd = n;
    double n2 = nd * nd;
    double n3 = n2 * nd;
    double n4 = n2 * n2;
    double n5 = n4 * nd;

    return (9 * n5 + 45 * n4 + 65 * n3 + 15 * n2 - 14 * nd) / 480.0;
  }
}
