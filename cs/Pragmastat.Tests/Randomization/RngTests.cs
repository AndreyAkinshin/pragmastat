using System.Text.Json;
using Pragmastat.Internal;
using Pragmastat.Randomization;

namespace Pragmastat.Tests.Randomization;

public class RngTests
{
  private static readonly string TestsDir = Path.Combine(SourceRepositoryLocator.RepositoryRoot, "tests");

  [Fact]
  public void UniformSeedTests()
  {
    string rngDir = Path.Combine(TestsDir, "rng");
    var files = Directory.GetFiles(rngDir, "uniform-seed-*.json");
    Assert.True(files.Length > 0, "No uniform seed test files found");

    foreach (string filePath in files)
    {
      string json = File.ReadAllText(filePath);
      using JsonDocument doc = JsonDocument.Parse(json);
      JsonElement root = doc.RootElement;

      long seed = root.GetProperty("input").GetProperty("seed").GetInt64();
      int count = root.GetProperty("input").GetProperty("count").GetInt32();
      JsonElement output = root.GetProperty("output");

      var rng = new Rng(seed);
      Assert.Equal(count, output.GetArrayLength());
      for (int i = 0; i < count; i++)
      {
        double actual = rng.Uniform();
        double expected = output[i].GetDouble();
        Assert.True(Math.Abs(actual - expected) < 1e-15,
            $"File: {Path.GetFileName(filePath)}, index {i}: expected {expected}, got {actual}");
      }
    }
  }

  [Fact]
  public void UniformIntTests()
  {
    string rngDir = Path.Combine(TestsDir, "rng");
    var files = Directory.GetFiles(rngDir, "uniform-int-*.json");
    Assert.True(files.Length > 0, "No uniform int test files found");

    foreach (string filePath in files)
    {
      string json = File.ReadAllText(filePath);
      using JsonDocument doc = JsonDocument.Parse(json);
      JsonElement root = doc.RootElement;

      long seed = root.GetProperty("input").GetProperty("seed").GetInt64();
      long min = root.GetProperty("input").GetProperty("min").GetInt64();
      long max = root.GetProperty("input").GetProperty("max").GetInt64();
      int count = root.GetProperty("input").GetProperty("count").GetInt32();
      JsonElement output = root.GetProperty("output");

      var rng = new Rng(seed);
      Assert.Equal(count, output.GetArrayLength());
      for (int i = 0; i < count; i++)
      {
        long actual = rng.UniformInt(min, max);
        long expected = output[i].GetInt64();
        Assert.Equal(expected, actual);
      }
    }
  }

  [Fact]
  public void StringSeedTests()
  {
    string rngDir = Path.Combine(TestsDir, "rng");
    var files = Directory.GetFiles(rngDir, "uniform-string-*.json");
    Assert.True(files.Length > 0, "No string seed test files found");

    foreach (string filePath in files)
    {
      string json = File.ReadAllText(filePath);
      using JsonDocument doc = JsonDocument.Parse(json);
      JsonElement root = doc.RootElement;

      string seed = root.GetProperty("input").GetProperty("seed").GetString()!;
      int count = root.GetProperty("input").GetProperty("count").GetInt32();
      JsonElement output = root.GetProperty("output");

      var rng = new Rng(seed);
      Assert.Equal(count, output.GetArrayLength());
      for (int i = 0; i < count; i++)
      {
        double actual = rng.Uniform();
        double expected = output[i].GetDouble();
        Assert.True(Math.Abs(actual - expected) < 1e-15,
            $"File: {Path.GetFileName(filePath)}, index {i}: expected {expected}, got {actual}");
      }
    }
  }

  [Fact]
  public void ShuffleTests()
  {
    string shuffleDir = Path.Combine(TestsDir, "shuffle");
    var files = Directory.GetFiles(shuffleDir, "*.json");
    Assert.True(files.Length > 0, "No shuffle test files found");

    foreach (string filePath in files)
    {
      string json = File.ReadAllText(filePath);
      using JsonDocument doc = JsonDocument.Parse(json);
      JsonElement root = doc.RootElement;

      long seed = root.GetProperty("input").GetProperty("seed").GetInt64();
      JsonElement x = root.GetProperty("input").GetProperty("x");
      JsonElement output = root.GetProperty("output");

      var input = new List<double>();
      foreach (JsonElement e in x.EnumerateArray())
        input.Add(e.GetDouble());

      var rng = new Rng(seed);
      List<double> actual = rng.Shuffle(input);

      Assert.Equal(output.GetArrayLength(), actual.Count);
      int i = 0;
      foreach (JsonElement e in output.EnumerateArray())
      {
        double expected = e.GetDouble();
        Assert.True(Math.Abs(actual[i] - expected) < 1e-15,
            $"File: {Path.GetFileName(filePath)}, index {i}: expected {expected}, got {actual[i]}");
        i++;
      }
    }
  }

  [Fact]
  public void SampleTests()
  {
    string sampleDir = Path.Combine(TestsDir, "sample");
    var files = Directory.GetFiles(sampleDir, "*.json");
    Assert.True(files.Length > 0, "No sample test files found");

    foreach (string filePath in files)
    {
      string json = File.ReadAllText(filePath);
      using JsonDocument doc = JsonDocument.Parse(json);
      JsonElement root = doc.RootElement;

      long seed = root.GetProperty("input").GetProperty("seed").GetInt64();
      JsonElement x = root.GetProperty("input").GetProperty("x");
      int k = root.GetProperty("input").GetProperty("k").GetInt32();
      JsonElement output = root.GetProperty("output");

      var input = new List<double>();
      foreach (JsonElement e in x.EnumerateArray())
        input.Add(e.GetDouble());

      var rng = new Rng(seed);
      List<double> actual = rng.Sample(input, k);

      Assert.Equal(output.GetArrayLength(), actual.Count);
      int i = 0;
      foreach (JsonElement e in output.EnumerateArray())
      {
        double expected = e.GetDouble();
        Assert.True(Math.Abs(actual[i] - expected) < 1e-15,
            $"File: {Path.GetFileName(filePath)}, index {i}: expected {expected}, got {actual[i]}");
        i++;
      }
    }
  }

  [Fact]
  public void SampleNegativeKThrows()
  {
    var rng = new Rng(42);
    var data = new List<double> { 1, 2, 3 };
    Assert.Throws<ArgumentOutOfRangeException>(() => rng.Sample(data, -1));
  }
}
