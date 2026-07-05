using Pragmastat.Exceptions;

namespace Pragmastat.Tests.Estimators;

/// <summary>
/// The raw (native-array, double misrate) bounds API must reject an out-of-[0,1] or
/// NaN misrate with a domain/misrate AssumptionError. Covers a one-sample (CenterBounds) and a
/// two-sample (ShiftBounds) entry. This is the double path, NOT the typed Probability path
/// (which rejects such values at construction time).
/// </summary>
public class BoundsMisrateDomainTests
{
  private static readonly double[] X = { 1, 2, 3, 4, 5, 6, 7, 8 };
  private static readonly double[] Y = { 2, 4, 6, 8, 10, 12, 14, 16 };

  private static void AssertMisrateDomain(Action act)
  {
    var ex = Assert.Throws<AssumptionException>(act);
    Assert.Equal(AssumptionId.Domain, ex.Violation.Id);
    Assert.Equal(Subject.Misrate, ex.Violation.Subject);
  }

  [Theory]
  [InlineData(2.0)]
  [InlineData(-0.1)]
  [InlineData(double.NaN)]
  public void CenterBounds_Raw_RejectsOutOfDomainMisrate(double misrate)
  {
    AssertMisrateDomain(() => Toolkit.CenterBounds(X, misrate));
  }

  [Theory]
  [InlineData(2.0)]
  [InlineData(-0.1)]
  [InlineData(double.NaN)]
  public void ShiftBounds_Raw_RejectsOutOfDomainMisrate(double misrate)
  {
    AssertMisrateDomain(() => Toolkit.ShiftBounds(X, Y, misrate));
  }
}
