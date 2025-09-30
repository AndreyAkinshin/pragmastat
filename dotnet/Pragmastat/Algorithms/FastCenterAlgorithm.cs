namespace Pragmastat.Algorithms;

internal static class FastCenterAlgorithm
{
    /// <summary>
    /// ACM Algorithm 616: fast computation of the Hodges-Lehmann location estimator
    /// </summary>
    /// <remarks>
    /// Computes the median of all pairwise averages (xi + xj)/2 efficiently.
    /// See: John F Monahan, "Algorithm 616: fast computation of the Hodges-Lehmann location estimator"
    /// (1984) DOI: 10.1145/1271.319414
    /// </remarks>
    /// <param name="values">A sorted sample of values</param>
    /// <param name="random">Random number generator</param>
    /// <param name="isSorted">If values are sorted</param>
    /// <returns>Exact center value (Hodges-Lehmann estimator)</returns>
    public static double Estimate(IReadOnlyList<double> values, Random? random = null, bool isSorted = false)
    {
        int n = values.Count;
        if (n == 1) return values[0];
        if (n == 2) return (values[0] + values[1]) / 2;
        random ??= new Random();
        if (!isSorted)
            values = values.OrderBy(x => x).ToList();

        // Calculate target median rank(s) among all pairwise sums
        long totalPairs = (long)n * (n + 1) / 2;
        long medianRankLow = (totalPairs + 1) / 2; // For odd totalPairs, this is the median
        long medianRankHigh =
            (totalPairs + 2) / 2; // For even totalPairs, average of ranks medianRankLow and medianRankHigh

        // Initialize search bounds for each row in the implicit matrix
        long[] leftBounds = new long[n];
        long[] rightBounds = new long[n];
        long[] partitionCounts = new long[n];

        for (int i = 0; i < n; i++)
        {
            leftBounds[i] = i + 1; // Row i can pair with columns [i+1..n] (1-based indexing)
            rightBounds[i] = n; // Initially, all columns are available
        }

        // Start with a good pivot: sum of middle elements (handles both odd and even n)
        double pivot = values[(n - 1) / 2] + values[n / 2];
        long activeSetSize = totalPairs;
        long previousCount = 0;

        while (true)
        {
            // === PARTITION STEP ===
            // Count pairwise sums less than current pivot
            long countBelowPivot = 0;
            long currentColumn = n;

            for (int row = 1; row <= n; row++)
            {
                partitionCounts[row - 1] = 0;

                // Move left from current column until we find sums < pivot
                // This exploits the sorted nature of the matrix
                while (currentColumn >= row && values[row - 1] + values[(int)currentColumn - 1] >= pivot)
                    currentColumn--;

                // Count elements in this row that are < pivot
                if (currentColumn >= row)
                {
                    long elementsBelow = currentColumn - row + 1;
                    partitionCounts[row - 1] = elementsBelow;
                    countBelowPivot += elementsBelow;
                }
            }

            // === CONVERGENCE CHECK ===
            // If no progress, we have ties - break them using midrange strategy
            if (countBelowPivot == previousCount)
            {
                double minActiveSum = double.MaxValue;
                double maxActiveSum = double.MinValue;

                // Find the range of sums still in the active search space
                for (int i = 0; i < n; i++)
                {
                    if (leftBounds[i] > rightBounds[i]) continue; // Skip empty rows

                    double rowValue = values[i];
                    double smallestInRow = values[(int)leftBounds[i] - 1] + rowValue;
                    double largestInRow = values[(int)rightBounds[i] - 1] + rowValue;

                    minActiveSum = Min(minActiveSum, smallestInRow);
                    maxActiveSum = Max(maxActiveSum, largestInRow);
                }

                pivot = (minActiveSum + maxActiveSum) / 2;
                if (pivot <= minActiveSum || pivot > maxActiveSum) pivot = maxActiveSum;

                // If all remaining values are identical, we're done
                if (minActiveSum == maxActiveSum || activeSetSize <= 2)
                    return pivot / 2;

                continue;
            }

            // === TARGET CHECK ===
            // Check if we've found the median rank(s)
            bool atTargetRank = countBelowPivot == medianRankLow || countBelowPivot == medianRankHigh - 1;
            if (atTargetRank)
            {
                // Find the boundary values: largest < pivot and smallest >= pivot
                double largestBelowPivot = double.MinValue;
                double smallestAtOrAbovePivot = double.MaxValue;

                for (int i = 1; i <= n; i++)
                {
                    long countInRow = partitionCounts[i - 1];
                    double rowValue = values[i - 1];
                    long totalInRow = n - i + 1;

                    // Find largest sum in this row that's < pivot
                    if (countInRow > 0)
                    {
                        long lastBelowIndex = i + countInRow - 1;
                        double lastBelowValue = rowValue + values[(int)lastBelowIndex - 1];
                        largestBelowPivot = Max(largestBelowPivot, lastBelowValue);
                    }

                    // Find smallest sum in this row that's >= pivot
                    if (countInRow < totalInRow)
                    {
                        long firstAtOrAboveIndex = i + countInRow;
                        double firstAtOrAboveValue = rowValue + values[(int)firstAtOrAboveIndex - 1];
                        smallestAtOrAbovePivot = Min(smallestAtOrAbovePivot, firstAtOrAboveValue);
                    }
                }

                // Calculate final result based on whether we have odd or even number of pairs
                if (medianRankLow < medianRankHigh)
                {
                    // Even total: average the two middle values
                    return (smallestAtOrAbovePivot + largestBelowPivot) / 4;
                }
                else
                {
                    // Odd total: return the single middle value
                    bool needLargest = countBelowPivot == medianRankLow;
                    return (needLargest ? largestBelowPivot : smallestAtOrAbovePivot) / 2;
                }
            }

            // === UPDATE BOUNDS ===
            // Narrow the search space based on partition result
            if (countBelowPivot < medianRankLow)
            {
                // Too few values below pivot - eliminate smaller values, search higher
                for (int i = 0; i < n; i++)
                    leftBounds[i] = i + partitionCounts[i] + 1;
            }
            else
            {
                // Too many values below pivot - eliminate larger values, search lower
                for (int i = 0; i < n; i++)
                    rightBounds[i] = i + partitionCounts[i];
            }

            // === PREPARE NEXT ITERATION ===
            previousCount = countBelowPivot;

            // Recalculate how many elements remain in the active search space
            activeSetSize = 0;
            for (int i = 0; i < n; i++)
            {
                long rowSize = rightBounds[i] - leftBounds[i] + 1;
                activeSetSize += Max(0, rowSize);
            }

            // Choose next pivot based on remaining active set size
            if (activeSetSize > 2)
            {
                // Use randomized row median strategy for efficiency
                // Handle large activeSetSize by using double precision for random selection
                double randomFraction = random.NextDouble();
                long targetIndex = (long)(randomFraction * activeSetSize);
                int selectedRow = 0;

                // Find which row contains the target index
                long cumulativeSize = 0;
                for (int i = 0; i < n; i++)
                {
                    long rowSize = Max(0, rightBounds[i] - leftBounds[i] + 1);
                    if (targetIndex < cumulativeSize + rowSize)
                    {
                        selectedRow = i;
                        break;
                    }

                    cumulativeSize += rowSize;
                }

                // Use median element of the selected row as pivot
                long medianColumnInRow = (leftBounds[selectedRow] + rightBounds[selectedRow]) / 2;
                pivot = values[selectedRow] + values[(int)medianColumnInRow - 1];
            }
            else
            {
                // Few elements remain - use midrange strategy
                double minRemainingSum = double.MaxValue;
                double maxRemainingSum = double.MinValue;

                for (int i = 0; i < n; i++)
                {
                    if (leftBounds[i] > rightBounds[i]) continue; // Skip empty rows

                    double rowValue = values[i];
                    double minInRow = values[(int)leftBounds[i] - 1] + rowValue;
                    double maxInRow = values[(int)rightBounds[i] - 1] + rowValue;

                    minRemainingSum = Min(minRemainingSum, minInRow);
                    maxRemainingSum = Max(maxRemainingSum, maxInRow);
                }

                pivot = (minRemainingSum + maxRemainingSum) / 2;
                if (pivot <= minRemainingSum || pivot > maxRemainingSum)
                    pivot = maxRemainingSum;

                if (minRemainingSum == maxRemainingSum)
                    return pivot / 2;
            }
        }
    }
}