using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Text;

public static class Identifier
{
    private const string ControlCharReplacement = "CTRL";
    private const string WhiteSpaceReplacement = "_";
    private const char KebabCaseWordSeparator = '-';

    private static readonly HashSet<char> GreekLowerCaseLetters =
        Enumerable.Range('α', 'ω' - 'α' + 1)
                  .Select(i => (char)i)
                  .ToHashSet();

    public static string Clean(string identifier)
    {
        var cleanIdBuilder = new StringBuilder();
        char? previousChar = null;
        foreach (char currentChar in identifier)
        {
            string? cleanedOutput = CleanChar(currentChar, previousChar);

            if (cleanedOutput != null)
            {
                cleanIdBuilder.Append(cleanedOutput);
            }
            previousChar = currentChar;
        }
        return cleanIdBuilder.ToString();
    }

    private static string? CleanChar(char currentChar, char? previousChar)
    {
        if (char.IsControl(currentChar))
        {
            return ControlCharReplacement;
        }
        else if (char.IsWhiteSpace(currentChar))
        {
            return WhiteSpaceReplacement;
        }
        else if (char.IsLetter(currentChar))
        {
            char letter = currentChar;

            bool isStartOfNextKebabCaseWord = previousChar == KebabCaseWordSeparator;
            if (isStartOfNextKebabCaseWord)
            {
                letter = char.ToUpperInvariant(letter);
            }

            if (!GreekLowerCaseLetters.Contains(letter))
            {
                return letter.ToString();
            }
        }
        return null;
    }
}
