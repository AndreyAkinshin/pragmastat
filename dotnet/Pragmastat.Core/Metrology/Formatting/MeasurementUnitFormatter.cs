using Pragmastat.Core.Internal;
using Pragmastat.Core.Metrology.Units;

namespace Pragmastat.Core.Metrology.Formatting;

public class MeasurementUnitFormatter(IReadOnlyList<MeasurementUnit> knownUnits)
{
    public static readonly MeasurementUnitFormatter Default = new([NumberUnit.Instance]);

    public IReadOnlyList<MeasurementUnit> KnownUnits { get; } = knownUnits;

    public string Format(
        MeasurementUnit unit,
        UnitPresentation? unitPresentation = null)
    {
        unitPresentation ??= UnitPresentation.Default;
        if (!unitPresentation.IsVisible)
            return "";

        string abbreviation = unitPresentation.ForceAscii ? unit.AbbreviationAscii : unit.Abbreviation;
        string unitName = abbreviation.PadLeft(unitPresentation.MinUnitWidth);
        string gap = unitPresentation.Gap ? " " : "";
        return $"{gap}{unitName}";
    }


    public bool TryParse(string? s, out MeasurementUnit unit)
    {
        unit = NumberUnit.Instance;
        if (s is { Length: 0 })
            return true;

        if (s != null && s.IsNotBlank())
            foreach (var measurementUnit in KnownUnits)
            {
                if (measurementUnit.Abbreviation.EquationsIgnoreCase(s) ||
                    measurementUnit.AbbreviationAscii.EquationsIgnoreCase(s) ||
                    measurementUnit.FullName.EquationsIgnoreCase(s))
                {
                    unit = measurementUnit;
                    return true;
                }
            }

        return false;
    }

    public MeasurementUnit Parse(string s) =>
        TryParse(s, out var unit) ? unit : throw new FormatException($"Unknown unit: {s}");
}