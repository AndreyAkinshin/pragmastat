namespace Pragmastat.Algorithms;

internal static class FastSpreadAlgorithm
{
  /// <summary>
  /// Shamos "Spread".  Expected O(n log n) time, O(n) extra space. Exact.
  /// </summary>
  public static double Estimate(IReadOnlyList<double> values, Random? random = null, bool isSorted = false)
  {
    int n = values.Count;
    if (n <= 1) return 0;
    if (n == 2) return Abs(values[1] - values[0]);
    random ??= new Random();

    // Prepare a sorted working copy.
    double[] a = isSorted ? CopySorted(values) : EnsureSorted(values);

    // Total number of pairwise differences with i < j
    long N = (long)n * (n - 1) / 2;
    long kLow = (N + 1) / 2; // 1-based rank of lower middle
    long kHigh = (N + 2) / 2; // 1-based rank of upper middle

    // Per-row active bounds over columns j (0-based indices).
    // Row i allows j in [i+1, n-1] initially.
    int[] L = new int[n];
    int[] R = new int[n];
    long[] rowCounts = new long[n]; // # of elements in row i that are < pivot (for current partition)

    for (int i = 0; i < n; i++)
    {
      L[i] = Min(i + 1, n); // n means empty
      R[i] = n - 1; // inclusive
      if (L[i] > R[i])
      {
        L[i] = 1;
        R[i] = 0;
      } // mark empty
    }

    // A reasonable initial pivot: a central gap
    double pivot = a[n / 2] - a[(n - 1) / 2];

    long prevCountBelow = -1;

    while (true)
    {
      // === PARTITION: count how many differences are < pivot; also track boundary neighbors ===
      long countBelow = 0;
      double largestBelow = double.NegativeInfinity; // max difference < pivot
      double smallestAtOrAbove = double.PositiveInfinity; // min difference >= pivot

      int j = 1; // global two-pointer (non-decreasing across rows)
      for (int i = 0; i < n - 1; i++)
      {
        if (j < i + 1) j = i + 1;
        while (j < n && a[j] - a[i] < pivot) j++;

        long cntRow = j - (i + 1);
        if (cntRow < 0) cntRow = 0;
        rowCounts[i] = cntRow;
        countBelow += cntRow;

        // boundary elements for this row
        if (cntRow > 0)
        {
          // last < pivot in this row is (j-1)
          double candBelow = a[j - 1] - a[i];
          if (candBelow > largestBelow) largestBelow = candBelow;
        }

        if (j < n)
        {
          double candAtOrAbove = a[j] - a[i];
          if (candAtOrAbove < smallestAtOrAbove) smallestAtOrAbove = candAtOrAbove;
        }
      }

      // === TARGET CHECK ===
      // If we've split exactly at the middle, we can return using the boundaries we just found.
      bool atTarget =
        (countBelow == kLow) || // lower middle is the largest < pivot
        (countBelow == (kHigh - 1)); // upper middle is the smallest >= pivot

      if (atTarget)
      {
        if (kLow < kHigh)
        {
          // Even N: average the two central order stats.
          return 0.5 * (largestBelow + smallestAtOrAbove);
        }
        else
        {
          // Odd N: pick the single middle depending on which side we hit.
          bool needLargest = (countBelow == kLow);
          return needLargest ? largestBelow : smallestAtOrAbove;
        }
      }

      // === STALL HANDLING (ties / no progress) ===
      if (countBelow == prevCountBelow)
      {
        // Compute min/max remaining difference in the ACTIVE set and pivot to their midrange.
        double minActive = double.PositiveInfinity;
        double maxActive = double.NegativeInfinity;
        long active = 0;

        for (int i = 0; i < n - 1; i++)
        {
          int Li = L[i], Ri = R[i];
          if (Li > Ri) continue;

          double rowMin = a[Li] - a[i];
          double rowMax = a[Ri] - a[i];
          if (rowMin < minActive) minActive = rowMin;
          if (rowMax > maxActive) maxActive = rowMax;
          active += (Ri - Li + 1);
        }

        if (active <= 0)
        {
          // No active candidates left: the only consistent answer is the boundary implied by counts.
          // Fall back to neighbors from this partition.
          if (kLow < kHigh) return 0.5 * (largestBelow + smallestAtOrAbove);
          return (countBelow >= kLow) ? largestBelow : smallestAtOrAbove;
        }

        if (maxActive <= minActive) return minActive; // all remaining equal

        double mid = 0.5 * (minActive + maxActive);
        pivot = (mid > minActive && mid <= maxActive) ? mid : maxActive;
        prevCountBelow = countBelow;
        continue;
      }

      // === SHRINK ACTIVE WINDOW ===
// --- SHRINK ACTIVE WINDOW (fixed) ---
      if (countBelow < kLow)
      {
        // Need larger differences: discard all strictly below pivot.
        for (int i = 0; i < n - 1; i++)
        {
          // First j with a[j] - a[i] >= pivot is j = i + 1 + cntRow (may be n => empty row)
          int newL = i + 1 + (int)rowCounts[i];
          if (newL > L[i]) L[i] = newL; // do NOT clamp; allow L[i] == n to mean empty
          if (L[i] > R[i])
          {
            L[i] = 1;
            R[i] = 0;
          } // mark empty
        }
      }
      else
      {
        // Too many below: keep only those strictly below pivot.
        for (int i = 0; i < n - 1; i++)
        {
          // Last j with a[j] - a[i] < pivot is j = i + cntRow  (not cntRow-1!)
          int newR = i + (int)rowCounts[i];
          if (newR < R[i]) R[i] = newR; // shrink downward to the true last-below
          if (R[i] < i + 1)
          {
            L[i] = 1;
            R[i] = 0;
          } // empty row if none remain
        }
      }

      prevCountBelow = countBelow;

      // === CHOOSE NEXT PIVOT FROM ACTIVE SET (weighted random row, then row median) ===
      long activeSize = 0;
      for (int i = 0; i < n - 1; i++)
      {
        if (L[i] <= R[i]) activeSize += (R[i] - L[i] + 1);
      }

      if (activeSize <= 2)
      {
        // Few candidates left: return midrange of remaining exactly.
        double minRem = double.PositiveInfinity, maxRem = double.NegativeInfinity;
        for (int i = 0; i < n - 1; i++)
        {
          if (L[i] > R[i]) continue;
          double lo = a[L[i]] - a[i];
          double hi = a[R[i]] - a[i];
          if (lo < minRem) minRem = lo;
          if (hi > maxRem) maxRem = hi;
        }

        if (activeSize <= 0) // safety net; fall back to boundary from last partition
        {
          if (kLow < kHigh) return 0.5 * (largestBelow + smallestAtOrAbove);
          return (countBelow >= kLow) ? largestBelow : smallestAtOrAbove;
        }

        if (kLow < kHigh) return 0.5 * (minRem + maxRem);
        return (Abs((kLow - 1) - countBelow) <= Abs(countBelow - kLow)) ? minRem : maxRem;
      }
      else
      {
        long t = NextIndex(random, activeSize); // 0..activeSize-1
        long acc = 0;
        int row = 0;
        for (; row < n - 1; row++)
        {
          if (L[row] > R[row]) continue;
          long size = R[row] - L[row] + 1;
          if (t < acc + size) break;
          acc += size;
        }

        // Median column of the selected row
        int col = (L[row] + R[row]) >> 1;
        pivot = a[col] - a[row];
      }
    }
  }
  // --- Helpers ---

  private static double[] CopySorted(IReadOnlyList<double> values)
  {
    var a = new double[values.Count];
    for (int i = 0; i < a.Length; i++)
    {
      double v = values[i];
      if (double.IsNaN(v)) throw new ArgumentException("NaN not allowed.", nameof(values));
      a[i] = v;
    }

    Array.Sort(a);
    return a;
  }

  private static double[] EnsureSorted(IReadOnlyList<double> values)
  {
    // Trust caller; still copy to array for fast indexed access.
    var a = new double[values.Count];
    for (int i = 0; i < a.Length; i++)
    {
      double v = values[i];
      if (double.IsNaN(v)) throw new ArgumentException("NaN not allowed.", nameof(values));
      a[i] = v;
    }

    return a;
  }

  private static long NextIndex(Random rng, long limitExclusive)
  {
    // Uniform 0..limitExclusive-1 even for large ranges.
    // Use rejection sampling for correctness.
    ulong uLimit = (ulong)limitExclusive;
    if (uLimit <= int.MaxValue)
    {
      return rng.Next((int)uLimit);
    }

    while (true)
    {
      ulong u = ((ulong)(uint)rng.Next() << 32) | (uint)rng.Next();
      ulong r = u % uLimit;
      if (u - r <= ulong.MaxValue - (ulong.MaxValue % uLimit)) return (long)r;
    }
  }
}
