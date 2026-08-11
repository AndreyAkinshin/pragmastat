using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;
using Pragmastat.Randomization;

namespace Pragmastat.Distributions;

/// <summary>
/// Piecewise-linear distribution reconstructed from a sample (algorithm <c>dist-linear-v1</c>).
/// </summary>
/// <remarks>
/// <para>
/// The model interpolates the order statistics at the anchors <c>(i-1)/(n-1)</c>, the
/// Hyndman-Fan type 7 rule. It is immutable: the constructor copies and, unless the caller
/// promises otherwise, sorts the input, so concurrent queries on a built model are safe.
/// </para>
/// <para>
/// Support is <c>[min(x), max(x)]</c>. v1 exposes plain numbers on the numeric path and does not
/// extend the runtime unit registry.
/// </para>
/// </remarks>
public sealed class LinearDistModel : IDistribution
{
  /// <summary>Normative algorithm identifier.</summary>
  public const string Algorithm = "dist-linear-v1";

  private readonly double[] x;

  /// <summary>
  /// Builds the model from a native numeric vector.
  /// </summary>
  /// <param name="values">
  /// Fitting vector; must be non-empty, finite, of size at least two, with a finite representable
  /// range and pairwise distinct elements.
  /// </param>
  /// <param name="assumeSorted">
  /// When true, the caller promises the input is already ascending and the internal copy is not
  /// sorted. Passing an unsorted array under this flag is a contract violation with an
  /// unspecified but terminating result.
  /// </param>
  public LinearDistModel(double[] values, bool assumeSorted = false)
  {
    Assertion.Validity(values, Subject.X);
    x = DistSampleGuard.Prepare(values, assumeSorted);
  }

  private LinearDistModel(double[] prepared)
  {
    x = prepared;
  }

  /// <summary>Number of observations backing the model.</summary>
  public int Size => x.Length;

  /// <summary>Lower end of the support.</summary>
  public double SupportMin => x[0];

  /// <summary>Upper end of the support.</summary>
  public double SupportMax => x[x.Length - 1];

  /// <summary>Value at each requested cumulance, in the order the cumulances were given.</summary>
  public double[] Quantile(double[] cumulances)
  {
    Assertion.QueryValidity(cumulances, Subject.Y);
    Assertion.CumulanceAssumption(cumulances, Subject.Y);
    return QuantileTrusted(cumulances);
  }

  /// <summary>Probability of not exceeding each requested value, in the order given.</summary>
  public double[] Cumulative(double[] values)
  {
    Assertion.QueryValidity(values, Subject.Y);
    return CumulativeTrusted(values);
  }

  /// <summary>Density at each requested value, in the order the values were given.</summary>
  public double[] Density(double[] values)
  {
    Assertion.QueryValidity(values, Subject.Y);
    return DensityTrusted(values);
  }

  private double[] QuantileTrusted(double[] cumulances)
  {
    double[] result = new double[cumulances.Length];
    for (int i = 0; i < cumulances.Length; i++)
      result[i] = QuantileAt(cumulances[i]);
    return result;
  }

  private double[] CumulativeTrusted(double[] values)
  {
    double[] result = new double[values.Length];
    for (int i = 0; i < values.Length; i++)
      result[i] = CumulativeAt(values[i]);
    return result;
  }

  private double[] DensityTrusted(double[] values)
  {
    double[] result = new double[values.Length];
    for (int i = 0; i < values.Length; i++)
      result[i] = DensityAt(values[i]);
    return result;
  }

  /// <summary>Unit of the observations this model was built from.</summary>
  public MeasurementUnit Unit { get; } = MeasurementUnit.Number;

  /// <summary>
  /// Builds the model from a measured sample.
  /// </summary>
  /// <remarks>
  /// The sample already stores its values in ascending order, so the model does not sort again.
  /// Weighted samples are refused: this reconstruction has no weighted form, and silently
  /// ignoring the weights would answer a different question than the caller asked.
  /// </remarks>
  public LinearDistModel(Sample x)
    : this(Prepared(x))
  {
    Unit = x.Unit;
  }

  private static double[] Prepared(Sample x)
  {
    Assertion.NotNull(nameof(x), x);
    Assertion.NonWeighted(nameof(x), x);
    // Sample construction already enforced validity, and its values are ascending.
    return DistSampleGuard.Prepare(x.SortedValues.ToArray(), true);
  }

  /// <summary>
  /// The measured view of this model, behind its own accessor rather than as overloads of the
  /// numeric methods: <see cref="Probability"/> converts implicitly from <c>double</c>, so
  /// overloading <c>Quantile</c> on the element type would make a literal argument ambiguous.
  /// </summary>
  public MeasuredView Measured => new(this);

  /// <summary>Measured queries against a built <see cref="LinearDistModel"/>.</summary>
  public readonly struct MeasuredView(LinearDistModel model)
  {
    private readonly LinearDistModel model = model;

    /// <summary>Values at the requested cumulances, in the unit of the sample.</summary>
    public Measurement[] Quantile(Probability[] cumulances)
    {
      Assertion.NotNull(nameof(cumulances), cumulances);
      double[] raw = new double[cumulances.Length];
      for (int i = 0; i < cumulances.Length; i++)
        raw[i] = cumulances[i].Value;
      return DistSampleGuard.ToMeasurements(model.Quantile(raw), model.Unit);
    }

    /// <summary>Probabilities of not exceeding the requested measurements.</summary>
    public Probability[] Cumulative(Measurement[] values)
    {
      double[] probabilities = model.Cumulative(DistSampleGuard.ToRaw(values, model.Unit));
      Probability[] result = new Probability[probabilities.Length];
      for (int i = 0; i < probabilities.Length; i++)
        result[i] = new Probability(probabilities[i]);
      return result;
    }

    /// <summary>Densities at the requested measurements, as plain numbers.</summary>
    public double[] Density(Measurement[] values) =>
      model.Density(DistSampleGuard.ToRaw(values, model.Unit));
  }

  /// <inheritdoc />
  public double Sample(Rng rng)
  {
    Assertion.NotNull(nameof(rng), rng);
    return QuantileAt(rng.UniformDouble());
  }

  /// <inheritdoc />
  public List<double> Samples(Rng rng, int count)
  {
    Assertion.NotNull(nameof(rng), rng);
    List<double> result = new(count);
    for (int i = 0; i < count; i++)
      result.Add(Sample(rng));
    return result;
  }

  /// <inheritdoc />
  /// <remarks>
  /// NaN first: every branch below compares false against it, and the fallback arithmetic would
  /// otherwise depend on how the platform casts a NaN cumulance to an index. The same guard opens
  /// <see cref="CumulativeAt"/> and <see cref="DensityAt"/>, so all three answer the same way to
  /// the same non-input.
  /// </remarks>
  public double QuantileAt(double p)
  {
    if (double.IsNaN(p))
      return double.NaN;

    int n = x.Length;
    double value;
    if (p <= 0.0)
      value = x[0];
    else if (p >= 1.0)
      value = x[n - 1];
    else
    {
      double r = (n - 1) * p;
      int j = (int)Floor(r);
      double g = r - j;
      value = DistNumerics.Lerp(x[j], x[j + 1], g);
    }

    // Every branch hands back a value the sample supplied -- the clamped ones return an order
    // statistic verbatim, and Lerp reproduces its endpoints exactly -- so a -0.0 among the
    // observations would otherwise reach the caller unnormalized.
    return value.NormalizeZero();
  }

  /// <inheritdoc />
  public double CumulativeAt(double x)
  {
    if (double.IsNaN(x))
      return double.NaN;

    int n = this.x.Length;
    if (x <= this.x[0])
      return 0.0;
    if (x >= this.x[n - 1])
      return 1.0;

    int j = DistNumerics.UpperBound(this.x, x) - 1;
    double t = DistNumerics.UnitRatio(x - this.x[j], this.x[j + 1] - this.x[j]);
    return (j + t) / (n - 1);
  }

  /// <inheritdoc />
  public double DensityAt(double x)
  {
    if (double.IsNaN(x))
      return double.NaN;

    int n = this.x.Length;
    if (x < this.x[0] || x > this.x[n - 1])
      return 0.0;

    int j = x == this.x[n - 1] ? n - 2 : DistNumerics.UpperBound(this.x, x) - 1;
    return 1.0 / (n - 1) / (this.x[j + 1] - this.x[j]);
  }

  /// <summary>
  /// The staircase this model is: cell boundaries, cumulative values and heights.
  /// </summary>
  /// <remarks>
  /// Point evaluation answers what the density is at a value a caller picked; this answers what
  /// the density <em>is</em>, on the boundaries the model itself uses. Drawing the second from the
  /// first means guessing a grid and rebuilding the normalization on top of it -- a second
  /// reconstruction of the same object that drifts from this one silently. Every height is the
  /// expression <see cref="DensityAt"/> evaluates inside the same cell, so the two agree bit for
  /// bit rather than approximately.
  /// </remarks>
  public LinearLandscape GetLandscape()
  {
    int n = x.Length;
    double[] cumulative = new double[n];
    // The general CumulativeAt path for an exact order statistic x[i] always resolves j = i and
    // t = 0.0, so it reduces to exactly (double)i / (n - 1) -- computing that directly skips the
    // O(log n) binary search per boundary without changing a single bit of the result.
    for (int i = 0; i < n; i++)
      cumulative[i] = (double)i / (n - 1);
    double[] densities = new double[n - 1];
    for (int j = 0; j < n - 1; j++)
      densities[j] = 1.0 / (n - 1) / (x[j + 1] - x[j]);
    return new LinearLandscape((double[])x.Clone(), cumulative, densities);
  }
}
