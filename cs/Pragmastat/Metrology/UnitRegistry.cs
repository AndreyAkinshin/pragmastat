namespace Pragmastat.Metrology;

public class UnitRegistry
{
  private readonly Dictionary<string, MeasurementUnit> _byId = new();

  public void Register(MeasurementUnit unit)
  {
    if (_byId.ContainsKey(unit.Id))
      throw new ArgumentException($"Unit with id '{unit.Id}' is already registered");
    _byId[unit.Id] = unit;
  }

  public void RegisterFamily(params MeasurementUnit[] units)
  {
    foreach (var unit in units)
      Register(unit);
  }

  public MeasurementUnit Resolve(string id)
  {
    if (_byId.TryGetValue(id, out var unit))
      return unit;
    throw new ArgumentException($"Unknown unit id: '{id}'");
  }

  public bool TryResolve(string id, out MeasurementUnit unit)
  {
    if (_byId.TryGetValue(id, out unit!))
      return true;
    unit = NumberUnit.Instance;
    return false;
  }

  public static UnitRegistry Standard()
  {
    var registry = new UnitRegistry();
    registry.Register(NumberUnit.Instance);
    registry.Register(RatioUnit.Instance);
    registry.Register(DisparityUnit.Instance);
    return registry;
  }
}
