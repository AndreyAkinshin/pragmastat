using Pragmastat.Randomization;

namespace Pragmastat.Tests.Randomization;

public class RngInvarianceTests
{
  [Fact]
  public void ShufflePreservesMultiset()
  {
    foreach (int n in new[] { 1, 2, 5, 10, 100 })
    {
      var x = Enumerable.Range(0, n).Select(i => (double)i).ToList();
      var rng = new Rng(42);
      var shuffled = rng.Shuffle<double>(x);
      var sortedShuffled = shuffled.OrderBy(v => v).ToList();
      Assert.Equal(x, sortedShuffled);
    }
  }

  [Fact]
  public void SampleCorrectSize()
  {
    var x = Enumerable.Range(0, 10).Select(i => (double)i).ToList();
    foreach (int k in new[] { 1, 3, 5, 10, 15 })
    {
      var rng = new Rng(42);
      var sampled = rng.Sample<double>(x, k);
      Assert.Equal(Math.Min(k, x.Count), sampled.Count);
    }
  }

  [Fact]
  public void SampleElementsFromSource()
  {
    var x = Enumerable.Range(0, 10).Select(i => (double)i).ToList();
    var rng = new Rng(42);
    var sampled = rng.Sample<double>(x, 5);
    foreach (double elem in sampled)
      Assert.Contains(elem, x);
  }

  [Fact]
  public void SamplePreservesOrder()
  {
    var x = Enumerable.Range(0, 10).Select(i => (double)i).ToList();
    var rng = new Rng(42);
    var sampled = rng.Sample<double>(x, 5);
    for (int i = 1; i < sampled.Count; i++)
      Assert.True(sampled[i] > sampled[i - 1]);
  }

  [Fact]
  public void SampleNoDuplicates()
  {
    foreach (int n in new[] { 2, 3, 5, 10, 20 })
    {
      var source = Enumerable.Range(0, n).Select(i => (double)i).ToList();
      foreach (int k in new[] { 1, n / 2, n })
      {
        var rng = new Rng(42);
        var sampled = rng.Sample<double>(source, k);
        Assert.Equal(sampled.Count, sampled.Distinct().Count());
      }
    }
  }

  [Fact]
  public void ResampleNegativeKThrows()
  {
    var rng = new Rng(42);
    Assert.Throws<ArgumentOutOfRangeException>(() => rng.Resample<double>(new List<double> { 1, 2, 3 }, -1));
  }

  [Fact]
  public void ResampleElementsFromSource()
  {
    var x = Enumerable.Range(0, 5).Select(i => (double)i).ToList();
    var rng = new Rng(42);
    var resampled = rng.Resample<double>(x, 10);
    foreach (double elem in resampled)
      Assert.Contains(elem, x);
  }

  [Fact]
  public void ResampleK0Throws()
  {
    var rng = new Rng(42);
    Assert.Throws<ArgumentOutOfRangeException>(() => rng.Resample<double>(new List<double> { 1, 2, 3 }, 0));
  }

  [Fact]
  public void ShuffleEmptyThrows()
  {
    var rng = new Rng(42);
    Assert.Throws<ArgumentException>(() => rng.Shuffle<double>(new List<double>()));
  }

  [Fact]
  public void SampleK0Throws()
  {
    var rng = new Rng(42);
    Assert.Throws<ArgumentOutOfRangeException>(() => rng.Sample<double>(new List<double> { 1, 2, 3 }, 0));
  }

  [Fact]
  public void SampleEmptyThrows()
  {
    var rng = new Rng(42);
    Assert.Throws<ArgumentException>(() => rng.Sample<double>(new List<double>(), 1));
  }
}
