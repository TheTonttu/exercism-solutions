using System;

public static class TwoFer
{
    private const string MessageTemplate = "One for {0}, one for me.";
    private const string DefaultName = "you";

    public static string Speak()
    {
        return Speak(String.Empty);
    }

    public static string Speak(string name)
    {
        string sanitizedName = SanitizeName(name);
        return String.Format(MessageTemplate, sanitizedName);
    }

    private static string SanitizeName(string name)
    {
        if (String.IsNullOrWhiteSpace(name))
        {
            return DefaultName;
        }
        return name.Trim();
    }
}
