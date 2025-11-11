using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework;

public class ReferenceTestCaseInputBuilder<TInput>
{
  private readonly Dictionary<string, TInput> inputs = new();
  public IReadOnlyDictionary<string, TInput> Build() => inputs;

  [PublicAPI]
  public ReferenceTestCaseInputBuilder<TInput> Add(string name, TInput input)
  {
    if (!inputs.TryAdd(name, input))
      throw new ArgumentException($"Input with name '{name}' already exists.", nameof(name));
    return this;
  }
}
