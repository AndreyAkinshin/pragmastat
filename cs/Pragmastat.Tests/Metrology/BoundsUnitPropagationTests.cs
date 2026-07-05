using Pragmastat.Metrology;

namespace Pragmastat.Tests.Metrology;

/// <summary>
/// The Sample-path bounds re-attach units (ratio/disparity get their dedicated units;
/// center/spread propagate x's unit; shift propagates finer(x, y)), while the RAW (native-array)
/// bounds API stays unitless (Number).
/// </summary>
public class BoundsUnitPropagationTests
{
  private static readonly Probability Misrate = new(0.3);

  // Two compatible (same family) time units with different base sizes so finer() is meaningful.
  private static readonly MeasurementUnit Ms = new("ms", "Time", "ms", "Millisecond", 1);
  private static readonly MeasurementUnit Sec = new("sec", "Time", "s", "Second", 1000);

  private static readonly double[] PosX = { 1, 2, 3, 4, 5, 6, 7, 8 };
  private static readonly double[] PosY = { 2, 4, 6, 8, 10, 12, 14, 16 };

  // --- Sample path: ratio/disparity get their dedicated units ---

  [Fact]
  public void RatioBounds_Sample_HasRatioUnit()
  {
    var x = new Sample(PosX, Ms);
    var y = new Sample(PosY, Ms);
    Assert.Equal(MeasurementUnit.Ratio, Toolkit.RatioBounds(x, y, Misrate).Unit);
  }

  [Fact]
  public void DisparityBounds_Sample_HasDisparityUnit()
  {
    var x = new Sample(PosX, Ms);
    var y = new Sample(PosY, Ms);
    Assert.Equal(MeasurementUnit.Disparity, Toolkit.DisparityBounds(x, y, Misrate).Unit);
  }

  // --- Sample path: center/spread propagate x's unit; shift propagates finer(x, y) ---

  [Fact]
  public void CenterBounds_Sample_PropagatesXUnit()
  {
    var x = new Sample(PosX, Ms);
    Assert.Equal(Ms, Toolkit.CenterBounds(x, Misrate).Unit);
  }

  [Fact]
  public void SpreadBounds_Sample_PropagatesXUnit()
  {
    var x = new Sample(PosX, Ms);
    Assert.Equal(Ms, Toolkit.SpreadBounds(x, Misrate).Unit);
  }

  [Fact]
  public void ShiftBounds_Sample_PropagatesFinerUnit()
  {
    var x = new Sample(PosX, Sec);
    var y = new Sample(PosY, Ms);
    // finer(sec, ms) == ms (smaller base unit)
    Assert.Equal(MeasurementUnit.Finer(Sec, Ms), Toolkit.ShiftBounds(x, y, Misrate).Unit);
  }

  // --- Raw path: unitless (Number) ---

  [Fact]
  public void RatioBounds_Raw_IsUnitless()
  {
    Assert.Equal(MeasurementUnit.Number, Toolkit.RatioBounds(PosX, PosY, (double)Misrate).Unit);
  }

  [Fact]
  public void DisparityBounds_Raw_IsUnitless()
  {
    Assert.Equal(MeasurementUnit.Number, Toolkit.DisparityBounds(PosX, PosY, (double)Misrate).Unit);
  }

  [Fact]
  public void CenterBounds_Raw_IsUnitless()
  {
    Assert.Equal(MeasurementUnit.Number, Toolkit.CenterBounds(PosX, (double)Misrate).Unit);
  }

  [Fact]
  public void SpreadBounds_Raw_IsUnitless()
  {
    Assert.Equal(MeasurementUnit.Number, Toolkit.SpreadBounds(PosX, (double)Misrate).Unit);
  }

  [Fact]
  public void ShiftBounds_Raw_IsUnitless()
  {
    Assert.Equal(MeasurementUnit.Number, Toolkit.ShiftBounds(PosX, PosY, (double)Misrate).Unit);
  }
}
