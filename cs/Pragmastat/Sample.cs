using JetBrains.Annotations;
using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat;

public class Sample
{
  public IReadOnlyList<double> Values { get; }
  public IReadOnlyList<double> Weights { get; }
  public double TotalWeight { get; }
  public bool IsWeighted { get; }
  public MeasurementUnit Unit { get; }

  private readonly Lazy<(IReadOnlyList<double> SortedValues, IReadOnlyList<double> SortedWeights)> lazySortedData;

  public IReadOnlyList<double> SortedValues => lazySortedData.Value.SortedValues;
  public IReadOnlyList<double> SortedWeights => lazySortedData.Value.SortedWeights;

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
      throw AssumptionException.Validity("Sample", ValidationSubject.Value);
    foreach (var value in values)
    {
      if (!value.IsFinite())
        throw AssumptionException.Validity("Sample", ValidationSubject.Value);
    }

    Values = values;
    Unit = unit ?? NumberUnit.Instance;
    double weight = 1.0 / values.Count;
    Weights = new IdenticalReadOnlyList<double>(values.Count, weight);
    TotalWeight = 1.0;
    WeightedSize = values.Count;
    IsWeighted = false;

    lazySortedData = new Lazy<(IReadOnlyList<double> SortedValues, IReadOnlyList<double> SortedWeights)>(() =>
    {
      if (IsSorted(Values))
        return (Values, Weights);
      return (Values.CopyToArrayAndSort(), Weights);
    });
  }

  public Sample(IReadOnlyList<double> values, IReadOnlyList<double> weights, MeasurementUnit? measurementUnit = null, Subject? validationSubject = null)
  {
    // Validate validity assumption at construction time
    if (values == null)
      throw new ArgumentNullException(nameof(values), "values can't be null");
    ValidationSubject = validationSubject ?? Subject.X;
    if (values.Count == 0)
      throw AssumptionException.Validity("Sample", ValidationSubject.Value);
    foreach (var value in values)
    {
      if (!value.IsFinite())
        throw AssumptionException.Validity("Sample", ValidationSubject.Value);
    }

    Assertion.NotNullOrEmpty(nameof(weights), weights);
    if (values.Count != weights.Count)
      throw new ArgumentException(
        $"{nameof(weights)} should have the same number of elements as {nameof(values)}",
        nameof(weights));

    double totalWeight = 0, maxWeight = double.MinValue, minWeight = double.MaxValue;
    double totalWeightSquared = 0; // Sum of weight squares
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
    Unit = measurementUnit ?? NumberUnit.Instance;
    TotalWeight = totalWeight;
    WeightedSize = totalWeight.Sqr() / totalWeightSquared;
    IsWeighted = true;

    lazySortedData = new Lazy<(IReadOnlyList<double> SortedValues, IReadOnlyList<double> SortedWeights)>(() =>
    {
      if (IsSorted(Values))
        return (Values, Weights);

      double[] sortedValues = Values.CopyToArray();
      double[] sortedWeights = Weights.CopyToArray();
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

  public Sample Concat(Sample sample)
  {
    if (Unit.GetFlavor() != sample.Unit.GetFlavor())
      throw new ArgumentException(
        $"Different measurement unit flavors: " +
        $"{Unit.GetFlavor()} vs. {sample.Unit.GetFlavor()}",
        nameof(sample));

    var unit1 = Unit;
    var unit2 = sample.Unit;
    var unit = unit1.BaseUnits < unit2.BaseUnits ? unit1 : unit2;

    IEnumerable<double> GetValues(Sample s)
    {
      if (unit.BaseUnits == s.Unit.BaseUnits)
        return s.Values;
      double ratio = s.Unit.BaseUnits * 1.0 / unit.BaseUnits;
      return s.Values.Select(x => x * ratio);
    }

    var values1 = GetValues(this);
    var values2 = GetValues(sample);
    var weights1 = Weights;
    var weights2 = sample.Weights;
    return new Sample(values1.Concat(values2).ToList(), weights1.Concat(weights2).ToList(), unit);
  }

  public override string ToString() => SampleFormatter.Default.Format(this);

  public static Sample operator *(Sample sample, double value)
  {
    double[] values = new double[sample.Size];
    for (int i = 0; i < sample.Size; i++)
      values[i] = sample.Values[i] * value;
    return sample.IsWeighted ? new Sample(values, sample.Weights, sample.Unit) : new Sample(values, sample.Unit);
  }

  public static Sample operator /(Sample sample, double value)
  {
    double[] values = new double[sample.Size];
    for (int i = 0; i < sample.Size; i++)
      values[i] = sample.Values[i] / value;
    return sample.IsWeighted ? new Sample(values, sample.Weights, sample.Unit) : new Sample(values, sample.Unit);
  }

  public static Sample operator +(Sample sample, double value)
  {
    double[] values = new double[sample.Size];
    for (int i = 0; i < sample.Size; i++)
      values[i] = sample.Values[i] + value;
    return sample.IsWeighted ? new Sample(values, sample.Weights, sample.Unit) : new Sample(values, sample.Unit);
  }

  public static Sample operator -(Sample sample, double value)
  {
    double[] values = new double[sample.Size];
    for (int i = 0; i < sample.Size; i++)
      values[i] = sample.Values[i] - value;
    return sample.IsWeighted ? new Sample(values, sample.Weights, sample.Unit) : new Sample(values, sample.Unit);
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
        throw AssumptionException.Positivity("Log", ValidationSubject ?? Subject.X);
      logValues[i] = Math.Log(Values[i]);
    }
    return new Sample(logValues, Weights);
  }
}
