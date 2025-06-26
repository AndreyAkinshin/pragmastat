namespace Entry.Core;

public class Placeholder(string name)
{
    public static readonly Placeholder Start = new("Start");

    public string ReplaceBy(string originalContent, string replacement)
    {
        var comment = $"<!-- PLACEHOLDER {name} -->";
        return originalContent.Replace(comment, replacement);
    }
}