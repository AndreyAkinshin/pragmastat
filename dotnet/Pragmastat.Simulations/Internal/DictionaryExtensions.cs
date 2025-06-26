namespace Pragmastat.Simulations.Internal;

internal static class DictionaryExtensions
{
    public static string GetOriginalKey<TValue>(this Dictionary<string, TValue> dict, string lookupKey)
    {
        foreach (string key in dict.Keys)
        {
            if (dict.Comparer.Equals(key, lookupKey))
                return key;
        }
        throw new KeyNotFoundException($"Unknown key: {lookupKey}");
    }
}