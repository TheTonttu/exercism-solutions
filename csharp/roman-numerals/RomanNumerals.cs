using System.Text;

public static class RomanNumeralExtension
{
    public static string ToRoman(this int value)
    {
        var stringBuilder = new StringBuilder(capacity: value);

        stringBuilder
            .Append('I', value)
            // 5
            .Replace("IIIII", "V")
            // 10
            .Replace("VV", "X")
            // 50
            .Replace("XXXXX", "L")
            // 100
            .Replace("LL", "C")
            // 500
            .Replace("CCCCC", "D")
            // 1000
            .Replace("DD", "M")
            // 4
            .Replace("IIII", "IV")
            // 5 + 4 = 9
            .Replace("VIV", "IX")
            // 40
            .Replace("XXXX", "XL")
            // 50 + 40 = 90
            .Replace("LXL", "XC")
            // 400
            .Replace("CCCC", "CD")
            // 500 + 400 = 900
            .Replace("DCD", "CM");

        return stringBuilder.ToString();
    }
}