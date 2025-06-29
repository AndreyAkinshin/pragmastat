namespace Pragmastat.Distributions.Randomization;

/// <summary>
/// Box, George EP. "A note on the generation of random normal deviates." Ann. Math. Stat. 29 (1958): 610-611.
/// </summary>
public static class BoxMullerTransform
{
    public static double Transform(double mean, double sd, Func<double> nextUniform)
    {
        double u = 0, v = 0;
        while (u < 1e-100)
        {
            u = nextUniform();
            v = nextUniform();
        }
        double stdDevFactor = Sqrt(-2.0 * Log(u)) * Sin(2.0 * PI * v);
        return mean + sd * stdDevFactor;
    }
}