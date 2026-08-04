namespace Pragmastat.Tests;

/// <summary>
/// Probability declares IComparable, whose contract is "compare me to another of my own type".
/// The non-generic overload used to forward to double.CompareTo(object), which accepts only a
/// boxed double, so the type could not compare itself to itself: every non-generic sort path
/// threw where an ordering was due. Measurement already had the right shape, and this follows it.
/// </summary>
public class ProbabilityComparisonTests
{
  [Fact]
  public void ComparesToAnotherProbability()
  {
    IComparable p = new Probability(0.5);

    Assert.Equal(1, p.CompareTo(new Probability(0.25)));
    Assert.Equal(-1, p.CompareTo(new Probability(0.75)));
    Assert.Equal(0, p.CompareTo(new Probability(0.5)));
  }

  [Fact]
  public void SortsThroughTheNonGenericPath()
  {
    object[] values = [new Probability(0.75), new Probability(0.25), new Probability(0.5)];

    Array.Sort(values);

    Assert.Equal([0.25, 0.5, 0.75], values.Select(v => ((Probability)v).Value));
  }

  [Fact]
  public void SortsNullFirst()
  {
    object?[] values = [new Probability(0.5), null];

    Array.Sort(values);

    Assert.Null(values[0]);
  }

  /// <summary>
  /// A boxed double used to be the one thing the non-generic overload accepted. It is refused now,
  /// which is what IComparable asks for: Probability is not a double, it is a double that has been
  /// checked. The implicit conversion remains available for callers who want the number.
  /// </summary>
  [Fact]
  public void RefusesAnUnrelatedType()
  {
    IComparable p = new Probability(0.5);

    Assert.Throws<ArgumentException>(() => p.CompareTo(0.5));
    Assert.Throws<ArgumentException>(() => p.CompareTo("0.5"));
  }
}
