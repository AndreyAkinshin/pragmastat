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
        WriteLine(x.RelSpread()); // 0.75

        WriteLine(Toolkit.Shift(x, x - 10)); // 10
        WriteLine(Toolkit.Ratio(x, x / 10)); // 10

        x = new Sample(-3, -2, -1, 0, 1, 2, 3);
        WriteLine(Toolkit.Disparity(x, x * 10)); // 0
        WriteLine(Toolkit.Disparity(x, x - 10)); // 5
        WriteLine(Toolkit.Disparity(x * 10, x * 10 - 100)); // 5
    }
}