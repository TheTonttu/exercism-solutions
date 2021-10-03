using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Text;

public static class Identifier
{
    private const string ControlCharReplacement = "CTRL";
    private const string WhiteSpaceReplacement = "_";

    private static readonly HashSet<char> GreekLowerCaseLetters =
        Enumerable.Range('α', 'ω' - 'α' + 1)
                  .Select(i => (char)i)
                  .ToHashSet();

    public static string Clean(string identifier)
    {
        var cleanIdBuilder = new StringBuilder();
        char? prevChar = null;
        foreach (char c in identifier)
        {
            if (char.IsControl(c))
            {
                cleanIdBuilder.Append(ControlCharReplacement);
            }
            else if (char.IsWhiteSpace(c))
            {
                cleanIdBuilder.Append(WhiteSpaceReplacement);
            }
            else if (char.IsLetter(c))
            {
                char letter = c;

                bool isKebabCase = prevChar == '-';
                if (isKebabCase)
                {
                    letter = char.ToUpperInvariant(letter);
                }

                if (!GreekLowerCaseLetters.Contains(letter))
                {
                    cleanIdBuilder.Append(letter);
                }

            }

            prevChar = c;
        }
        return cleanIdBuilder.ToString();
    }
}
