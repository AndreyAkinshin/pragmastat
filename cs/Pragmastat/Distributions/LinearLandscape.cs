namespace Pragmastat.Distributions;

/// <summary>
/// The staircase a <see cref="LinearDistModel"/> is: cell boundaries, cumulative values and
/// heights.
/// </summary>
/// <remarks>
/// The model interpolates the order statistics linearly, so its density is a staircase: one cell
/// between each pair of adjacent observations, each carrying the same mass <c>1/(n-1)</c> and
/// therefore a height that is that mass over its width. Every array is a copy handed out by the
/// model, so holding a landscape can never mutate it.
/// </remarks>
public sealed class LinearLandscape
{
  internal LinearLandscape(double[] boundaries, double[] cumulative, double[] densities)
  {
    Boundaries = boundaries;
    Cumulative = cumulative;
    Densities = densities;
  }

  /// <summary>The sorted observations, which are the cell boundaries; length CellCount + 1.</summary>
  public IReadOnlyList<double> Boundaries { get; }

  /// <summary>Cumulative values at the boundaries, from 0 to exactly 1.</summary>
  public IReadOnlyList<double> Cumulative { get; }

  /// <summary>Cell heights, mass divided by width; length CellCount.</summary>
  public IReadOnlyList<double> Densities { get; }

  /// <summary>Number of cells, one fewer than the number of observations.</summary>
  public int CellCount => Densities.Count;
}
