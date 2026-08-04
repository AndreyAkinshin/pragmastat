using Pragmastat.Distributions;
using Pragmastat.Randomization;

namespace Pragmastat.Tests.Distributions;

/// <summary>
/// The five parametric families refuse malformed parameters at construction. Every parameter has to
/// be finite, and every scale or shape parameter has to be positive.
///
/// Two gaps this covers. Each family used to check only its scale parameter, so a non-finite
/// location constructed a family whose every draw is NaN. And the predicate was written negatively
/// (<c>stdDev &lt;= 0</c>), which is not the negation of <c>stdDev &gt; 0</c> under IEEE 754:
/// <c>NaN &lt;= 0</c> is false, so NaN walked past the guard. The predicates are positive now, which
/// refuses NaN without a branch of its own.
/// </summary>
public class DistributionParameterTests
{
  private static string RefusalMessage(Action construct) =>
    Assert.Throws<ArgumentException>(construct).Message;

  [Fact]
  public void Uniform_RefusesNonFiniteBounds()
  {
    Assert.Contains("min must be finite", RefusalMessage(() => new Uniform(double.NaN, 1)));
    Assert.Contains("min must be finite", RefusalMessage(() => new Uniform(double.NegativeInfinity, 1)));
    Assert.Contains("max must be finite", RefusalMessage(() => new Uniform(0, double.NaN)));
    Assert.Contains("max must be finite", RefusalMessage(() => new Uniform(0, double.PositiveInfinity)));
  }

  [Fact]
  public void Uniform_RefusesAnEmptyInterval()
  {
    Assert.Contains("min must be less than max", RefusalMessage(() => new Uniform(1, 1)));
    Assert.Contains("min must be less than max", RefusalMessage(() => new Uniform(2, 1)));
  }

  [Fact]
  public void Additive_RefusesMalformedParameters()
  {
    Assert.Contains("mean must be finite", RefusalMessage(() => new Additive(double.NaN, 1)));
    Assert.Contains("mean must be finite", RefusalMessage(() => new Additive(double.PositiveInfinity, 1)));
    Assert.Contains("stdDev must be finite and positive", RefusalMessage(() => new Additive(0, double.NaN)));
    Assert.Contains("stdDev must be finite and positive",
      RefusalMessage(() => new Additive(0, double.PositiveInfinity)));
    Assert.Contains("stdDev must be finite and positive", RefusalMessage(() => new Additive(0, 0)));
    Assert.Contains("stdDev must be finite and positive", RefusalMessage(() => new Additive(0, -1)));
  }

  [Fact]
  public void Multiplic_RefusesMalformedParameters()
  {
    Assert.Contains("logMean must be finite", RefusalMessage(() => new Multiplic(double.NaN, 1)));
    Assert.Contains("logMean must be finite", RefusalMessage(() => new Multiplic(double.PositiveInfinity, 1)));
    Assert.Contains("logStdDev must be finite and positive", RefusalMessage(() => new Multiplic(0, double.NaN)));
    Assert.Contains("logStdDev must be finite and positive",
      RefusalMessage(() => new Multiplic(0, double.PositiveInfinity)));
    Assert.Contains("logStdDev must be finite and positive", RefusalMessage(() => new Multiplic(0, 0)));
    Assert.Contains("logStdDev must be finite and positive", RefusalMessage(() => new Multiplic(0, -1)));
  }

  [Fact]
  public void Exp_RefusesMalformedParameters()
  {
    Assert.Contains("rate must be finite and positive", RefusalMessage(() => new Exp(double.NaN)));
    Assert.Contains("rate must be finite and positive", RefusalMessage(() => new Exp(double.PositiveInfinity)));
    Assert.Contains("rate must be finite and positive", RefusalMessage(() => new Exp(0)));
    Assert.Contains("rate must be finite and positive", RefusalMessage(() => new Exp(-1)));
  }

  [Fact]
  public void Power_RefusesMalformedParameters()
  {
    Assert.Contains("min must be finite and positive", RefusalMessage(() => new Power(double.NaN, 3)));
    Assert.Contains("min must be finite and positive", RefusalMessage(() => new Power(double.PositiveInfinity, 3)));
    Assert.Contains("min must be finite and positive", RefusalMessage(() => new Power(0, 3)));
    Assert.Contains("min must be finite and positive", RefusalMessage(() => new Power(-1, 3)));
    Assert.Contains("shape must be finite and positive", RefusalMessage(() => new Power(1, double.NaN)));
    Assert.Contains("shape must be finite and positive", RefusalMessage(() => new Power(1, double.PositiveInfinity)));
    Assert.Contains("shape must be finite and positive", RefusalMessage(() => new Power(1, 0)));
    Assert.Contains("shape must be finite and positive", RefusalMessage(() => new Power(1, -1)));
  }

  [Fact]
  public void WellFormedFamilies_StillConstructAndDraw()
  {
    var rng = new Rng("distribution-parameters");
    var families = new IDistribution[]
    {
      new Uniform(-2.5, 7.5),
      new Additive(10, 2),
      new Multiplic(1, 0.5),
      new Exp(3),
      new Power(1, 2)
    };

    foreach (var family in families)
      Assert.True(double.IsFinite(family.Sample(rng)));
  }
}
