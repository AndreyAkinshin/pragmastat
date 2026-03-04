using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat;

public sealed class Threshold
{
  public Metric Metric { get; }
  public Measurement Value { get; }
  public Probability Misrate { get; }

  public Threshold(Metric metric, Measurement value, Probability misrate)
  {
    if (misrate.Value <= 0 || misrate.Value > 1)
      throw AssumptionException.Domain(Subject.Misrate);
    if (!value.NominalValue.IsFinite())
      throw new ArgumentOutOfRangeException(nameof(value), "threshold value must be finite");

    Metric = metric;
    Value = value;
    Misrate = misrate;
  }
}
