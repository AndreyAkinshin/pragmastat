using System.Globalization;
using System.Text;
using JetBrains.Annotations;
using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat;

public class Sample
{
  public IReadOnlyList<double> Values { get; }
  public IReadOnlyList<double>? Weights { get; }
  public double TotalWeight { get; }
  public bool IsWeighted { get; }
  public MeasurementUnit Unit { get; }

  private readonly Lazy<(IReadOnlyList<double> SortedValues, IReadOnlyList<double>? SortedWeights)> lazySortedData;

  public IReadOnlyList<double> SortedValues => lazySortedData.Value.SortedValues;
  public IReadOnlyList<double>? SortedWeights => lazySortedData.Value.SortedWeights;

  /// <summary>
  /// Sample size (always positive)
  /// </summary>
  public int Size => Values.Count;

  /// <summary>
  /// Kish's Effective Sample Size
  /// </summary>
  public double WeightedSize { get; }

  public Sample(params double[] values) : this((IReadOnlyList<double>)values, null, null)
  {
  }

  public Sample(params int[] values) : this(Array.ConvertAll(values, x => (double)x), null, null)
  {
  }

  internal Subject? ValidationSubject { get; }

  public Sample(IReadOnlyList<double> values, MeasurementUnit? unit = null, Subject? validationSubject = null)
  {
    // Validate validity assumption at construction time
    if (values == null)
      throw new ArgumentNullException(nameof(values), "values can't be null");
    ValidationSubject = validationSubject ?? Subject.X;
    if (values.Count == 0)
      throw AssumptionException.Validity(ValidationSubject.Value);
    foreach (var value in values)
    {
      if (!value.IsFinite())
        throw AssumptionException.Validity(ValidationSubject.Value);
    }

    Values = values;
    Unit = unit ?? MeasurementUnit.Number;
    Weights = null;
    TotalWeight = 1.0;
    WeightedSize = values.Count;
    IsWeighted = false;

    lazySortedData = new Lazy<(IReadOnlyList<double>, IReadOnlyList<double>?)>(() =>
      (IsSorted(Values) ? Values : Values.CopyToArrayAndSort(), null));
  }

  public Sample(IReadOnlyList<double> values, IReadOnlyList<double> weights, MeasurementUnit? measurementUnit = null, Subject? validationSubject = null)
  {
    // Validate validity assumption at construction time
    if (values == null)
      throw new ArgumentNullException(nameof(values), "values can't be null");
    ValidationSubject = validationSubject ?? Subject.X;
    if (values.Count == 0)
      throw AssumptionException.Validity(ValidationSubject.Value);
    foreach (var value in values)
    {
      if (!value.IsFinite())
        throw AssumptionException.Validity(ValidationSubject.Value);
    }

    Assertion.NotNullOrEmpty(nameof(weights), weights);
    if (values.Count != weights.Count)
      throw new ArgumentException(
        $"{nameof(weights)} should have the same number of elements as {nameof(values)}",
        nameof(weights));

    double totalWeight = 0, maxWeight = double.MinValue, minWeight = double.MaxValue;
    double totalWeightSquared = 0;
    foreach (double weight in weights)
    {
      totalWeight += weight;
      totalWeightSquared += weight.Sqr();
      maxWeight = Math.Max(maxWeight, weight);
      minWeight = Math.Min(minWeight, weight);
    }

    if (minWeight < 0)
      throw new ArgumentOutOfRangeException(nameof(weights),
        $"All weights in {nameof(weights)} should be non-negative");
    if (totalWeight < 1e-9)
      throw new ArgumentException(nameof(weights),
        $"The sum of all elements from {nameof(weights)} should be positive");

    Values = values;
    Weights = weights;
    Unit = measurementUnit ?? MeasurementUnit.Number;
    TotalWeight = totalWeight;
    WeightedSize = totalWeight.Sqr() / totalWeightSquared;
    IsWeighted = true;

    lazySortedData = new Lazy<(IReadOnlyList<double>, IReadOnlyList<double>?)>(() =>
    {
      if (IsSorted(Values))
        return (Values, Weights);

      double[] sortedValues = Values.CopyToArray();
      double[] sortedWeights = Weights!.CopyToArray();
      Array.Sort(sortedValues, sortedWeights);

      return (sortedValues, sortedWeights);
    });
  }

  [PublicAPI]
  public Sample(IEnumerable<int> values, MeasurementUnit? unit = null)
    : this(values.Select(x => (double)x).ToList(), unit)
  {
  }

  public Sample(IEnumerable<long> values, MeasurementUnit? unit = null)
    : this(values.Select(x => (double)x).ToList(), unit)
  {
  }

  public Sample ConvertTo(MeasurementUnit target)
  {
    if (!Unit.IsCompatible(target))
      throw new UnitMismatchException(Unit, target);
    if (Unit == target) return this;
    double factor = MeasurementUnit.ConversionFactor(Unit, target);
    double[] converted = new double[Size];
    for (int i = 0; i < Size; i++)
      converted[i] = Values[i] * factor;
    return IsWeighted
      ? new Sample(converted, Weights!, target)
      : new Sample(converted, target);
  }

  public Sample Concat(Sample sample)
  {
    if (!Unit.IsCompatible(sample.Unit))
      throw new UnitMismatchException(Unit, sample.Unit);

    var target = MeasurementUnit.Finer(Unit, sample.Unit);

    IEnumerable<double> GetValues(Sample s)
    {
      if (s.Unit == target)
        return s.Values;
      double factor = MeasurementUnit.ConversionFactor(s.Unit, target);
      return s.Values.Select(x => x * factor);
    }

    double[] UniformWeights(int count)
    {
      double w = 1.0 / count;
      double[] result = new double[count];
      for (int i = 0; i < count; i++)
        result[i] = w;
      return result;
    }

    var values1 = GetValues(this);
    var values2 = GetValues(sample);
    var weights1 = Weights ?? UniformWeights(Size);
    var weights2 = sample.Weights ?? UniformWeights(sample.Size);
    return new Sample(values1.Concat(values2).ToList(), weights1.Concat(weights2).ToList(), target);
  }

  public override string ToString()
  {
    var builder = new StringBuilder();
    builder.Append('[');
    for (int i = 0; i < Values.Count; i++)
    {
      if (i != 0) builder.Append(',');
      builder.Append(Values[i].ToString("G", CultureInfo.InvariantCulture));
    }
    builder.Append(']');
    if (Unit.Abbreviation.Length > 0)
      builder.Append(Unit.Abbreviation);
    return builder.ToString();
  }

  public static Sample operator *(Sample sample, double value)
  {
    double[] values = new double[sample.Size];
    for (int i = 0; i < sample.Size; i++)
      values[i] = sample.Values[i] * value;
    return sample.IsWeighted ? new Sample(values, sample.Weights!, sample.Unit) : new Sample(values, sample.Unit);
  }

  public static Sample operator /(Sample sample, double value)
  {
    double[] values = new double[sample.Size];
    for (int i = 0; i < sample.Size; i++)
      values[i] = sample.Values[i] / value;
    return sample.IsWeighted ? new Sample(values, sample.Weights!, sample.Unit) : new Sample(values, sample.Unit);
  }

  public static Sample operator +(Sample sample, double value)
  {
    double[] values = new double[sample.Size];
    for (int i = 0; i < sample.Size; i++)
      values[i] = sample.Values[i] + value;
    return sample.IsWeighted ? new Sample(values, sample.Weights!, sample.Unit) : new Sample(values, sample.Unit);
  }

  public static Sample operator -(Sample sample, double value)
  {
    double[] values = new double[sample.Size];
    for (int i = 0; i < sample.Size; i++)
      values[i] = sample.Values[i] - value;
    return sample.IsWeighted ? new Sample(values, sample.Weights!, sample.Unit) : new Sample(values, sample.Unit);
  }

  public static Sample operator *(double value, Sample sample) => sample * value;
  public static Sample operator +(double value, Sample sample) => sample + value;

  public static Sample operator *(Sample sample, int value) => sample * (double)value;
  public static Sample operator /(Sample sample, int value) => sample / (double)value;
  public static Sample operator +(Sample sample, int value) => sample + (double)value;
  public static Sample operator -(Sample sample, int value) => sample - (double)value;

  public static Sample operator *(int value, Sample sample) => sample * value;
  public static Sample operator +(int value, Sample sample) => sample + value;

  private static bool IsSorted(IReadOnlyList<double> list)
  {
    for (int i = 0; i < list.Count - 1; i++)
      if (list[i] > list[i + 1])
        return false;
    return true;
  }

  public Measurement Min() => SortedValues[0].WithUnit(Unit);
  public Measurement Max() => SortedValues[SortedValues.Count - 1].WithUnit(Unit);

  public Sample Log()
  {
    double[] logValues = new double[Size];
    for (int i = 0; i < Size; i++)
    {
      if (Values[i] <= 0)
        throw AssumptionException.Positivity(ValidationSubject ?? Subject.X);
      logValues[i] = Math.Log(Values[i]);
    }
    return IsWeighted
      ? new Sample(logValues, Weights!, MeasurementUnit.Number)
      : new Sample(logValues, MeasurementUnit.Number);
  }
}
