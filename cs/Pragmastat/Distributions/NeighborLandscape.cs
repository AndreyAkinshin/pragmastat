namespace Pragmastat.Distributions;

/// <summary>
/// The staircase a <see cref="NeighborDistModel"/> is: cell boundaries, cumulative values, masses
/// and heights.
/// </summary>
/// <remarks>
/// <para>
/// Point evaluation answers what the density is at a value a caller picked; this answers what the
/// density <em>is</em>, on the boundaries the model itself uses. Drawing the second from the
/// first means guessing a grid and rebuilding the normalization on top of it, which is a second
/// reconstruction of the same object and drifts from this one silently.
/// </para>
/// <para>
/// Every array is a copy handed out by the model, so holding a landscape can never mutate it.
/// </para>
/// </remarks>
public sealed class NeighborLandscape
{
  internal NeighborLandscape(
    int scaleIndex,
    int radius,
    double[] boundaries,
    double[] cumulative,
    double[] masses,
    double[] densities,
    int[] sources)
  {
    ScaleIndex = scaleIndex;
    Radius = radius;
    Boundaries = boundaries;
    Cumulative = cumulative;
    Masses = masses;
    Densities = densities;
    Sources = sources;
  }

  /// <summary>Smoothing scale index k0 = ceil(sqrt(n)).</summary>
  public int ScaleIndex { get; }

  /// <summary>Window radius r = floor(k0 / 2).</summary>
  public int Radius { get; }

  /// <summary>Strictly increasing cell boundaries, length <c>CellCount + 1</c>.</summary>
  public IReadOnlyList<double> Boundaries { get; }

  /// <summary>Cumulative values at the boundaries, from 0 to exactly 1.</summary>
  public IReadOnlyList<double> Cumulative { get; }

  /// <summary>Strictly positive cell masses, length <c>CellCount</c>.</summary>
  public IReadOnlyList<double> Masses { get; }

  /// <summary>Cell heights, mass divided by width.</summary>
  public IReadOnlyList<double> Densities { get; }

  /// <summary>Index of the sorted observation each surviving cell was derived from.</summary>
  public IReadOnlyList<int> Sources { get; }

  /// <summary>Number of surviving cells.</summary>
  public int CellCount => Masses.Count;
}
