using System;

public static class Bob
{
    public static string Response(string statement)
    {
        var trimmedStatement = statement.AsSpan().Trim();

        if (Blank(trimmedStatement))
        {
            return "Fine. Be that way!";
        }

        if (Question(trimmedStatement))
        {
            return Yelling(trimmedStatement)
                ? "Calm down, I know what I'm doing!"
                : "Sure.";
        }

        if (Yelling(trimmedStatement))
        {
            return "Whoa, chill out!";
        }

        return "Whatever.";
    }

    private static bool Blank(ReadOnlySpan<char> statement) => statement.IsEmpty;
    private static bool Question(ReadOnlySpan<char> statement) => statement.EndsWith("?");

    private static bool Yelling(ReadOnlySpan<char> statement)
    {
        bool hasUpperLetters = false;
        foreach (var c in statement)
        {
            if (!char.IsLetter(c)) { continue; }
            if (char.IsLower(c)) { return false; }
            hasUpperLetters = true;
        }
        return hasUpperLetters;
    }
}