using Pragmastat.Exceptions;

namespace Pragmastat.Tests;

/// <summary>
/// The canonical id of an enum member is the string every port emits and every fixture stores.
/// C# cannot put the operation on the enum itself, so each family carries a ToId extension, and
/// nothing but a test says the extensions still cover every member: a member added without a
/// switch arm throws only when something reaches it.
/// </summary>
public class EnumIdTests
{
  [Theory]
  [InlineData(AssumptionId.Validity, "validity")]
  [InlineData(AssumptionId.Domain, "domain")]
  [InlineData(AssumptionId.Positivity, "positivity")]
  [InlineData(AssumptionId.Sparity, "sparity")]
  [InlineData(AssumptionId.Distinctness, "distinctness")]
  [InlineData(AssumptionId.Cumulance, "cumulance")]
  public void AssumptionIdHasItsId(AssumptionId id, string expected) => Assert.Equal(expected, id.ToId());

  [Theory]
  [InlineData(Subject.X, "x")]
  [InlineData(Subject.Y, "y")]
  [InlineData(Subject.Misrate, "misrate")]
  public void SubjectHasItsId(Subject subject, string expected) => Assert.Equal(expected, subject.ToId());

  [Theory]
  [InlineData(ComparisonVerdict.Less, "less")]
  [InlineData(ComparisonVerdict.Greater, "greater")]
  [InlineData(ComparisonVerdict.Inconclusive, "inconclusive")]
  public void VerdictHasItsId(ComparisonVerdict verdict, string expected) =>
    Assert.Equal(expected, verdict.ToId());

  [Fact]
  public void EveryMemberOfEveryFamilyHasAnId()
  {
    foreach (var id in Enum.GetValues<AssumptionId>())
      Assert.NotEmpty(id.ToId());
    foreach (var subject in Enum.GetValues<Subject>())
      Assert.NotEmpty(subject.ToId());
    foreach (var verdict in Enum.GetValues<ComparisonVerdict>())
      Assert.NotEmpty(verdict.ToId());
  }

  /// <summary>
  /// A violation renders as its two ids, which is the form the fixtures record and the form
  /// AssumptionException carries as its message.
  /// </summary>
  [Fact]
  public void ViolationRendersAsBothIds()
  {
    var violation = new Violation(AssumptionId.Sparity, Subject.Y);

    Assert.Equal("sparity(y)", violation.ToString());
    Assert.Equal("sparity(y)", AssumptionException.Sparity(Subject.Y).Message);
  }
}
