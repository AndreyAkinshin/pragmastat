using System.Globalization;
using Pragmastat.Internal;

namespace Pragmastat.Metrology;

public class MeasurementFormatter(
    MeasurementUnitFormatter measurementUnitFormatter,
    string defaultFormat = FormattingDefaults.Format)
{
    public static readonly MeasurementFormatter Default = new(MeasurementUnitFormatter.Default);

    public string Format(
        double nominalValue,
        MeasurementUnit unit,
        string? format = null,
        IFormatProvider? formatProvider = null,
        UnitPresentation? unitPresentation = null)
    {
        format ??= unit.DefaultFormat ?? defaultFormat;
        formatProvider ??= FormattingDefaults.FormatProvider;

        string nominalPart = nominalValue.ToString(format, formatProvider);
        string unitPart = measurementUnitFormatter.Format(unit, unitPresentation);
        return nominalPart + unitPart;
    }

    public string Format(
        Measurement measurement,
        string? format = null,
        IFormatProvider? formatProvider = null,
        UnitPresentation? unitPresentation = null)
    {
        return Format(measurement.NominalValue, measurement.Unit, format, formatProvider, unitPresentation);
    }

    public bool TryParse(string s, out Measurement value)
    {
        if (s.IsNotBlank())
        {
            foreach (var unit in measurementUnitFormatter.KnownUnits)
            {
                if (TryParse(s, unit, out value))
                    return true;
            }
        }

        value = new Measurement(0, NumberUnit.Instance);
        return false;
    }

    public Measurement Parse(string s) => TryParse(s, out var value)
        ? value
        : throw new FormatException($"Cannot parse {s} as a measurement");

    public static bool TryParse(string s, MeasurementUnit unit, out Measurement value)
    {
        if (TryParseBySuffix(unit.Abbreviation, out double nominalValue) ||
            TryParseBySuffix(unit.AbbreviationAscii, out nominalValue) ||
            TryParseBySuffix(unit.FullName, out nominalValue))
        {
            value = new Measurement(nominalValue, unit);
            return true;
        }

        value = Measurement.Zero();
        return false;

        bool TryParseBySuffix(string suffix, out double value)
        {
            const NumberStyles numberStyles = NumberStyles.Float;
            var formatProvider = CultureInfo.InvariantCulture;
            if (s.EndsWith(suffix, StringComparison.OrdinalIgnoreCase))
            {
                string number = s.Substring(0, s.Length - suffix.Length).Trim();
                return double.TryParse(number, numberStyles, formatProvider, out value);
            }
            value = 0;
            return false;
        }
    }
}