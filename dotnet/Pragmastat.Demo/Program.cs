using Pragmastat.Core;
using static System.Console;

namespace Pragmastat.Demo;

class Program
{
    static void Main()
    {
        var x = new Sample(1, 2, 3, 4, 5, 6, 273);
        WriteLine(x.Center()); // 4
        WriteLine(x.Spread()); // 3
        WriteLine(x.Volatility()); // 0.75
        WriteLine(x.Precision()); // 2.2677868380553634

        WriteLine(Toolkit.MedShift(x, x - 10)); // 10
        WriteLine(Toolkit.MedRatio(x, x / 10)); // 10

        x = new Sample(-3, -2, -1, 0, 1, 2, 3);
        WriteLine(Toolkit.MedDisparity(x, x * 10)); // 0
        WriteLine(Toolkit.MedDisparity(x, x - 10)); // 5
        WriteLine(Toolkit.MedDisparity(x * 10, x * 10 - 100)); // 5
    }
}