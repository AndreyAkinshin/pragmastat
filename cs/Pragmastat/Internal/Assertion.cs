using System.Globalization;
using JetBrains.Annotations;
using Pragmastat.Exceptions;
using Pragmastat.Metrology;

namespace Pragmastat.Internal;

internal static class Assertion
{
  [AssertionMethod]
  public static void NotNull(string name, object? value)
  {
    if (value == null)
      throw new ArgumentNullException(name, $"{name} can't be null");
  }

  [AssertionMethod]
  public static void NotNullOrEmpty<T>(string name, IReadOnlyList<T>? values)
  {
    if (values == null)
      throw new ArgumentNullException(name, $"{name} can't be null");
    if (values.Count == 0)
      throw new ArgumentException(name, $"{name} can't be empty");
  }

  [AssertionMethod]
  public static void NotNullOrEmpty(string name, Sample? sample)
  {
    if (sample == null)
      throw new ArgumentNullException(name, $"{name} can't be null");
    if (sample.Size == 0)
      throw new ArgumentException(name, $"{name} can't be empty");
  }

  [AssertionMethod]
  public static void ItemNotNull<T>(string name, IReadOnlyList<T> values)
  {
    for (int i = 0; i < values.Count; i++)
      if (values[i] is null)
        throw new ArgumentNullException($"{name}[{i}] is null, but {name} should not contain null items");
  }

  [AssertionMethod]
  public static void InRangeInclusive(string name, double value, double min, double max)
  {
    if (value < min || value > max)
    {
      string message = Format("{0}={1}, but it should be in range [{2};{3}]", name, value, min, max);
      throw new ArgumentOutOfRangeException(name, value, message);
    }
  }

  [AssertionMethod]
  public static void InRangeInclusive(string name, int value, int min, int max)
  {
    if (value < min || value > max)
    {
      string message = Format("{0}={1}, but it should be in range [{2};{3}]", name, value, min, max);
      throw new ArgumentOutOfRangeException(name, value, message);
    }
  }

  [AssertionMethod]
  public static void InRangeInclusive(string name, IReadOnlyList<double> values, double min, double max)
  {
    for (int i = 0; i < values.Count; i++)
    {
      double value = values[i];
      if (value < min || value > max)
      {
        string message = Format("{0}[{1}]={2}, but it should be in range [{3};{4}]", name, i, value, min, max);
        throw new ArgumentOutOfRangeException(name, value, message);
      }
    }
  }

  [AssertionMethod]
  public static void InRangeExclusive(string name, double value, double min, double max)
  {
    if (value <= min || value >= max)
    {
      string message = Format("{0}={1}, but it should be in range ({2};{3})", name, value, min, max);
      throw new ArgumentOutOfRangeException(name, value, message);
    }
  }

  [AssertionMethod]
  public static void Positive(string name, double value)
  {
    if (value <= 0)
    {
      string message = Format("{0}={1}, but it should be positive", name, value);
      throw new ArgumentOutOfRangeException(name, value, message);
    }
  }

  [AssertionMethod]
  public static void Positive(string name, int value)
  {
    if (value <= 0)
    {
      string message = Format("{0}={1}, but it should be positive", name, value);
      throw new ArgumentOutOfRangeException(name, value, message);
    }
  }

  [AssertionMethod]
  public static void Positive(string name, IReadOnlyList<double> values)
  {
    for (int i = 0; i < values.Count; i++)
    {
      double value = values[i];
      if (value <= 0)
      {
        string message = Format("{0}[{1}]={2}, but it should be positive", name, i, value);
        throw new ArgumentOutOfRangeException(name, value, message);
      }
    }
  }

  [AssertionMethod]
  public static void NonNegative(string name, double value)
  {
    if (value < 0)
    {
      string message = Format("{0}={1}, but it should be non-negative", name, value);
      throw new ArgumentOutOfRangeException(name, value, message);
    }
  }

  [AssertionMethod]
  public static void Finite(string name, double value)
  {
    if (!value.IsFinite())
      throw new ArgumentOutOfRangeException(name, $"{name} should have a finite value");
  }

  [AssertionMethod]
  public static void MoreThan(string name, double value, double threshold)
  {
    if (value <= threshold)
    {
      string message = Format("{0}={1}, but it should be more than {2}", name, value, threshold);
      throw new ArgumentOutOfRangeException(name, value, message);
    }
  }

  [AssertionMethod]
  public static void MoreThan(string name, double value, int threshold)
  {
    if (value <= threshold)
    {
      string message = Format("{0}={1}, but it should be more than {2}", name, value, threshold);
      throw new ArgumentOutOfRangeException(name, value, message);
    }
  }

  [AssertionMethod]
  public static void MoreThan(string name, int value, int threshold)
  {
    if (value <= threshold)
    {
      string message = Format("{0}={1}, but it should be more than {2}", name, value, threshold);
      throw new ArgumentOutOfRangeException(name, value, message);
    }
  }

  [AssertionMethod]
  public static void SizeLargerThan(string name, Sample sample, int threshold)
  {
    if (sample.Size <= threshold)
    {
      string message = Format("{0}.Count={1}, but it should be more than {2}", name, sample.Size, threshold);
      throw new ArgumentOutOfRangeException(name, sample.Size, message);
    }
  }

  [AssertionMethod]
  public static void Equal(string name1, int value1, string name2, int value2)
  {
    if (value1 != value2)
    {
      string message = Format("{0}={1}, {2}={3}, but {0} and {2} should be equal", name1, value1, name2, value2);
      throw new ArgumentOutOfRangeException(name1, value1, message);
    }
  }

  [AssertionMethod]
  public static void CompatibleUnits(Sample sample1, Sample sample2)
  {
    if (!sample1.Unit.IsCompatible(sample2.Unit))
      throw new UnitMismatchException(sample1.Unit, sample2.Unit);
  }

  public static (Sample, Sample) ConvertToFiner(Sample a, Sample b)
  {
    if (a.Unit == b.Unit) return (a, b);
    var target = MeasurementUnit.Finer(a.Unit, b.Unit);
    return (a.ConvertTo(target), b.ConvertTo(target));
  }

  [AssertionMethod]
  public static void Equal(string name, double value, double expectedValue, double eps = 1e-9)
  {
    if (Abs(value - expectedValue) > eps)
    {
      string message = Format("{0}={1}, but it should be equal to {2}", name, value, expectedValue);
      throw new ArgumentOutOfRangeException(name, value, message);
    }
  }

  [AssertionMethod]
  public static void NonWeighted(string name, Sample? sample)
  {
    NotNull(name, sample);
    if (sample is { IsWeighted: true })
      throw new WeightedSampleNotSupportedException(
        $"Weighted samples are not supported (parameter '{name}').", name);
  }

  // For the Sample-based API the Sample constructor already guarantees non-empty input
  // with finite values, so validity is enforced at construction time. The raw native-array
  // API bypasses Sample construction, so it must run this check explicitly.

  /// <summary>
  /// Checks the validity assumption for a raw value collection: it must be non-empty and
  /// contain only finite values. Used by the raw native-array API, where there is no
  /// Sample constructor to enforce it.
  /// </summary>
  [AssertionMethod]
  public static void Validity(IReadOnlyList<double>? values, Subject subject)
  {
    if (values == null || values.Count == 0)
      throw AssumptionException.Validity(subject);
    for (int i = 0; i < values.Count; i++)
      if (!values[i].IsFinite())
        throw AssumptionException.Validity(subject);
  }

  /// <summary>
  /// Checks that all values in a raw collection are strictly positive.
  /// </summary>
  [AssertionMethod]
  public static void PositivityAssumption(IReadOnlyList<double> values, Subject subject)
  {
    for (int i = 0; i < values.Count; i++)
      if (values[i] <= 0)
        throw AssumptionException.Positivity(subject);
  }

  [StringFormatMethod("format")]
  private static string Format(string format, params object[] args) =>
    string.Format(CultureInfo.InvariantCulture, format, args);
}
