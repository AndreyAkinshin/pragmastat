using Pragmastat.Exceptions;

namespace Pragmastat.Functions;

/// <summary>
/// SignMargin function for one-sample bounds based on Binomial(n, 0.5).
/// </summary>
internal class SignMargin
{
  public static readonly SignMargin Instance = new();

  public int CalcRandomized(int n, double misrate, Pragmastat.Randomization.Rng rng)
  {
    if (n <= 0)
      throw AssumptionException.Domain(Subject.X);
    if (double.IsNaN(misrate) || misrate < 0 || misrate > 1)
      throw AssumptionException.Domain(Subject.Misrate);

    double minMisrate = MinAchievableMisrate.OneSample(n);
    if (misrate < minMisrate)
      throw AssumptionException.Domain(Subject.Misrate);

    double target = misrate / 2;
    if (target <= 0)
      return 0;
    if (target >= 1)
      return checked(n * 2);

    var split = CalcSplit(n, target);
    int rLow = split.RLow;
    double logCdf = split.LogCdf;
    double logPmfHigh = split.LogPmfHigh;

    double logTarget = Math.Log(target);
    double logNum = logTarget > logCdf ? LogSubExp(logTarget, logCdf) : double.NegativeInfinity;
    double p = (IsFinite(logPmfHigh) && IsFinite(logNum))
      ? Math.Exp(logNum - logPmfHigh)
      : 0.0;
    if (p < 0)
      p = 0;
    else if (p > 1)
      p = 1;

    double u = rng.UniformDouble();
    int r = u < p ? rLow + 1 : rLow;
    return checked(r * 2);
  }

  private readonly struct SplitResult
  {
    public readonly int RLow;
    public readonly double LogCdf;
    public readonly double LogPmfHigh;

    public SplitResult(int rLow, double logCdfLow, double logPmfHigh)
    {
      RLow = rLow;
      LogCdf = logCdfLow;
      LogPmfHigh = logPmfHigh;
    }
  }

  private static SplitResult CalcSplit(int n, double target)
  {
    double logTarget = Math.Log(target);

    double logPmf = -n * Math.Log(2);
    double logCdf = logPmf;

    int rLow = 0;

    if (logCdf > logTarget)
      return new SplitResult(0, logCdf, logPmf);

    for (int k = 1; k <= n; k++)
    {
      double logPmfNext = logPmf + Math.Log(n - k + 1) - Math.Log(k);
      double logCdfNext = LogAddExp(logCdf, logPmfNext);

      if (logCdfNext > logTarget)
        return new SplitResult(rLow, logCdf, logPmfNext);

      rLow = k;
      logPmf = logPmfNext;
      logCdf = logCdfNext;
    }

    return new SplitResult(rLow, logCdf, double.NegativeInfinity);
  }

  private static double LogAddExp(double a, double b)
  {
    if (double.IsNegativeInfinity(a))
      return b;
    if (double.IsNegativeInfinity(b))
      return a;
    double m = Math.Max(a, b);
    return m + Math.Log(Math.Exp(a - m) + Math.Exp(b - m));
  }

  private static double LogSubExp(double a, double b)
  {
    if (double.IsNegativeInfinity(b))
      return a;
    double diff = Math.Exp(b - a);
    if (diff >= 1.0)
      return double.NegativeInfinity;
    return a + Math.Log(1.0 - diff);
  }

  private static bool IsFinite(double value)
  {
    return !double.IsNaN(value) && !double.IsInfinity(value);
  }
}
