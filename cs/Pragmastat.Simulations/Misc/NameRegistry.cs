using Pragmastat.Distributions;
using Pragmastat.Estimators;
using Pragmastat.Extended.Estimators;
using Pragmastat.Internal;

namespace Pragmastat.Simulations.Misc;

public record Named<T>(string Name, T Value);

public class NameRegistry<T>
{
  private readonly OrderedDictionary<string, Named<T>> dictionary = new(StringComparer.OrdinalIgnoreCase);

  public NameRegistry<T> Add(string name, T value)
  {
    if (dictionary.ContainsKey(name))
      throw new ArgumentException("Entry with such name already exist", nameof(name));
    dictionary[name] = new Named<T>(name, value);
    return this;
  }

  public IReadOnlyList<Named<T>> ParseCommandSeparatedNames(string? input)
  {
    var entities = new List<Named<T>>();

    string[] names = (input ?? "").Split(",", StringSplitOptions.RemoveEmptyEntries);
    if (names.IsEmpty())
      return entities;

    foreach (string name in names)
    {
      if (dictionary.TryGetValue(name, out var entry))
        entities.Add(entry);
      else
        throw new ArgumentException($"Unknown name for {typeof(T).Name}: {name}");
    }

    return entities;
  }
}

public static class Registries
{
  public static readonly NameRegistry<IContinuousDistribution> Distributions =
    new NameRegistry<IContinuousDistribution>()
      .Add("Additive", Additive.Standard)
      .Add("Multiplic", Multiplic.Standard)
      .Add("Exp", Exp.Standard)
      .Add("Power", Power.Standard)
      .Add("Uniform", Uniform.Standard);

  public static readonly NameRegistry<IOneSampleEstimator> AverageEstimators =
    new NameRegistry<IOneSampleEstimator>()
      .Add("Mean", MeanEstimator.Instance)
      .Add("Median", MedianEstimator.Instance)
      .Add("Center", CenterEstimator.Instance);

  public static readonly NameRegistry<IOneSampleEstimator> DispersionEstimators =
    new NameRegistry<IOneSampleEstimator>()
      .Add("StdDev", StandardDeviationEstimator.Corrected)
      .Add("MAD", MedianAbsoluteDeviationEstimator.Instance)
      .Add("Spread", SpreadEstimator.Instance);
}
