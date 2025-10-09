using System.Globalization;

namespace Pragmastat;

public readonly struct Interval
{
  public static Interval PositiveInfinity = Of(double.PositiveInfinity, double.PositiveInfinity);
  public static Interval NegativeInfinity = Of(double.NegativeInfinity, double.NegativeInfinity);
  public static Interval Zero = Of(0, 0);
  public static Interval NaN = Of(double.NaN, double.NaN);

  private const string DefaultFormat = "F2";

  public double Left { get; }
  public double Right { get; }

  public double Middle() => (Left + Right) / 2;
  public double Width() => Right - Left;

  private Interval(double left, double right)
  {
    Left = left;
    Right = right;
  }

  public static Interval Of(double left, double right) => new Interval(left, right);

  public bool IsInside(Interval outerRange)
  {
    return outerRange.Left <= Left && Right <= outerRange.Right;
  }

  public bool ContainsInclusive(double value) => Left <= value && value <= Right;

  public string ToString(CultureInfo? cultureInfo, string? format = "F2")
  {
    cultureInfo ??= CultureInfo.InvariantCulture;
    format ??= DefaultFormat;
    return $"[{Left.ToString(format, cultureInfo)};{Right.ToString(format, cultureInfo)}]";
  }

  public string ToString(string? format) => ToString(null, format);

  public override string ToString() => ToString(null, null);
}
