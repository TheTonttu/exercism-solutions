using System;
using System.Linq;

public static class ResistorColor
{
    public static int ColorCode(string color)
    {
        return (int)Enum.Parse<ResistorColorCode>(color, ignoreCase: true);
    }

    public static string[] Colors()
    {
        return Enum.GetNames(typeof(ResistorColorCode))
                   .Select(s => s.ToLower())
                   .ToArray();
    }
}

public enum ResistorColorCode
{
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