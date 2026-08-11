using Pragmastat.Exceptions;
using Pragmastat.Internal;

namespace Pragmastat.Tests.Internal;

public class AssertionTests
{
  [Fact]
  public void CumulanceAssumptionRejectsNaN()
  {
    var ex = Assert.Throws<AssumptionException>(
      () => Assertion.CumulanceAssumption([double.NaN], Subject.Y));
    Assert.Equal(AssumptionId.Cumulance, ex.Violation.Id);
  }

  [Fact]
  public void CumulanceAssumptionAcceptsTheClosedUnitInterval()
  {
    Assertion.CumulanceAssumption([0.0, 0.5, 1.0], Subject.Y);
  }

  [Fact]
  public void CumulanceAssumptionRejectsOutsideTheUnitInterval()
  {
    var ex = Assert.Throws<AssumptionException>(
      () => Assertion.CumulanceAssumption([1.5], Subject.Y));
    Assert.Equal(AssumptionId.Cumulance, ex.Violation.Id);
  }
}
