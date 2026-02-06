using System.Text.Json.Serialization;
using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework;

public class ReferenceTestCase<TInput, TOutput>(TInput input, TOutput output)
{
  [UsedImplicitly] public TInput Input { get; init; } = input;

  [UsedImplicitly] public TOutput Output { get; init; } = output;
}

public class ExpectedError
{
  [UsedImplicitly] public string Id { get; init; } = null!;
  [UsedImplicitly] public string Subject { get; init; } = null!;
}

public class ErrorTestCase<TInput>
{
  [UsedImplicitly] public TInput Input { get; init; } = default!;

  [UsedImplicitly]
  [JsonPropertyName("expected_error")]
  public ExpectedError ExpectedError { get; init; } = null!;
}
