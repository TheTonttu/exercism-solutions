using System;
using System.Text;

public static class Acronym
{
    public static string Abbreviate(string phrase)
    {
        if (string.IsNullOrEmpty(phrase))
        {
            return string.Empty;
        }

        var acronymBuilder = new StringBuilder();

        ReadOnlySpan<char> remainder = phrase.AsSpan();
        char prevChar = '\0';
        while (remainder.Length > 0)
        {
            char currChar = remainder[0];

            if (char.IsAsciiLetter(currChar) &&
                // Not part of word
                !(char.IsAsciiLetter(prevChar) || IsDiacriticalMark(prevChar)))
            {
                char capitalized = char.ToUpperInvariant(currChar);
                acronymBuilder.Append(capitalized);
            }

            remainder = remainder[1..];
            prevChar = currChar;
        }

        return acronymBuilder.ToString();
    }
    private static bool IsDiacriticalMark(char character) => character is '\'';
}