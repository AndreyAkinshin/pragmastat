using Pragmastat.Internal;

namespace Pragmastat.Tests;

/// <summary>
/// ArgumentException takes (message, paramName) while ArgumentOutOfRangeException takes
/// (paramName, message). The two orders are opposite, and the library throws both, in places a few
/// lines apart. Passing the name first compiles, so nothing but a test tells the two apart: the
/// exception then reports a whole sentence as its ParamName and the parameter name as its Message.
///
/// The Sample overload of <see cref="Assertion.NotNullOrEmpty(string, Sample?)"/> is not covered:
/// every Sample constructor refuses an empty value list before it runs, so its empty branch is
/// unreachable. It was corrected alongside the reachable sites rather than left as the one
/// remaining example of the wrong shape.
/// </summary>
public class ArgumentExceptionShapeTests
{
  [Fact]
  public void EmptyList_ReportsTheParameterName()
  {
    var ex = Assert.Throws<ArgumentException>(
      () => Assertion.NotNullOrEmpty("values", Array.Empty<double>()));

    Assert.Equal("values", ex.ParamName);
    Assert.Contains("can't be empty", ex.Message);
  }

  [Fact]
  public void NonPositiveTotalWeight_ReportsTheParameterName()
  {
    var ex = Assert.Throws<ArgumentException>(
      () => new Sample([1.0, 2.0], [0.0, 0.0]));

    Assert.Equal("weights", ex.ParamName);
    Assert.Contains("should be positive", ex.Message);
  }

  /// <summary>
  /// The neighboring ArgumentOutOfRangeException site takes the opposite order, and is correct.
  /// Pinned so a later sweep does not "fix" it into agreement with the wrong shape.
  /// </summary>
  [Fact]
  public void NegativeWeight_ReportsTheParameterName()
  {
    var ex = Assert.Throws<ArgumentOutOfRangeException>(
      () => new Sample([1.0, 2.0], [-1.0, 2.0]));

    Assert.Equal("weights", ex.ParamName);
    Assert.Contains("non-negative", ex.Message);
  }
}
