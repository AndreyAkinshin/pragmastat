using Pragmastat.Exceptions;

namespace Pragmastat.Tests.Estimators;

/// <summary>
/// RatioBounds must report assumption violations in the documented priority order:
/// domain(misrate) is checked before positivity(x); with a valid misrate, the positivity
/// violation surfaces. Uses the raw (double misrate) API because the typed Probability path
/// rejects an invalid misrate at construction time.
/// </summary>
public class RatioBoundsErrorPriorityTests
{
  [Fact]
  public void RatioBounds_DomainBeforePositivity()
  {
    // misrate=-0.1 is invalid (domain), x=-1 is non-positive (positivity);
    // domain(misrate) must take priority over positivity(x).
    var x = new double[] { -1.0 };
    var y = new double[] { 1.0 };
    var ex = Assert.Throws<AssumptionException>(() => Toolkit.RatioBounds(x, y, -0.1));
    Assert.Equal(AssumptionId.Domain, ex.Violation.Id);
    Assert.Equal(Subject.Misrate, ex.Violation.Subject);
  }

  [Fact]
  public void RatioBounds_PositivityWhenMisrateValid()
  {
    // Valid misrate but non-positive x -> positivity(x).
    var x = new double[] { -1.0, -2.0, -3.0 };
    var y = new double[] { 1.0, 2.0, 3.0 };
    var ex = Assert.Throws<AssumptionException>(() => Toolkit.RatioBounds(x, y, 0.5));
    Assert.Equal(AssumptionId.Positivity, ex.Violation.Id);
    Assert.Equal(Subject.X, ex.Violation.Subject);
  }
}
