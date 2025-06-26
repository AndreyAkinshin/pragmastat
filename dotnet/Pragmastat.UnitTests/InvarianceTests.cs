using System.Diagnostics.CodeAnalysis;
using Pragmastat.Core;
using Pragmastat.Core.Metrology;
using Pragmastat.Distributions;
using Pragmastat.Distributions.Helpers;
using static Pragmastat.Toolkit;

namespace Pragmastat.UnitTests;

[SuppressMessage("ReSharper", "ConvertClosureToMethodGroup")]
public class InvarianceTests
{
    private const int Seed = 1729;

    private readonly int[] sampleSizes = [2, 3, 4, 5, 6, 7, 8, 9, 10];

    private void PerformTest(Func<Sample, Measurement> expression1, Func<Sample, Measurement> expression2)
    {
        var generator = UniformDistribution.Standard.Random(Seed);
        foreach (var n in sampleSizes)
        {
            var x = new Sample(generator.Next(n));
            var result1 = expression1(x);
            var result2 = expression2(x);
            Assert.Equal(result1.Unit.FullName, result2.Unit.FullName);
            Assert.Equal(result1.NominalValue, result2.NominalValue, 9);
        }
    }

    private void PerformTest(
        Func<Sample, Sample, Measurement> expression1,
        Func<Sample, Sample, Measurement> expression2)
    {
        var generator = UniformDistribution.Standard.Random(Seed);
        foreach (var n in sampleSizes)
        {
            var x = new Sample(generator.Next(n));
            var y = new Sample(generator.Next(n));
            var result1 = expression1(x, y);
            var result2 = expression2(x, y);
            Assert.Equal(result1.Unit.FullName, result2.Unit.FullName);
            Assert.Equal(result1.NominalValue, result2.NominalValue, 9);
        }
    }

    // Center

    [Fact]
    public void CenterShift() => PerformTest(x => Center(x + 2), x => Center(x) + 2);

    [Fact]
    public void CenterScale() => PerformTest(x => Center(2 * x), x => 2 * Center(x));

    [Fact]
    public void CenterNegate() => PerformTest(x => Center(-1 * x), x => -1 * Center(x));

    // Spread

    [Fact]
    public void SpreadShift() => PerformTest(x => Spread(x + 2), x => Spread(x));

    [Fact]
    public void SpreadScale() => PerformTest(x => Spread(2 * x), x => 2 * Spread(x));

    [Fact]
    public void SpreadNegate() => PerformTest(x => Spread(-1 * x), x => Spread(x));

    // Volatility

    [Fact]
    public void VolatilityScale() => PerformTest(x => Volatility(2 * x), x => Volatility(x));

    // Precision

    [Fact]
    public void PrecisionShift() => PerformTest(x => Precision(x + 2), x => Precision(x));

    [Fact]
    public void PrecisionScale() => PerformTest(x => Precision(2 * x), x => 2 * Precision(x));

    [Fact]
    public void PrecisionScaleNegate() => PerformTest(x => Precision(-2 * x), x => 2 * Precision(x));

    // MedShift

    [Fact]
    public void MedShiftShift() => PerformTest((x, y) => MedShift(x + 3, y + 2), (x, y) => MedShift(x, y) + 1);

    [Fact]
    public void MedShiftScale() => PerformTest((x, y) => MedShift(2 * x, 2 * y), (x, y) => 2 * MedShift(x, y));

    [Fact]
    public void MedShiftAntisymmetry() => PerformTest((x, y) => MedShift(x, y), (x, y) => -1 * MedShift(y, x));

    // MedRatio

    [Fact]
    public void MedRatioScale() => PerformTest((x, y) => MedRatio(2 * x, 3 * y), (x, y) => 2.0 / 3 * MedRatio(x, y));

    // MedSpread

    [Fact]
    public void MedSpreadEqual() => PerformTest(x => MedSpread(x, x), x => Spread(x));

    [Fact]
    public void MedSpreadSymmetry() => PerformTest((x, y) => MedSpread(x, y), (x, y) => MedSpread(y, x));

    [Fact]
    public void MedSpreadAverage() => PerformTest(x => MedSpread(x, 5 * x), x => 3 * Spread(x));

    [Fact]
    public void MedSpreadScale() => PerformTest((x, y) => MedSpread(-2 * x, -2 * y), (x, y) => 2 * MedSpread(x, y));

    // MedDisparity

    [Fact]
    public void MedDisparityShift() => PerformTest((x, y) => MedDisparity(x + 2, y + 2), (x, y) => MedDisparity(x, y));

    [Fact]
    public void MedDisparityScale() => PerformTest((x, y) => MedDisparity(2 * x, 2 * y), (x, y) => MedDisparity(x, y));

    [Fact]
    public void MedDisparityScaleNeg() => PerformTest((x, y) => MedDisparity(-2 * x, -2 * y), (x, y) => -1 * MedDisparity(x, y));

    [Fact]
    public void MedDisparityAntisymmetry() => PerformTest((x, y) => MedDisparity(x, y), (x, y) => -1 * MedDisparity(y, x));
}