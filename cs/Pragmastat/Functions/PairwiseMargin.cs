using System;
using JetBrains.Annotations;
using Pragmastat.Exceptions;
using Pragmastat.Internal;

namespace Pragmastat.Functions;

/// <summary>
/// PairwiseMargin function
/// </summary>
/// <param name="threshold">The maximum value for n+m, after which implementation switches from exact to approx</param>
public class PairwiseMargin(int threshold = PairwiseMargin.MaxExactSize)
{
  public static readonly PairwiseMargin Instance = new();

  private const int MaxExactSize = 400;

  [PublicAPI]
  public int Calc(int n, int m, double misrate)
  {
    if (n <= 0)
      throw AssumptionException.Domain(Subject.X);
    if (m <= 0)
      throw AssumptionException.Domain(Subject.Y);
    if (double.IsNaN(misrate) || misrate < 0 || misrate > 1)
      throw AssumptionException.Domain(Subject.Misrate);

    double minMisrate = MinAchievableMisrate.TwoSample(n, m);
    if (misrate < minMisrate)
      throw AssumptionException.Domain(Subject.Misrate);

    return n + m <= threshold
      ? CalcExact(n, m, misrate)
      : CalcApprox(n, m, misrate);
  }

  internal int CalcExact(int n, int m, double misrate)
  {
    int raw = CalcExactRaw(n, m, misrate / 2);
    return checked(raw * 2);
  }

  internal int CalcApprox(int n, int m, double misrate)
  {
    long raw = CalcApproxRaw(n, m, misrate / 2);
    long margin = raw * 2;
    if (margin > int.MaxValue)
      throw new OverflowException($"Pairwise margin exceeds supported range for n={n}, m={m}");
    return (int)margin;
  }

  // Inversed implementation of Andreas Löffler's (1982) "Über eine Partition der nat. Zahlen und ihre Anwendung beim U-Test"
  private static int CalcExactRaw(int n, int m, double p)
  {
    double total = n + m < BinomialCoefficientFunction.MaxAcceptableN
      ? BinomialCoefficientFunction.BinomialCoefficient(n + m, m)
      : BinomialCoefficientFunction.BinomialCoefficient(n + m, m * 1.0);

    var pmf = new List<double> { 1 }; // pmf[0] = 1
    var sigma = new List<double> { 0 }; // sigma[0] is unused

    int u = 0;
    double cdf = 1.0 / total;

    if (cdf >= p)
      return 0;

    while (true)
    {
      u++;
      // Ensure sigma has entry for u
      if (sigma.Count <= u)
      {
        int value = 0;
        for (int d = 1; d <= n; d++)
          if (u % d == 0 && u >= d)
            value += d;
        for (int d = m + 1; d <= m + n; d++)
          if (u % d == 0 && u >= d)
            value -= d;
        sigma.Add(value);
      }

      // Compute pmf[u] using Loeffler recurrence
      double sum = 0;
      for (int i = 0; i < u; i++)
        sum += pmf[i] * sigma[u - i];
      sum /= u;
      pmf.Add(sum);

      cdf += sum / total;
      if (cdf >= p)
        return u;
      if (sum == 0)
        break;
    }

    return pmf.Count - 1;
  }

  // Inverse Edgeworth Approximation
  private static long CalcApproxRaw(int n, int m, double misrate)
  {
    long a = 0;
    long b = (long)n * m;
    while (a < b - 1)
    {
      long c = (a + b) / 2;
      double p = EdgeworthCdf(n, m, c);
      if (p < misrate)
        a = c;
      else
        b = c;
    }

    return EdgeworthCdf(n, m, b) < misrate ? b : a;
  }

  private static double EdgeworthCdf(int n, int m, long u)
  {
    double nm = (double)n * m;
    double mu = nm / 2.0;
    double su = Sqrt(nm * (n + m + 1) / 12.0);
    // -0.5 continuity correction: computing P(U ≥ u) for a right-tail discrete CDF
    double z = (u - mu - 0.5) / su;
    double phi = Exp(-z.Sqr() / 2) / Sqrt(2 * PI);
    double Phi = AcmAlgorithm209.Gauss(z);

    // Pre-compute powers of n and m for efficiency
    double n2 = (double)n * n;
    double n3 = n2 * n;
    double n4 = n2 * n2;
    double m2 = (double)m * m;
    double m3 = m2 * m;
    double m4 = m2 * m2;

    double mu2 = (double)n * m * (n + m + 1) / 12.0;
    double mu4 =
      (double)n * m * (n + m + 1) *
      (0
       + 5 * m * n * (m + n)
       - 2 * (m2 + n2)
       + 3 * m * n
       - 2 * (n + m)
      ) / 240.0;

    double mu6 =
      (double)n * m * (n + m + 1) *
      (0
       + 35 * m2 * n2 * (m2 + n2)
       + 70 * m3 * n3
       - 42 * m * n * (m3 + n3)
       - 14 * m2 * n2 * (n + m)
       + 16 * (n4 + m4)
       - 52 * n * m * (n2 + m2)
       - 43 * n2 * m2
       + 32 * (m3 + n3)
       + 14 * m * n * (n + m)
       + 8 * (n2 + m2)
       + 16 * n * m
       - 8 * (n + m)
      ) / 4032.0;

    // Pre-compute powers of mu2 and related terms
    double mu2_2 = mu2 * mu2;
    double mu2_3 = mu2_2 * mu2;
    double mu4_mu2_2 = mu4 / mu2_2;

    // Factorial constants: 4! = 24, 6! = 720, 8! = 40320
    double e3 = (mu4_mu2_2 - 3) / 24.0;
    double e5 = (mu6 / mu2_3 - 15 * mu4_mu2_2 + 30) / 720.0;
    double e7 = 35 * (mu4_mu2_2 - 3) * (mu4_mu2_2 - 3) / 40320.0;

    // Pre-compute powers of z for Hermite polynomials
    double z2 = z * z;
    double z3 = z2 * z;
    double z5 = z3 * z2;
    double z7 = z5 * z2;

    double f3 = -phi * (z3 - 3 * z);
    double f5 = -phi * (z5 - 10 * z3 + 15 * z);
    double f7 = -phi * (z7 - 21 * z5 + 105 * z3 - 105 * z);

    double edgeworth = Phi + e3 * f3 + e5 * f5 + e7 * f7;
    return Min(Max(edgeworth, 0), 1);
  }
}
