using Pragmastat.Distributions;
using Pragmastat.Exceptions;
using Pragmastat.Metrology;
using Pragmastat.Randomization;

namespace Pragmastat.Tests.Distributions;

public class NeighborDistModelTests
{
  private static readonly double[] X = [0.0, 1.0, 2.0, 4.0, 8.0];

  [Fact]
  public void SupportMatchesTheMinAndMaxObservation()
  {
    var model = new NeighborDistModel(X);
    Assert.Equal(0.0, model.SupportMin);
    Assert.Equal(8.0, model.SupportMax);
  }

  [Fact]
  public void CellCountIsAtMostOneFewerThanTheObservationCount()
  {
    var model = new NeighborDistModel(X);
    // No adjacent midpoint collapses on this sample, so every observation survives its own cell.
    Assert.Equal(X.Length, model.CellCount);
  }

  [Fact]
  public void CumulativeAtTheSupportIsZeroAndOneExactly()
  {
    var model = new NeighborDistModel(X);
    Assert.Equal(0.0, model.CumulativeAt(model.SupportMin));
    Assert.Equal(1.0, model.CumulativeAt(model.SupportMax));
  }

  [Fact]
  public void CellMassesSumToExactlyOne()
  {
    // f[0] = 0.0 and f[CellCount] = 1.0 are fixed by construction, so the masses telescope to
    // exactly 1.0 regardless of how the intermediate cumulative values rounded.
    var model = new NeighborDistModel(X);
    NeighborLandscape landscape = model.GetLandscape();
    double sum = 0.0;
    foreach (double mass in landscape.Masses)
      sum += mass;
    Assert.Equal(1.0, sum);
  }

  [Fact]
  public void CumulativeAtIsNonDecreasingAcrossTheBoundaries()
  {
    var model = new NeighborDistModel(X);
    NeighborLandscape landscape = model.GetLandscape();
    for (int i = 1; i < landscape.Cumulative.Count; i++)
      Assert.True(landscape.Cumulative[i] > landscape.Cumulative[i - 1]);
  }

  [Fact]
  public void QuantileAtInvertsCumulativeAtAtEachBoundary()
  {
    var model = new NeighborDistModel(X);
    NeighborLandscape landscape = model.GetLandscape();
    for (int i = 0; i < landscape.Boundaries.Count; i++)
    {
      double c = landscape.Cumulative[i];
      Assert.Equal(landscape.Boundaries[i], model.QuantileAt(c));
    }
  }

  [Fact]
  public void QuantileAtClampsOutsideTheUnitInterval()
  {
    var model = new NeighborDistModel(X);
    Assert.Equal(model.SupportMin, model.QuantileAt(-0.1));
    Assert.Equal(model.SupportMax, model.QuantileAt(1.1));
  }

  [Fact]
  public void DensityAtIsZeroOutsideTheSupport()
  {
    var model = new NeighborDistModel(X);
    Assert.Equal(0.0, model.DensityAt(model.SupportMin - 1.0));
    Assert.Equal(0.0, model.DensityAt(model.SupportMax + 1.0));
  }

  [Theory]
  [InlineData(nameof(NeighborDistModel.QuantileAt))]
  [InlineData(nameof(NeighborDistModel.CumulativeAt))]
  [InlineData(nameof(NeighborDistModel.DensityAt))]
  public void ScalarQueriesPropagateNaN(string method)
  {
    var model = new NeighborDistModel(X);
    double result = method switch
    {
      nameof(NeighborDistModel.QuantileAt) => model.QuantileAt(double.NaN),
      nameof(NeighborDistModel.CumulativeAt) => model.CumulativeAt(double.NaN),
      _ => model.DensityAt(double.NaN),
    };
    Assert.True(double.IsNaN(result));
  }

  [Fact]
  public void GetLandscapeAgreesWithTheScalarQueriesBitForBit()
  {
    var model = new NeighborDistModel(X);
    NeighborLandscape landscape = model.GetLandscape();

    for (int i = 0; i < landscape.Boundaries.Count; i++)
      Assert.Equal(model.CumulativeAt(landscape.Boundaries[i]), landscape.Cumulative[i]);

    for (int j = 0; j < landscape.CellCount; j++)
    {
      double midpoint = (landscape.Boundaries[j] + landscape.Boundaries[j + 1]) / 2.0;
      Assert.Equal(model.DensityAt(midpoint), landscape.Densities[j]);
    }
  }

  [Fact]
  public void GetLandscapeExposesScaleIndexAndRadius()
  {
    var model = new NeighborDistModel(X);
    NeighborLandscape landscape = model.GetLandscape();
    // n = 5: CeilSqrt(5) = 3 (smallest k with k >= ceil(5/k)), radius = 3 / 2 = 1.
    Assert.Equal(3, landscape.ScaleIndex);
    Assert.Equal(1, landscape.Radius);
  }

  [Fact]
  public void GetLandscapeSourcesMapEachCellToItsObservation()
  {
    var model = new NeighborDistModel(X);
    NeighborLandscape landscape = model.GetLandscape();
    Assert.Equal([0, 1, 2, 3, 4], landscape.Sources);
  }

  [Fact]
  public void AZeroWidthCellFromACollapsedMidpointIsDropped()
  {
    // The center observation sits exactly one ULP from each neighbor on either side. Both
    // adjacent midpoints round to 1.0 itself under round-to-nearest-even (each is exactly
    // halfway between two representable doubles, and 1.0 has the even mantissa), so the center
    // cell's two boundaries collapse onto the same point and the cell is dropped.
    double center = 1.0;
    double below = BitConverter.Int64BitsToDouble(BitConverter.DoubleToInt64Bits(center) - 1);
    double above = BitConverter.Int64BitsToDouble(BitConverter.DoubleToInt64Bits(center) + 1);
    double[] values = [0.0, below, center, above, 10.0];

    var model = new NeighborDistModel(values);
    NeighborLandscape landscape = model.GetLandscape();

    Assert.Equal(values.Length - 1, landscape.CellCount);
    Assert.DoesNotContain(2, landscape.Sources);
  }

  [Fact]
  public void SampleIsQuantileAtOfARawDrawnCumulance()
  {
    var drawing = new Rng("identity-seed");
    var inverting = new Rng("identity-seed");
    var model = new NeighborDistModel(X);
    for (int i = 0; i < 100; i++)
    {
      double drawn = model.Sample(drawing);
      double u = inverting.UniformDouble();
      Assert.Equal(BitConverter.DoubleToInt64Bits(model.QuantileAt(u)), BitConverter.DoubleToInt64Bits(drawn));
    }
  }

  [Fact]
  public void ConstructorRejectsFewerThanTwoObservations()
  {
    var ex = Assert.Throws<AssumptionException>(() => new NeighborDistModel([1.0]));
    Assert.Equal(AssumptionId.Domain, ex.Violation.Id);
  }

  [Fact]
  public void ConstructorRejectsARepeatedValue()
  {
    var ex = Assert.Throws<AssumptionException>(() => new NeighborDistModel([1.0, 1.0, 2.0]));
    Assert.Equal(AssumptionId.Distinctness, ex.Violation.Id);
  }

  [Fact]
  public void QuantileRejectsACumulanceOutsideTheUnitInterval()
  {
    var model = new NeighborDistModel(X);
    var ex = Assert.Throws<AssumptionException>(() => model.Quantile([-0.1]));
    Assert.Equal(AssumptionId.Cumulance, ex.Violation.Id);
  }

  [Fact]
  public void AnEmptyQueryVectorIsNotAViolation()
  {
    var model = new NeighborDistModel(X);
    Assert.Empty(model.Quantile([]));
    Assert.Empty(model.Cumulative([]));
    Assert.Empty(model.Density([]));
  }

  [Fact]
  public void ABrokenAssumeSortedPromiseIsANumericalConformanceFailure()
  {
    // DistSampleGuard's distinctness check only looks at ADJACENT elements of the (unsorted,
    // since assumeSorted is a promise it trusts) array, so a non-adjacent repeat -- x[2] and x[6]
    // both 5.0 here -- slips through it. That repeat then makes one window's span exactly zero
    // while its cell keeps a nonzero width, so the cell's unnormalized mass divides by zero and
    // the total the whole reconstruction divides by is not a usable number.
    double[] scrambled = [0.0, 2.0, 5.0, 8.0, 10.0, 12.0, 5.0, 20.0, 25.0, 30.0];
    Assert.Throws<InvalidOperationException>(() => new NeighborDistModel(scrambled, assumeSorted: true));
  }

  [Fact]
  public void SampleConstructorTakesTheSamplesUnit()
  {
    var meter = new MeasurementUnit("meter", "Length", "m", "Meter", 1);
    var sample = new Sample(X, meter);
    var model = new NeighborDistModel(sample);
    Assert.Equal(meter, model.Unit);
  }

  [Fact]
  public void MeasuredQuantileReturnsAMeasurementInTheModelsUnit()
  {
    var meter = new MeasurementUnit("meter", "Length", "m", "Meter", 1);
    var model = new NeighborDistModel(new Sample(X, meter));
    Measurement[] result = model.Measured.Quantile([new Probability(0.5)]);
    Assert.Equal(model.QuantileAt(0.5), result[0].Value);
    Assert.Equal(meter, result[0].Unit);
  }

  [Fact]
  public void MeasuredDensityReturnsAPlainNumber()
  {
    var meter = new MeasurementUnit("meter", "Length", "m", "Meter", 1);
    var model = new NeighborDistModel(new Sample(X, meter));
    double[] result = model.Measured.Density([new Measurement(3.0, meter)]);
    Assert.Equal(model.DensityAt(3.0), result[0]);
  }
}
