using System.Globalization;

namespace Pragmastat.Core.Internal;

internal static class FormattingDefaults
{
    public const string Format = "G";
    public static readonly IFormatProvider FormatProvider = CultureInfo.InvariantCulture;
}