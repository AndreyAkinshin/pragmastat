using JetBrains.Annotations;

namespace Pragmastat.ReferenceTests.Generator.Framework;

public class ReferenceTestCase<TInput, TOutput>(TInput input, TOutput output)
{
  [UsedImplicitly] public TInput Input { get; init; } = input;

  [UsedImplicitly] public TOutput Output { get; init; } = output;
}
