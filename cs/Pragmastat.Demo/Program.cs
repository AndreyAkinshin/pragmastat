using static System.Console;
using Pragmastat.Distributions;
using Pragmastat.Randomization;

namespace Pragmastat.Demo;

class Program
{
  static void Main()
  {
    // --- One-Sample ---

    var x = new Sample(
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
      11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
      21, 22);

    WriteLine(Toolkit.Center(x));             // 11.5
    WriteLine(Toolkit.CenterBounds(x, 1e-3)); // [6;17]
    WriteLine(Toolkit.Spread(x));             // 7
    WriteLine(Toolkit.SpreadBounds(x, 1e-3, "demo")); // [1;18]

    // --- Two-Sample ---

    x = new Sample(
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
      16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30);
    var y = new Sample(
      21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
      36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50);

    WriteLine(Toolkit.Shift(x, y));             // -20
    WriteLine(Toolkit.ShiftBounds(x, y, 1e-3)); // [-28;-12]
    WriteLine(Toolkit.Ratio(x, y));             // 0.43669798282695127
    WriteLine(Toolkit.RatioBounds(x, y, 1e-3)); // [0.23255813953488377;0.6428571428571428]
    WriteLine(Toolkit.Disparity(x, y));         // -2.2222222222222223
    WriteLine(Toolkit.DisparityBounds(x, y, 1e-3, "demo")); // [-29;-0.4782608695652174]

    // --- Randomization ---

    var rng = new Rng("demo-uniform");
    WriteLine(rng.UniformDouble()); // 0.2640554428629759
    WriteLine(rng.UniformDouble()); // 0.9348534835582796

    rng = new Rng("demo-uniform-int");
    WriteLine(rng.UniformInt32(0, 100)); // 41

    rng = new Rng("demo-sample");
    WriteLine(string.Join(", ", rng.Sample([0.0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3))); // 3, 8, 9

    rng = new Rng("demo-resample");
    WriteLine(string.Join(", ", rng.Resample([1.0, 2, 3, 4, 5], 7))); // 3, 1, 3, 2, 4, 1, 2

    rng = new Rng("demo-shuffle");
    WriteLine(string.Join(", ", rng.Shuffle([1.0, 2, 3, 4, 5]))); // 4, 2, 3, 5, 1

    // --- Distributions ---

    rng = new Rng("demo-dist-additive");
    WriteLine(new Additive(0, 1).Sample(rng)); // 0.17410448679568188

    rng = new Rng("demo-dist-multiplic");
    WriteLine(new Multiplic(0, 1).Sample(rng)); // 1.1273244602673853

    rng = new Rng("demo-dist-exp");
    WriteLine(new Exp(1).Sample(rng)); // 0.6589065267276553

    rng = new Rng("demo-dist-power");
    WriteLine(new Power(1, 2).Sample(rng)); // 1.023677535537084

    rng = new Rng("demo-dist-uniform");
    WriteLine(new Uniform(0, 10).Sample(rng)); // 6.54043657816832
  }
}
