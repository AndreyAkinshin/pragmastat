using Pragmastat.Internal;

namespace Pragmastat.Functions;

internal static class PairwiseSampleTransformer
{
    public static Sample Transform(
        Sample x,
        Func<double, double, double> func,
        bool includeDiagonal)
    {
        int n = x.Size;
        int skip = includeDiagonal ? 0 : 1;
        int size = n * (n + 1) / 2 - n * skip;
        if (x.IsWeighted)
        {
            double[] values = new double[size];
            double[] weights = new double[size];
            int k = 0;
            for (int i = 0; i < n; i++)
            for (int j = i + skip; j < n; j++)
            {
                values[k] = func(x.Values[i], x.Values[j]);
                weights[k++] = x.Weights[i] * x.Weights[j];
            }
            return new Sample(values, weights, x.Unit);
        }
        else
        {
            double[] values = new double[size];
            int k = 0;
            for (int i = 0; i < n; i++)
            for (int j = i + skip; j < n; j++)
                values[k++] = func(x.Values[i], x.Values[j]);
            return new Sample(values, x.Unit);
        }
    }

    public static Sample Transform(
        Sample x,
        Sample y,
        Func<double, double, double> func)
    {
        Assertion.MatchedUnit(x, y);

        int n = x.Size;
        int m = y.Size;
        int size = n * m;
        if (x.IsWeighted)
        {
            double[] values = new double[size];
            double[] weights = new double[size];
            int k = 0;
            for (int i = 0; i < n; i++)
            for (int j = 0; j < m; j++)
            {
                values[k] = func(x.Values[i], y.Values[j]);
                weights[k++] = x.Weights[i] * y.Weights[j];
            }
            return new Sample(values, weights, x.Unit);
        }
        else
        {
            double[] values = new double[size];
            int k = 0;
            for (int i = 0; i < n; i++)
            for (int j = 0; j < m; j++)
                values[k++] = func(x.Values[i], y.Values[j]);
            return new Sample(values, x.Unit);
        }
    }
}