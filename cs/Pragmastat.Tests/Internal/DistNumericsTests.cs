using Pragmastat.Internal;

namespace Pragmastat.Tests.Internal;

public class DistNumericsTests
{
  [Fact]
  public void NextUpPropagatesNaNAndPositiveInfinity()
  {
    Assert.True(double.IsNaN(DistNumerics.NextUp(double.NaN)));
    Assert.Equal(double.PositiveInfinity, DistNumerics.NextUp(double.PositiveInfinity));
  }

  [Fact]
  public void NextUpLiftsEitherZeroToTheSmallestPositiveSubnormal()
  {
    Assert.Equal(5e-324, DistNumerics.NextUp(0.0));
    Assert.Equal(5e-324, DistNumerics.NextUp(-0.0));
  }

  [Fact]
  public void NextUpStepsTowardPositiveInfinity()
  {
    Assert.True(DistNumerics.NextUp(1.0) > 1.0);
    Assert.True(DistNumerics.NextUp(-1.0) > -1.0);
    Assert.Equal(1.0, DistNumerics.NextDown(DistNumerics.NextUp(1.0)));
  }

  [Fact]
  public void NextDownPropagatesNaNAndNegativeInfinity()
  {
    Assert.True(double.IsNaN(DistNumerics.NextDown(double.NaN)));
    Assert.Equal(double.NegativeInfinity, DistNumerics.NextDown(double.NegativeInfinity));
  }

  [Fact]
  public void NextDownDropsEitherZeroToTheSmallestNegativeSubnormal()
  {
    Assert.Equal(-5e-324, DistNumerics.NextDown(0.0));
    Assert.Equal(-5e-324, DistNumerics.NextDown(-0.0));
  }

  [Fact]
  public void NextDownStepsTowardNegativeInfinity()
  {
    Assert.True(DistNumerics.NextDown(1.0) < 1.0);
    Assert.True(DistNumerics.NextDown(-1.0) < -1.0);
    Assert.Equal(1.0, DistNumerics.NextUp(DistNumerics.NextDown(1.0)));
  }

  [Fact]
  public void MidpointDoesNotOverflowOnTwoLargeEqualValues() =>
    Assert.Equal(double.MaxValue, DistNumerics.Midpoint(double.MaxValue, double.MaxValue));

  [Fact]
  public void MidpointCrossesZeroThroughTheSplitBranch() =>
    Assert.Equal(-1.0, DistNumerics.Midpoint(-5.0, 3.0));

  [Fact]
  public void MidpointOfEqualEndpointsIsThatEndpoint() =>
    Assert.Equal(2.5, DistNumerics.Midpoint(2.5, 2.5));

  [Fact]
  public void LerpReproducesBothEndpointsExactly()
  {
    Assert.Equal(1.0, DistNumerics.Lerp(1.0, 2.0, 0.0));
    Assert.Equal(2.0, DistNumerics.Lerp(1.0, 2.0, 1.0));
  }

  [Fact]
  public void LerpInterpolatesAcrossZero() =>
    Assert.Equal(-1.0, DistNumerics.Lerp(-5.0, 3.0, 0.5));

  [Fact]
  public void LerpInterpolatesWithoutCrossingZero() =>
    Assert.Equal(1.5, DistNumerics.Lerp(1.0, 2.0, 0.5));

  [Fact]
  public void UnitRatioComputesTheOrdinaryCase() => Assert.Equal(0.25, DistNumerics.UnitRatio(1.0, 4.0));

  [Fact]
  public void UnitRatioClampsToTheUnitInterval()
  {
    Assert.Equal(0.0, DistNumerics.UnitRatio(0.0, 1.0));
    Assert.Equal(1.0, DistNumerics.UnitRatio(1.0, 1.0));
  }

  [Fact]
  public void UnitRatioThrowsOnNaN() =>
    Assert.Throws<InvalidOperationException>(() => DistNumerics.UnitRatio(0.0, 0.0));

  [Theory]
  [InlineData(1, 1)]
  [InlineData(2, 2)]
  [InlineData(3, 2)]
  [InlineData(4, 2)]
  [InlineData(5, 3)]
  [InlineData(9, 3)]
  [InlineData(10, 4)]
  [InlineData(16, 4)]
  [InlineData(17, 5)]
  [InlineData(100, 10)]
  [InlineData(101, 11)]
  public void CeilSqrtMatchesTheCeilingOfTheRealSquareRoot(int n, int expected) =>
    Assert.Equal(expected, DistNumerics.CeilSqrt(n));

  [Fact]
  public void LowerBoundFindsTheFirstIndexAtOrAboveTheKey()
  {
    double[] values = [1.0, 2.0, 2.0, 3.0, 5.0];
    Assert.Equal(0, DistNumerics.LowerBound(values, 0, values.Length, 0.0));
    Assert.Equal(1, DistNumerics.LowerBound(values, 0, values.Length, 2.0));
    Assert.Equal(4, DistNumerics.LowerBound(values, 0, values.Length, 4.0));
    Assert.Equal(5, DistNumerics.LowerBound(values, 0, values.Length, 6.0));
  }

  [Fact]
  public void LowerBoundRespectsTheGivenRange()
  {
    double[] values = [1.0, 2.0, 2.0, 3.0, 5.0];
    Assert.Equal(2, DistNumerics.LowerBound(values, 2, 5, 2.0));
  }

  [Fact]
  public void UpperBoundFindsTheFirstIndexStrictlyAboveTheKey()
  {
    double[] values = [1.0, 2.0, 2.0, 3.0, 5.0];
    Assert.Equal(0, DistNumerics.UpperBound(values, 0.0));
    Assert.Equal(3, DistNumerics.UpperBound(values, 2.0));
    Assert.Equal(4, DistNumerics.UpperBound(values, 3.0));
    Assert.Equal(5, DistNumerics.UpperBound(values, 6.0));
  }
}
