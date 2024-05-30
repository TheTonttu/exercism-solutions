using System;
using System.Text;

public static class RomanNumeralExtension
{
    public static string ToRoman(this int value)
    {
        var stringBuilder = new StringBuilder();

        int remainder = value;
        while (remainder > 0)
        {
            (int subtraction, string romanNumerals) = remainder switch
            {
                >= 1000 => (1000, "M"),
                >= 900 => (900, "CM"),
                >= 500 => (500, "D"),
                >= 400 => (400, "CD"),
                >= 100 => (100, "C"),
                >= 90 => (90, "XC"),
                >= 50 => (50, "L"),
                >= 40 => (40, "XL"),
                >= 10 => (10, "X"),
                >= 9 => (9, "IX"),
                >= 5 => (5, "V"),
                >= 4 => (4, "IV"),
                >= 1 => (1, "I"),
                _ => throw new InvalidOperationException("Should be out of loop by now")
            };
            stringBuilder.Append(romanNumerals);
            remainder -= subtraction;
        }

        return stringBuilder.ToString();
    }
}