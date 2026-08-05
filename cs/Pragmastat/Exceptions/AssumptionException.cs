namespace Pragmastat.Exceptions;

/// <summary>
/// Assumption identifiers in canonical priority order.
/// Lower values indicate higher priority.
/// </summary>
public enum AssumptionId
{
  /// <summary>Non-empty input with finite defined real values.</summary>
  Validity,

  /// <summary>Parameter value is outside the achievable domain.</summary>
  Domain,

  /// <summary>Values must be strictly positive.</summary>
  Positivity,

  /// <summary>Sample must be non tie-dominant (Spread &gt; 0).</summary>
  Sparity
}

/// <summary>
/// Identifies which input caused an assumption violation.
/// </summary>
public enum Subject
{
  /// <summary>First sample (x).</summary>
  X,

  /// <summary>Second sample (y).</summary>
  Y,

  /// <summary>Misrate parameter.</summary>
  Misrate
}

/// <summary>Extension helpers for <see cref="AssumptionId"/>.</summary>
public static class AssumptionIdExtensions
{
  /// <summary>
  /// Canonical lowercase identifier ("validity"/"domain"/"positivity"/"sparity"),
  /// matching the string every other language port emits.
  /// </summary>
  public static string ToId(this AssumptionId id) => id switch
  {
    AssumptionId.Validity => "validity",
    AssumptionId.Domain => "domain",
    AssumptionId.Positivity => "positivity",
    AssumptionId.Sparity => "sparity",
    _ => throw new ArgumentOutOfRangeException(nameof(id), id, null)
  };
}

/// <summary>Extension helpers for <see cref="Subject"/>.</summary>
public static class SubjectExtensions
{
  /// <summary>
  /// Canonical lowercase identifier ("x"/"y"/"misrate"), matching the string every
  /// other language port emits.
  /// </summary>
  public static string ToId(this Subject subject) => subject switch
  {
    Subject.X => "x",
    Subject.Y => "y",
    Subject.Misrate => "misrate",
    _ => throw new ArgumentOutOfRangeException(nameof(subject), subject, null)
  };
}

/// <summary>
/// Represents a specific assumption violation.
/// </summary>
public readonly struct Violation
{
  public AssumptionId Id { get; }
  public Subject Subject { get; }

  public Violation(AssumptionId id, Subject subject)
  {
    Id = id;
    Subject = subject;
  }

  public override string ToString() => $"{Id.ToId()}({Subject.ToId()})";

  public override bool Equals(object? obj) =>
    obj is Violation other && Id == other.Id && Subject == other.Subject;

  public override int GetHashCode() => unchecked(Id.GetHashCode() * 31 + Subject.GetHashCode());

  public static bool operator ==(Violation left, Violation right) => left.Equals(right);
  public static bool operator !=(Violation left, Violation right) => !left.Equals(right);
}

/// <summary>
/// Exception thrown when an assumption is violated.
/// </summary>
public class AssumptionException : ArgumentException
{
  public Violation Violation { get; }

  public AssumptionException(Violation violation)
    : base(violation.ToString())
  {
    Violation = violation;
  }

  public static AssumptionException Validity(Subject subject) =>
    new(new Violation(AssumptionId.Validity, subject));

  public static AssumptionException Positivity(Subject subject) =>
    new(new Violation(AssumptionId.Positivity, subject));

  public static AssumptionException Sparity(Subject subject) =>
    new(new Violation(AssumptionId.Sparity, subject));

  public static AssumptionException Domain(Subject subject) =>
    new(new Violation(AssumptionId.Domain, subject));
}
