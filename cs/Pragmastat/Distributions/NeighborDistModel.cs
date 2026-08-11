using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;
using Pragmastat.Randomization;

namespace Pragmastat.Distributions;

/// <summary>
/// Local step-density distribution reconstructed from a sample (algorithm
/// <c>dist-neighbor-v1</c>).
/// </summary>
/// <remarks>
/// <para>
/// Each observation contributes a cell whose height reflects how tightly its nearest neighbors
/// are packed, so the reconstruction follows local structure instead of imposing a single global
/// shape. The window is indexed by <c>k0 = ceil(sqrt(n))</c> with radius <c>r = floor(k0 / 2)</c>;
/// windows near the ends are truncated and the numerator is always the actual neighbor count.
/// </para>
/// <para>
/// The construction is kNN-inspired but is not the Loftsgaarden-Quesenberry estimator: support is
/// bounded by the observed range, and the cell masses are explicitly normalized so the density
/// integrates to one.
/// </para>
/// <para>
/// The model is immutable, so concurrent queries on a built model are safe.
/// </para>
/// </remarks>
public sealed class NeighborDistModel : IDistribution
{
  /// <summary>Normative algorithm identifier.</summary>
  public const string Algorithm = "dist-neighbor-v1";

  private readonly double[] b;
  private readonly double[] f;
  private readonly double[] p;
  private readonly double[] d;
  private readonly int[] sources;
  private readonly int scaleIndex;
  private readonly int radius;

  /// <summary>
  /// Builds the model from a native numeric vector.
  /// </summary>
  /// <param name="values">
  /// Fitting vector; must be non-empty, finite, of size at least two, with a finite representable
  /// range and pairwise distinct elements.
  /// </param>
  /// <param name="assumeSorted">
  /// When true, the caller promises the input is already ascending and the internal copy is not
  /// sorted.
  /// </param>
  public NeighborDistModel(double[] values, bool assumeSorted = false)
    : this(Guard(values, assumeSorted))
  {
  }

  private NeighborDistModel(double[] x)
  {
    int n = x.Length;
    scaleIndex = DistNumerics.CeilSqrt(n);
    radius = scaleIndex / 2;

    int[] counts = new int[n];
    double[] spans = new double[n];
    for (int i = 0; i < n; i++)
    {
      int left = Max(0, i - radius);
      int right = Min(n - 1, i + radius);
      counts[i] = right - left + 1;
      spans[i] = x[right] - x[left];
    }

    double[] e = new double[n + 1];
    e[0] = x[0];
    for (int i = 1; i < n; i++)
      e[i] = DistNumerics.Midpoint(x[i - 1], x[i]);
    e[n] = x[n - 1];

    // Rounding can collapse the midpoint of two adjacent binary64 observations onto a
    // neighboring boundary. Such a cell carries no mass and is dropped; the surviving cells
    // keep the index of the observation they came from.
    List<double> boundaries = new(n + 1) { e[0] };
    List<int> surviving = new(n);
    for (int i = 0; i < n; i++)
    {
      if (e[i + 1] == e[i])
        continue;
      surviving.Add(i);
      boundaries.Add(e[i + 1]);
    }

    int s = surviving.Count;
    double[] a = new double[s];
    for (int j = 0; j < s; j++)
    {
      int i = surviving[j];
      double width = e[i + 1] - e[i];
      a[j] = counts[i] / (double)n * (width / spans[i]);
    }

    double total = 0.0;
    for (int j = 0; j < s; j++)
      total = total + a[j];

    // A broken assumeSorted promise can make a cell wider than the window it normalizes against,
    // so its unnormalized mass overflows, or it can collapse every cell and leave a zero total.
    // Either way the divisor is unusable and NaN would reach the query kernels. The sample itself
    // satisfies domain(x) and distinctness(x), so this is not an assumption violation: it is a
    // numerical-conformance failure, reported through the same channel UnitRatio uses. For a
    // genuinely ascending sample the total is finite and strictly positive -- it is not bounded
    // by 1, since it is a normalizer rather than a probability; it sits above 1 and decays toward
    // it as n grows.
    if (!total.IsFinite() || total <= 0.0)
      throw new InvalidOperationException(
        "Numerical conformance failure: the NeighborDistModel normalizer is not usable. "
        + "This indicates an assumeSorted promise that the input does not satisfy.");

    // Reserve one representable step for every cell that has not been placed yet, so a mass too
    // small to register at this point in the prefix still advances the cumulative value instead
    // of flattening it into a plateau.
    double[] cap = new double[s + 1];
    cap[s] = 1.0;
    for (int j = s - 1; j >= 0; j--)
      cap[j] = DistNumerics.NextDown(cap[j + 1]);

    f = new double[s + 1];
    f[0] = 0.0;
    for (int j = 0; j < s - 1; j++)
    {
      double raw = f[j] + a[j] / total;
      f[j + 1] = Min(cap[j + 1], Max(DistNumerics.NextUp(f[j]), raw));
    }

    f[s] = 1.0;

    b = boundaries.ToArray();
    sources = surviving.ToArray();
    p = new double[s];
    d = new double[s];
    for (int j = 0; j < s; j++)
    {
      p[j] = f[j + 1] - f[j];
      d[j] = p[j] / (b[j + 1] - b[j]);
    }
  }

  /// <summary>Number of surviving cells in the reconstruction.</summary>
  public int CellCount => p.Length;

  /// <summary>Lower end of the support.</summary>
  public double SupportMin => b[0];

  /// <summary>Upper end of the support.</summary>
  public double SupportMax => b[b.Length - 1];

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
  public NeighborDistModel(Sample x)
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

  /// <summary>Measured queries against a built <see cref="NeighborDistModel"/>.</summary>
  public readonly struct MeasuredView(NeighborDistModel model)
  {
    private readonly NeighborDistModel model = model;

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

  private static double[] Guard(double[] values, bool assumeSorted)
  {
    Assertion.Validity(values, Subject.X);
    return DistSampleGuard.Prepare(values, assumeSorted);
  }

  /// <inheritdoc />
  /// <remarks>
  /// NaN first, for the same reason <see cref="LinearDistModel.QuantileAt"/> checks it: every
  /// branch below compares false against it, and the fallback would otherwise depend on how the
  /// platform casts a NaN cumulance to an index.
  /// </remarks>
  public double QuantileAt(double p)
  {
    if (double.IsNaN(p))
      return double.NaN;

    int s = this.p.Length;
    double value;
    if (p <= 0.0)
      value = b[0];
    else if (p >= 1.0)
      value = b[s];
    else
    {
      int j = DistNumerics.LowerBound(f, 1, s + 1, p) - 1;
      double t = DistNumerics.UnitRatio(p - f[j], this.p[j]);
      value = DistNumerics.Lerp(b[j], b[j + 1], t);
    }

    // Every branch hands back a value built from the sample's own boundaries -- the clamped ones
    // return one verbatim, and Lerp reproduces its endpoints exactly -- so a -0.0 among the
    // observations would otherwise reach the caller unnormalized.
    return value.NormalizeZero();
  }

  /// <inheritdoc />
  public double CumulativeAt(double x)
  {
    if (double.IsNaN(x))
      return double.NaN;

    int s = p.Length;
    if (x <= b[0])
      return 0.0;
    if (x >= b[s])
      return 1.0;

    int j = DistNumerics.UpperBound(b, x) - 1;
    double t = DistNumerics.UnitRatio(x - b[j], b[j + 1] - b[j]);
    return DistNumerics.Lerp(f[j], f[j + 1], t);
  }

  /// <inheritdoc />
  public double DensityAt(double x)
  {
    if (double.IsNaN(x))
      return double.NaN;

    int s = p.Length;
    if (x < b[0] || x > b[s])
      return 0.0;

    int j = x == b[s] ? s - 1 : DistNumerics.UpperBound(b, x) - 1;
    return p[j] / (b[j + 1] - b[j]);
  }

  /// <summary>
  /// The staircase this model is: cell boundaries, cumulative values, masses and heights.
  /// </summary>
  /// <remarks>
  /// Point evaluation answers what the density is at a value a caller picked; this answers what
  /// the density <em>is</em>, on the boundaries the model itself uses. Every height is the
  /// expression <see cref="DensityAt"/> evaluates inside the same cell, so the two agree bit for
  /// bit rather than approximately.
  /// </remarks>
  public NeighborLandscape GetLandscape() =>
    new(
      scaleIndex,
      radius,
      (double[])b.Clone(),
      (double[])f.Clone(),
      (double[])p.Clone(),
      (double[])d.Clone(),
      (int[])sources.Clone());
}
