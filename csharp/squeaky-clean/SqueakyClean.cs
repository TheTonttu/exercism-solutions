using System;
using System.Text;

public static class Identifier
{
    private const string ControlCharReplacement = "CTRL";
    private const string WhiteSpaceReplacement = "_";
    private const char KebabCaseWordSeparator = '-';

    public static string Clean(string identifier)
    {
        var cleanIdBuilder = new StringBuilder();
        char? prevChar = null;
        foreach (char currChar in identifier)
        {
            if (CleanChar(currChar, prevChar) is string cleanedOutput)
            {
                cleanIdBuilder.Append(cleanedOutput);
            }
            prevChar = currChar;
        }
        return cleanIdBuilder.ToString();
    }

    private static string? CleanChar(char currChar, char? prevChar)
    {
        return currChar switch
        {
            _ when char.IsControl(currChar) => ControlCharReplacement,
            _ when char.IsWhiteSpace(currChar) => WhiteSpaceReplacement,
            char letter when char.IsLetter(currChar) => CleanLetter(letter, prevChar)?.ToString(),
            _ => null
        };
    }

    private static char? CleanLetter(char letter, char? prevChar)
    {
        if (IsStartOfNextWord(prevChar)) { return char.ToUpperInvariant(letter); }
        if (IsGreekLowercaseLetter(letter)) { return null; }

        return letter;
    }

    private static bool IsStartOfNextWord(char? prevChar) => prevChar == KebabCaseWordSeparator;
    private static bool IsGreekLowercaseLetter(char letter) => letter is >= 'α' and <= 'ω';
}
