using System.Reflection;
using JetBrains.Annotations;

namespace Pragmastat.Internal;

internal static class CtorArgumentSerializer
{
    [PublicAPI]
    public static Dictionary<string, double> SerializeToDictionary(object obj)
    {
        var list = SerializeToList(obj);
        var result = new Dictionary<string, double>();

        foreach (var (name, value) in list)
        {
            result[name] = value;
        }

        return result;
    }

    [PublicAPI]
    public static List<(string Name, double Value)> SerializeToList(object obj)
    {
        var result = new List<(string Name, double Value)>();
        var type = obj.GetType();

        // Find first public constructor
        var constructor = type.GetConstructors(BindingFlags.Public | BindingFlags.Instance)
            .FirstOrDefault();

        if (constructor == null)
            return result;

        // Get constructor parameter names
        var parameters = constructor.GetParameters();
        var properties = type.GetProperties(BindingFlags.Public | BindingFlags.Instance);

        foreach (var parameter in parameters)
        {
            string? parameterName = parameter.Name;
            if (parameterName == null)
                continue;

            // Find matching property (case-insensitive)
            var matchingProperty = properties.FirstOrDefault(p =>
                string.Equals(p.Name, parameter.Name, StringComparison.OrdinalIgnoreCase));

            if (matchingProperty?.PropertyType == typeof(double) && matchingProperty.CanRead)
            {
                var propertyValue = matchingProperty.GetValue(obj);
                if (propertyValue is double doubleValue)
                {
                    // Use camelCase for parameter name
                    string camelCaseName = ToCamelCase(parameterName);
                    result.Add((camelCaseName, doubleValue));
                }
            }
        }

        return result;
    }

    [PublicAPI]
    public static T Deserialize<T>(Dictionary<string, double> parameters)
    {
        var type = typeof(T);

        // Find first public constructor
        var constructor = type.GetConstructors(BindingFlags.Public | BindingFlags.Instance)
            .FirstOrDefault();

        if (constructor == null)
            throw new InvalidOperationException($"No public constructor found for type {type.Name}");

        var constructorParameters = constructor.GetParameters();
        var args = new object[constructorParameters.Length];

        for (int i = 0; i < constructorParameters.Length; i++)
        {
            var parameter = constructorParameters[i];
            string? parameterName = parameter.Name;
            if (parameterName == null)
                throw new InvalidOperationException($"No parameter name found for parameter {parameterName}");
            string camelCaseName = ToCamelCase(parameterName);

            if (parameter.ParameterType == typeof(double))
            {
                if (parameters.TryGetValue(camelCaseName, out var value))
                {
                    args[i] = value;
                }
                else if (parameter is { HasDefaultValue: true, DefaultValue: not null })
                {
                    args[i] = parameter.DefaultValue;
                }
                else
                {
                    throw new ArgumentException($"Missing required parameter: {camelCaseName}");
                }
            }
            else if (parameter is { HasDefaultValue: true, DefaultValue: not null })
            {
                args[i] = parameter.DefaultValue;
            }
            else
            {
                throw new InvalidOperationException(
                    $"Unsupported parameter type: {parameter.ParameterType.Name} for parameter {parameter.Name}");
            }
        }

        return (T)constructor.Invoke(args);
    }

    private static string ToCamelCase(string input)
    {
        if (string.IsNullOrEmpty(input))
            return input;

        return char.ToLowerInvariant(input[0]) + input.Substring(1);
    }
}