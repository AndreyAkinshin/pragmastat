using System;

namespace Pragmastat;

public enum ComparisonVerdict { Inconclusive, Less, Greater }

/// <summary>Extension helpers for <see cref="ComparisonVerdict"/>.</summary>
public static class ComparisonVerdictExtensions
{
  /// <summary>
  /// Canonical lowercase identifier ("less"/"greater"/"inconclusive"),
  /// matching the string every other language port emits.
  /// </summary>
  public static string ToId(this ComparisonVerdict verdict) => verdict switch
  {
    ComparisonVerdict.Less => "less",
    ComparisonVerdict.Greater => "greater",
    ComparisonVerdict.Inconclusive => "inconclusive",
    _ => throw new ArgumentOutOfRangeException(nameof(verdict), verdict, null)
  };
}
