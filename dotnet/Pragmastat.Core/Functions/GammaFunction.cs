using Pragmastat.Core.Internal;

namespace Pragmastat.Core.Functions;

internal static class GammaFunction
{
    public static double Value(double x)
    {
        if (x < 1e-5)
            throw new ArgumentOutOfRangeException(nameof(x), "x should be positive");

        // For small x, the Stirling approximation has a noticeable error
        // We resolve this problem using Gamma(x) = Gamma(x+1)/x
        if (x < 1)
            return StirlingApproximation(x + 3) / x / (x + 1) / (x + 2);
        if (x < 2)
            return StirlingApproximation(x + 2) / x / (x + 1);
        if (x < 3)
            return StirlingApproximation(x + 1) / x;

        return StirlingApproximation(x);
    }
        
    public static double LogValue(double x)
    {
        if (x < 1e-5)
            throw new ArgumentOutOfRangeException(nameof(x), "x should be positive");

        // For small x, the Stirling approximation has a noticeable error
        // We resolve this problem using Gamma(x) = Gamma(x+1)/x
        if (x < 1)
            return StirlingApproximationLog(x + 3) - Log(x * (x + 1) * (x + 2));
        if (x < 2)
            return StirlingApproximationLog(x + 2) - Log(x * (x + 1));
        if (x < 3)
            return StirlingApproximationLog(x + 1) - Log(x);

        return StirlingApproximationLog(x);
    }

    private static double StirlingApproximation(double x)
    {
        return Sqrt(2 * PI / x) * (x / E).Pow(x) * Exp(GetSeriesValue(x));
    }
        
    private static double StirlingApproximationLog(double x)
    {
        return x * Log(x) - x + Log(2 * PI / x) / 2 + GetSeriesValue(x);
    }

    // sum = sum(b[2*n] / (2n * (2n-1) * z^(2n-1)))
    private static double GetSeriesValue(double x)
    {
        // Bernoulli numbers
        const double b2 = 1.0 / 6;
        const double b4 = -1.0 / 30;
        const double b6 = 1.0 / 42;
        const double b8 = -1.0 / 30;
        const double b10 = 5.0 / 66;

        return b2 / 2 / x +
               b4 / 12 / (x * x * x) +
               b6 / 30 / (x * x * x * x * x) +
               b8 / 56 / (x * x * x * x * x * x * x) +
               b10 / 90 / (x * x * x * x * x * x * x * x * x);
    }
}