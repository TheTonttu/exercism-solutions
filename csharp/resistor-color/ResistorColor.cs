using System;
using System.Linq;

public static class ResistorColor
{
    public static int ColorCode(string color)
    {
        if (Enum.TryParse<ResistorColorCode>(color, ignoreCase: true, out var colorCode))
        {
            return (int)colorCode;
        }
        return (int)ResistorColorCode.Undefined;
    }

    public static string[] Colors()
    {
        return Enum.GetValues(typeof(ResistorColorCode))
                   .Cast<ResistorColorCode>()
                   .Where(e => e != ResistorColorCode.Undefined)
                   .Select(e => e.ToString().ToLower())
                   .ToArray();
    }
}

public enum ResistorColorCode
{
    Undefined = int.MinValue,
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9
}