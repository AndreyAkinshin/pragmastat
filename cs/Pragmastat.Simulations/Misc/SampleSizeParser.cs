namespace Pragmastat.Simulations.Misc;

public static class SampleSizeParser
{
    /// <summary>
    /// Parses strings like `2,3,4,5,10..20,50..100` to int[]
    /// </summary>
    public static int[] ParseSampleSizes(string? sampleSizes)
    {
        if (string.IsNullOrWhiteSpace(sampleSizes))
            return [];
        var result = new List<int>();
        var parts = sampleSizes.Split(",", StringSplitOptions.RemoveEmptyEntries);
        foreach (var part in parts)
        {
            var trimmed = part.Trim();
            if (trimmed.Contains(".."))
            {
                string[] rangeParts = trimmed.Split("..", StringSplitOptions.RemoveEmptyEntries);
                if (rangeParts.Length == 2 &&
                    int.TryParse(rangeParts[0], out int start) &&
                    int.TryParse(rangeParts[1], out int end))
                {
                    if (start <= end)
                    {
                        for (int i = start; i <= end; i++)
                            result.Add(i);
                    }
                    else
                    {
                        for (int i = start; i >= end; i--)
                            result.Add(i);
                    }
                }
                // else: skip invalid range
            }
            else if (int.TryParse(trimmed, out int value))
            {
                result.Add(value);
            }
            // else: skip invalid part
        }
        return result.ToArray();
    }
}