using Pragmastat.Core.Internal;

namespace Pragmastat.Core.Functions;

internal static class ErfInverse
{
    /// <summary>
    /// The value of the inverse error function
    ///
    /// <remarks>
    /// Numerical recipes, 3rd ed., page 265
    /// </remarks>
    /// </summary>
    public static double Value(double p)
    {
        Assertion.InRangeExclusive(nameof(p), p, -1, 1);

        p = 1 - p;
        double pp = p < 1.0 ? p : 2 - p;
        double t = Sqrt(-2 * Log(pp / 2));
        double x = -0.70711 * ((2.30753 + t * 0.27061) / (1 + t * (0.99229 + t * 0.04481)) - t);
        for (int i = 0; i < 2; i++)
        {
            double err = 1 - AbramowitzStegunErf.Value(x) - pp;
            x += err / (1.12837916709551257 * Exp(-x.Sqr()) - x * err);
        }
        return p < 1.0 ? x : -x;
    }
}