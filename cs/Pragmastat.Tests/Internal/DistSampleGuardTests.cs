using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.Tests.Internal;

public class DistSampleGuardTests
{
  [Fact]
  public void PrepareSortsByDefault()
  {
    double[] copy = DistSampleGuard.Prepare([3.0, 1.0, 2.0], assumeSorted: false);
    Assert.Equal([1.0, 2.0, 3.0], copy);
  }

  [Fact]
  public void PrepareTrustsAnAlreadySortedArrayWhenAsked()
  {
    double[] copy = DistSampleGuard.Prepare([1.0, 2.0, 3.0], assumeSorted: true);
    Assert.Equal([1.0, 2.0, 3.0], copy);
  }

  [Fact]
  public void PrepareNeverRetainsTheCallersArray()
  {
    double[] input = [1.0, 2.0];
    double[] copy = DistSampleGuard.Prepare(input, assumeSorted: false);
    copy[0] = 99.0;
    Assert.Equal(1.0, input[0]);
  }

  [Fact]
  public void PrepareRejectsFewerThanTwoObservations()
  {
    var ex = Assert.Throws<AssumptionException>(() => DistSampleGuard.Prepare([1.0], assumeSorted: false));
    Assert.Equal(AssumptionId.Domain, ex.Violation.Id);
    Assert.Equal(Subject.X, ex.Violation.Subject);
  }

  [Fact]
  public void PrepareRejectsARangeThatOverflowsToInfinity()
  {
    var ex = Assert.Throws<AssumptionException>(
      () => DistSampleGuard.Prepare([-double.MaxValue, double.MaxValue], assumeSorted: false));
    Assert.Equal(AssumptionId.Domain, ex.Violation.Id);
  }

  [Fact]
  public void PrepareRejectsARepeatedValue()
  {
    var ex = Assert.Throws<AssumptionException>(() => DistSampleGuard.Prepare([1.0, 1.0], assumeSorted: false));
    Assert.Equal(AssumptionId.Distinctness, ex.Violation.Id);
    Assert.Equal(Subject.X, ex.Violation.Subject);
  }

  [Fact]
  public void PrepareRejectsAMixedSignZeroPairAsRepeated()
  {
    Assert.Throws<AssumptionException>(() => DistSampleGuard.Prepare([0.0, -0.0], assumeSorted: false));
    Assert.Throws<AssumptionException>(() => DistSampleGuard.Prepare([-0.0, 0.0], assumeSorted: false));
  }

  [Fact]
  public void ToMeasurementsAttachesTheUnitToEveryElement()
  {
    Measurement[] result = DistSampleGuard.ToMeasurements([1.0, 2.0], MeasurementUnit.Number);
    Assert.Equal(2, result.Length);
    Assert.Equal(1.0, result[0].Value);
    Assert.Equal(MeasurementUnit.Number, result[0].Unit);
    Assert.Equal(2.0, result[1].Value);
  }

  [Fact]
  public void ToRawConvertsACompatibleButDifferentUnit()
  {
    var meter = new MeasurementUnit("meter", "Length", "m", "Meter", 1);
    var kilometer = new MeasurementUnit("kilometer", "Length", "km", "Kilometer", 1000);
    Measurement[] values = [new Measurement(2.0, kilometer)];

    double[] raw = DistSampleGuard.ToRaw(values, meter);

    Assert.Equal(2000.0, raw[0]);
  }

  [Fact]
  public void ToRawRejectsAnIncompatibleUnit()
  {
    var meter = new MeasurementUnit("meter", "Length", "m", "Meter", 1);
    Measurement[] values = [new Measurement(1.0, MeasurementUnit.Number)];

    Assert.Throws<UnitMismatchException>(() => DistSampleGuard.ToRaw(values, meter));
  }
}
