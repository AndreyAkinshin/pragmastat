using System.Text.Json;
using Pragmastat.Distributions;
using Pragmastat.Internal;
using Pragmastat.Randomization;

namespace Pragmastat.Tests.Distributions;

public class DistributionTests
{
  private static readonly string TestsDir = Path.Combine(SourceRepositoryLocator.RepositoryRoot, "tests", "distributions");
  private const double Tolerance = 1e-12;

  [Fact]
  public void UniformTests()
  {
    foreach (string filePath in GetFiles("uniform"))
    {
      using JsonDocument doc = JsonDocument.Parse(File.ReadAllText(filePath));
      JsonElement input = doc.RootElement.GetProperty("input");
      JsonElement output = doc.RootElement.GetProperty("output");

      long seed = input.GetProperty("seed").GetInt64();
      double min = input.GetProperty("min").GetDouble();
      double max = input.GetProperty("max").GetDouble();

      var rng = new Rng(seed);
      var dist = new Uniform(min, max);

      for (int i = 0; i < output.GetArrayLength(); i++)
      {
        double actual = dist.Sample(rng);
        double expected = output[i].GetDouble();
        Assert.True(Math.Abs(actual - expected) < Tolerance,
          $"File: {Path.GetFileName(filePath)}, index {i}: expected {expected}, got {actual}");
      }
    }
  }

  [Fact]
  public void AdditiveTests()
  {
    foreach (string filePath in GetFiles("additive"))
    {
      using JsonDocument doc = JsonDocument.Parse(File.ReadAllText(filePath));
      JsonElement input = doc.RootElement.GetProperty("input");
      JsonElement output = doc.RootElement.GetProperty("output");

      long seed = input.GetProperty("seed").GetInt64();
      double mean = input.GetProperty("mean").GetDouble();
      double stdDev = input.GetProperty("stdDev").GetDouble();

      var rng = new Rng(seed);
      var dist = new Additive(mean, stdDev);

      for (int i = 0; i < output.GetArrayLength(); i++)
      {
        double actual = dist.Sample(rng);
        double expected = output[i].GetDouble();
        Assert.True(Math.Abs(actual - expected) < Tolerance,
          $"File: {Path.GetFileName(filePath)}, index {i}: expected {expected}, got {actual}");
      }
    }
  }

  [Fact]
  public void MultiplicTests()
  {
    foreach (string filePath in GetFiles("multiplic"))
    {
      using JsonDocument doc = JsonDocument.Parse(File.ReadAllText(filePath));
      JsonElement input = doc.RootElement.GetProperty("input");
      JsonElement output = doc.RootElement.GetProperty("output");

      long seed = input.GetProperty("seed").GetInt64();
      double logMean = input.GetProperty("logMean").GetDouble();
      double logStdDev = input.GetProperty("logStdDev").GetDouble();

      var rng = new Rng(seed);
      var dist = new Multiplic(logMean, logStdDev);

      for (int i = 0; i < output.GetArrayLength(); i++)
      {
        double actual = dist.Sample(rng);
        double expected = output[i].GetDouble();
        Assert.True(Math.Abs(actual - expected) < Tolerance,
          $"File: {Path.GetFileName(filePath)}, index {i}: expected {expected}, got {actual}");
      }
    }
  }

  [Fact]
  public void ExpTests()
  {
    foreach (string filePath in GetFiles("exp"))
    {
      using JsonDocument doc = JsonDocument.Parse(File.ReadAllText(filePath));
      JsonElement input = doc.RootElement.GetProperty("input");
      JsonElement output = doc.RootElement.GetProperty("output");

      long seed = input.GetProperty("seed").GetInt64();
      double rate = input.GetProperty("rate").GetDouble();

      var rng = new Rng(seed);
      var dist = new Exp(rate);

      for (int i = 0; i < output.GetArrayLength(); i++)
      {
        double actual = dist.Sample(rng);
        double expected = output[i].GetDouble();
        Assert.True(Math.Abs(actual - expected) < Tolerance,
          $"File: {Path.GetFileName(filePath)}, index {i}: expected {expected}, got {actual}");
      }
    }
  }

  [Fact]
  public void PowerTests()
  {
    foreach (string filePath in GetFiles("power"))
    {
      using JsonDocument doc = JsonDocument.Parse(File.ReadAllText(filePath));
      JsonElement input = doc.RootElement.GetProperty("input");
      JsonElement output = doc.RootElement.GetProperty("output");

      long seed = input.GetProperty("seed").GetInt64();
      double min = input.GetProperty("min").GetDouble();
      double shape = input.GetProperty("shape").GetDouble();

      var rng = new Rng(seed);
      var dist = new Power(min, shape);

      for (int i = 0; i < output.GetArrayLength(); i++)
      {
        double actual = dist.Sample(rng);
        double expected = output[i].GetDouble();
        Assert.True(Math.Abs(actual - expected) < Tolerance,
          $"File: {Path.GetFileName(filePath)}, index {i}: expected {expected}, got {actual}");
      }
    }
  }

  private static string[] GetFiles(string distributionName)
  {
    string dir = Path.Combine(TestsDir, distributionName);
    var files = Directory.GetFiles(dir, "*.json");
    Assert.True(files.Length > 0, $"No {distributionName} distribution test files found");
    return files;
  }
}
