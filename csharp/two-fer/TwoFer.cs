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
        name = name?.Trim();
        if (String.IsNullOrEmpty(name))
        {
            name = DefaultName;
        }
        return String.Format(MessageTemplate, name);
    }
}
