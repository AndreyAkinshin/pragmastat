using Pragmastat.Core;
using static System.Console;

namespace Pragmastat.Demo;

class Program
{
    static void Main()
    {
        var x = new Sample(0, 2, 4, 6, 8);
        WriteLine(x.Center()); // 4
        WriteLine((x + 10).Center()); // 14
        WriteLine((x * 3).Center()); // 12

        WriteLine(x.Spread()); // 4
        WriteLine((x + 10).Spread()); // 4
        WriteLine((x * 2).Spread()); // 8

        WriteLine(x.RelSpread()); // 1
        WriteLine((x * 5).RelSpread()); // 1

        var y = new Sample(10, 12, 14, 16, 18);
        WriteLine(x.Shift(y)); // -10
        WriteLine(x.Shift(x)); // 0
        WriteLine((x + 7).Shift(y + 3)); // -6
        WriteLine((x * 2).Shift(y * 2)); // -20
        WriteLine(y.Shift(x)); // 10

        x = new Sample(1, 2, 4, 8, 16);
        y = new Sample(2, 4, 8, 16, 32);
        WriteLine(x.Ratio(y)); // 0.5
        WriteLine(x.Ratio(x)); // 1
        WriteLine((x * 2).Ratio(y * 5)); // 0.2

        x = new Sample(0, 3, 6, 9, 12);
        y = new Sample(0, 2, 4, 6, 8);
        WriteLine(x.Spread()); // 6
        WriteLine(y.Spread()); // 4

        WriteLine(x.AvgSpread(y)); // 5
        WriteLine(x.AvgSpread(x)); // 6
        WriteLine((x * 2).AvgSpread(x * 3)); // 15
        WriteLine(y.AvgSpread(x)); // 5
        WriteLine((x * 2).AvgSpread(y * 2)); // 10

        WriteLine(x.Shift(y)); // 2
        WriteLine(x.AvgSpread(y)); // 5

        WriteLine(x.Disparity(y)); // 0.4
        WriteLine((x + 5).Disparity(y + 5)); // 0.4
        WriteLine((x * 2).Disparity(y * 2)); // 0.4
        WriteLine(y.Disparity(x)); // -0.4
    }
}