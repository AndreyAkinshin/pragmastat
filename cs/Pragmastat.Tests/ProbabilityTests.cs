namespace Pragmastat.Tests;

/// <summary>
/// Probability must reject values outside [0, 1] AND NaN. The range check
/// (value &lt; min || value &gt; max) is false for NaN, so a separate guard rejects NaN to keep
/// C# in agreement with Kotlin.
/// </summary>
public class ProbabilityTests
{
  [Theory]
  [InlineData(-0.1)]
  [InlineData(1.1)]
  [InlineData(double.NaN)]
  public void Probability_RejectsOutOfRangeAndNaN(double value)
  {
    Assert.Throws<ArgumentOutOfRangeException>(() => new Probability(value));
  }

  [Theory]
  [InlineData(0.0)]
  [InlineData(0.5)]
  [InlineData(1.0)]
  public void Probability_AcceptsInRange(double value)
  {
    var p = new Probability(value);
    Assert.Equal(value, p.Value);
  }
}
