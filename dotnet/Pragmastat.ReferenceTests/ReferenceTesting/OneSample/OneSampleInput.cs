using JetBrains.Annotations;
using Pragmastat.Core;

namespace Pragmastat.ReferenceTests.ReferenceTesting.OneSample;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class OneSampleInput
{
    public Dictionary<string, double>? Parameters { get; set; }
    public double[] X { get; set; } = [];
    public double[]? W { get; set; }

    public OneSampleInput()
    {
    }

    public OneSampleInput(Dictionary<string, double>? parameters, double[] x, double[]? w)
    {
        Parameters = parameters;
        X = x;
        W = w;
    }

    public OneSampleInput(Sample s, Dictionary<string, double>? parameters = null)
    {
        Parameters = parameters;
        X = s.Values.ToArray();
        W = s.IsWeighted ? s.Weights.ToArray() : null;
    }

    public Sample ToSample() => W == null ? new Sample(X) : new Sample(X, W);
}