using JetBrains.Annotations;

namespace Pragmastat.Exceptions;

public class WeightedSampleNotSupportedException : ArgumentException
{
  [PublicAPI]
  public WeightedSampleNotSupportedException()
  {
  }

  [PublicAPI]
  public WeightedSampleNotSupportedException(string message) : base(message)
  {
  }

  [PublicAPI]
  public WeightedSampleNotSupportedException(string message, Exception innerException) : base(message, innerException)
  {
  }

  [PublicAPI]
  public WeightedSampleNotSupportedException(string message, string paramName) : base(message, paramName)
  {
  }

  [PublicAPI]
  public WeightedSampleNotSupportedException(string message, string paramName, Exception innerException)
    : base(message, paramName, innerException)
  {
  }
}
