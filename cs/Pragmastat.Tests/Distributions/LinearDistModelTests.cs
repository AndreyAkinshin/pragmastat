using Pragmastat.Distributions;
using Pragmastat.Exceptions;
using Pragmastat.Metrology;
using Pragmastat.Randomization;

namespace Pragmastat.Tests.Distributions;

/// <summary>
/// Every expected value here is computed by hand against the model's own definition: piecewise
/// linear interpolation of the sorted sample at anchors (i-1)/(n-1), so the checks double as a
/// derivation, not just a pin.
/// </summary>
public class LinearDistModelTests
{
  private static readonly double[] X = [1.0, 2.0, 4.0, 8.0];

  [Fact]
  public void QuantileAtLandsOnEachObservationAtItsAnchor()
  {
    var model = new LinearDistModel(X);
    Assert.Equal(1.0, model.QuantileAt(0.0));
    Assert.Equal(2.0, model.QuantileAt(1.0 / 3.0));
    Assert.Equal(4.0, model.QuantileAt(2.0 / 3.0));
    Assert.Equal(8.0, model.QuantileAt(1.0));
  }

  [Fact]
  public void QuantileAtInterpolatesBetweenTwoObservations()
  {
    var model = new LinearDistModel(X);
    Assert.Equal(3.0, model.QuantileAt(0.5));
  }

  [Fact]
  public void QuantileAtClampsOutsideTheUnitInterval()
  {
    var model = new LinearDistModel(X);
    Assert.Equal(1.0, model.QuantileAt(-0.1));
    Assert.Equal(8.0, model.QuantileAt(1.1));
  }

  [Fact]
  public void CumulativeAtInvertsQuantileAt()
  {
    var model = new LinearDistModel(X);
    Assert.Equal(0.5, model.CumulativeAt(3.0));
    Assert.Equal(1.0 / 3.0, model.CumulativeAt(2.0));
    Assert.Equal(2.0 / 3.0, model.CumulativeAt(4.0));
  }

  [Fact]
  public void CumulativeAtIsZeroBelowAndOneAboveTheSupport()
  {
    var model = new LinearDistModel(X);
    Assert.Equal(0.0, model.CumulativeAt(1.0));
    Assert.Equal(0.0, model.CumulativeAt(-5.0));
    Assert.Equal(1.0, model.CumulativeAt(8.0));
    Assert.Equal(1.0, model.CumulativeAt(20.0));
  }

  [Fact]
  public void DensityAtIsMassOverWidthInsideEachCell()
  {
    var model = new LinearDistModel(X);
    Assert.Equal(1.0 / 3.0, model.DensityAt(1.5));
    Assert.Equal(1.0 / 6.0, model.DensityAt(3.0));
    Assert.Equal(1.0 / 12.0, model.DensityAt(6.0));
    // The right edge belongs to the last cell, not to a zero-width cell past it.
    Assert.Equal(1.0 / 12.0, model.DensityAt(8.0));
  }

  [Fact]
  public void DensityAtIsZeroOutsideTheSupport()
  {
    var model = new LinearDistModel(X);
    Assert.Equal(0.0, model.DensityAt(0.5));
    Assert.Equal(0.0, model.DensityAt(9.0));
  }

  [Theory]
  [InlineData(nameof(LinearDistModel.QuantileAt))]
  [InlineData(nameof(LinearDistModel.CumulativeAt))]
  [InlineData(nameof(LinearDistModel.DensityAt))]
  public void ScalarQueriesPropagateNaN(string method)
  {
    var model = new LinearDistModel(X);
    double result = method switch
    {
      nameof(LinearDistModel.QuantileAt) => model.QuantileAt(double.NaN),
      nameof(LinearDistModel.CumulativeAt) => model.CumulativeAt(double.NaN),
      _ => model.DensityAt(double.NaN),
    };
    Assert.True(double.IsNaN(result));
  }

  [Fact]
  public void GetLandscapeAgreesWithTheScalarQueriesBitForBit()
  {
    var model = new LinearDistModel(X);
    LinearLandscape landscape = model.GetLandscape();

    Assert.Equal(X, landscape.Boundaries);
    Assert.Equal(3, landscape.CellCount);
    for (int i = 0; i < X.Length; i++)
      Assert.Equal(model.CumulativeAt(X[i]), landscape.Cumulative[i]);
    for (int j = 0; j < landscape.CellCount; j++)
    {
      double midpoint = (X[j] + X[j + 1]) / 2.0;
      Assert.Equal(model.DensityAt(midpoint), landscape.Densities[j]);
    }
  }

  [Fact]
  public void GetLandscapeHandsOutACopyRatherThanALiveView()
  {
    var model = new LinearDistModel(X);
    LinearLandscape landscape = model.GetLandscape();
    Assert.Equal(X[0], landscape.Boundaries[0]);
    Assert.Equal(model.SupportMin, model.QuantileAt(0.0));
  }

  [Fact]
  public void SampleIsQuantileAtOfARawDrawnCumulance()
  {
    // QuantileAt(0) is already the model's own support minimum, in the support, so unlike
    // Additive/Multiplic this model never lifts a drawn zero away from it.
    var drawing = new Rng("identity-seed");
    var inverting = new Rng("identity-seed");
    var model = new LinearDistModel(X);
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
    var ex = Assert.Throws<AssumptionException>(() => new LinearDistModel([1.0]));
    Assert.Equal(AssumptionId.Domain, ex.Violation.Id);
  }

  [Fact]
  public void ConstructorRejectsARepeatedValue()
  {
    var ex = Assert.Throws<AssumptionException>(() => new LinearDistModel([1.0, 1.0, 2.0]));
    Assert.Equal(AssumptionId.Distinctness, ex.Violation.Id);
  }

  [Fact]
  public void QuantileRejectsACumulanceOutsideTheUnitInterval()
  {
    var model = new LinearDistModel(X);
    var ex = Assert.Throws<AssumptionException>(() => model.Quantile([1.5]));
    Assert.Equal(AssumptionId.Cumulance, ex.Violation.Id);
  }

  [Fact]
  public void QueryMethodsRejectANonFiniteElement()
  {
    var model = new LinearDistModel(X);
    Assert.Equal(AssumptionId.Validity, Assert.Throws<AssumptionException>(
      () => model.Cumulative([double.PositiveInfinity])).Violation.Id);
  }

  [Fact]
  public void AnEmptyQueryVectorIsNotAViolation()
  {
    var model = new LinearDistModel(X);
    Assert.Empty(model.Quantile([]));
    Assert.Empty(model.Cumulative([]));
    Assert.Empty(model.Density([]));
  }

  [Fact]
  public void SampleConstructorTakesTheSamplesUnit()
  {
    var meter = new MeasurementUnit("meter", "Length", "m", "Meter", 1);
    var sample = new Sample([1.0, 2.0, 4.0, 8.0], meter);
    var model = new LinearDistModel(sample);
    Assert.Equal(meter, model.Unit);
  }

  [Fact]
  public void SampleConstructorRefusesAWeightedSample()
  {
    var sample = new Sample([1.0, 2.0], [1.0, 2.0]);
    Assert.Throws<WeightedSampleNotSupportedException>(() => new LinearDistModel(sample));
  }

  [Fact]
  public void MeasuredQuantileReturnsAMeasurementInTheModelsUnit()
  {
    var meter = new MeasurementUnit("meter", "Length", "m", "Meter", 1);
    var model = new LinearDistModel(new Sample(X, meter));
    Measurement[] result = model.Measured.Quantile([new Probability(0.5)]);
    Assert.Equal(3.0, result[0].Value);
    Assert.Equal(meter, result[0].Unit);
  }

  [Fact]
  public void MeasuredDensityReturnsAPlainNumber()
  {
    var meter = new MeasurementUnit("meter", "Length", "m", "Meter", 1);
    var model = new LinearDistModel(new Sample(X, meter));
    double[] result = model.Measured.Density([new Measurement(3.0, meter)]);
    Assert.Equal(1.0 / 6.0, result[0]);
  }
}
