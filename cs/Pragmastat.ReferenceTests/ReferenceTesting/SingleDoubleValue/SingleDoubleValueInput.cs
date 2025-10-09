namespace Pragmastat.ReferenceTests.ReferenceTesting.SingleDoubleValue;

public class SingleDoubleValueInput(string name, double[] arg)
{
  public string Name { get; } = name;
  public double[] Arg { get; } = arg;
}
