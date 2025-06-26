using System.Globalization;
using System.Text;
using JetBrains.Annotations;

namespace Pragmastat.Core.Metrology.Formatting;

public class SampleFormatter(
    MeasurementUnitFormatter measurementUnitFormatter,
    string defaultFormat = "G",
    string openBracket = "[",
    string closeBracket = "]",
    char separator = ','
)
{
    public static readonly SampleFormatter Default = new(MeasurementUnitFormatter.Default);

    public string Format(
        Sample sample,
        string? format = null,
        IFormatProvider? formatProvider = null,
        UnitPresentation? unitPresentation = null)
    {
        format ??= defaultFormat;
        formatProvider ??= CultureInfo.InvariantCulture;

        var builder = new StringBuilder();
        builder.Append(openBracket);
        for (int i = 0; i < sample.Values.Count; i++)
        {
            if (i != 0)
                builder.Append(separator);
            builder.Append(sample.Values[i].ToString(format, formatProvider));
        }
        builder.Append(closeBracket);
        builder.Append(measurementUnitFormatter.Format(sample.Unit));
        return builder.ToString();
    }

    [PublicAPI]
    public bool TryParse(string s, out Sample sample)
    {
        sample = new Sample(0);
        try
        {
            if (s.IndexOf(openBracket, StringComparison.Ordinal) != 0 || !s.Contains(closeBracket))
                return false;
            int openBracketIndex = s.IndexOf(openBracket, StringComparison.Ordinal);
            int closeBracketIndex = s.IndexOf(closeBracket, StringComparison.Ordinal);
            string main = s.Substring(openBracketIndex + 1, closeBracketIndex - openBracketIndex - 1);
            string[] valueStrings = main.Split(separator);
            double[] values = new double[valueStrings.Length];
            for (int i = 0; i < valueStrings.Length; i++)
            {
                string valueString = valueStrings[i];
                if (!double.TryParse(valueString, NumberStyles.Any, CultureInfo.InvariantCulture, out double value))
                    return false;
                values[i] = value;
            }

            string unitString = s.Substring(closeBracketIndex + 1);
            if (!measurementUnitFormatter.TryParse(unitString, out var unit))
                return false;

            sample = new Sample(values, unit);
            return true;
        }
        catch (Exception)
        {
            return false;
        }
    }

    public Sample Parse(string s) =>
        TryParse(s, out var sample) ? sample : throw new FormatException($"Unknown sample: {s}");
}