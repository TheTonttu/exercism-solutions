using System;
using System.Linq;

public static class Bob
{
    public static string Response(string statement)
    {
        if (Question(statement))
        {
            return Yelling(statement)
                ? "Calm down, I know what I'm doing!"
                : "Sure.";
        }

        if (Yelling(statement))
        {
            return "Whoa, chill out!";
        }

        if (Blank(statement))
        {
            return "Fine. Be that way!";
        }

        return "Whatever.";
    }

    private static bool Question(string statement) => statement.AsSpan().Trim().EndsWith("?");
    private static bool Yelling(string statement) => statement.Any(c => char.IsUpper(c)) && statement.All(c => !char.IsLetter(c) || char.IsUpper(c));
    private static bool Blank(string statement) => statement is not null && string.IsNullOrWhiteSpace(statement);
}