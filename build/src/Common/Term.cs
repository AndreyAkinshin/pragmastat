using Spectre.Console;

namespace Common;

public static class Term
{
    public static void Info(string message)
    {
        try
        {
            AnsiConsole.MarkupLine($"[green]{message}[/]");
        }
        catch (Exception e)
        {
            Console.WriteLine(message);
            RawError(e);
        }
    }

    public static void Warning(string message)
    {
        try
        {
            AnsiConsole.MarkupLine($"[yellow]{message}[/]");
        }
        catch (Exception e)
        {
            Console.WriteLine(message);
            RawError(e);
        }
    }

    public static void Error(string message)
    {
        try
        {
            AnsiConsole.MarkupLine($"[red]{message}[/]");
        }
        catch (Exception e)
        {
            Console.WriteLine(message);
            RawError(e);
        }
    }

    private static void RawError(Exception e) => RawError(e.ToString());

    private static void RawError(string message)
    {
        Console.ForegroundColor = ConsoleColor.Red;
        Console.WriteLine(message);
        Console.ResetColor();
    }
}