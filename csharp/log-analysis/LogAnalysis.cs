using System;

public static class LogAnalysis
{
    private const int NotFoundIndex = -1;

    public static string SubstringAfter(this string str, string delimiter)
    {
        if (delimiter is null)
        {
            throw new ArgumentNullException(nameof(delimiter));
        }

        if (String.IsNullOrEmpty(str)) { return String.Empty; }

        int delimiterIndex = str.IndexOf(delimiter);
        if (delimiterIndex <= NotFoundIndex) { return String.Empty; }

        int substringIndex = delimiterIndex + delimiter.Length;
        return str[substringIndex..];
    }

    public static string SubstringBetween(this string str, string startDelimiter, string endDelimiter)
    {
        if (startDelimiter is null)
        {
            throw new ArgumentNullException(nameof(startDelimiter));
        }

        if (endDelimiter is null)
        {
            throw new ArgumentNullException(nameof(endDelimiter));
        }

        if (String.IsNullOrEmpty(str)) { return String.Empty; }

        int startDelimiterIndex = str.IndexOf(startDelimiter);
        if (startDelimiterIndex <= NotFoundIndex) { return String.Empty; }

        int substringStartIndex = startDelimiterIndex + startDelimiter.Length;
        int endDelimiterIndex = str.IndexOf(endDelimiter, substringStartIndex);
        if (endDelimiterIndex <= NotFoundIndex) { return String.Empty; }

        int substringEndIndex = endDelimiterIndex;
        return str[substringStartIndex..substringEndIndex];
    }

    public static string Message(this string logLine)
    {
        return logLine.SubstringAfter(":").Trim();
    }

    public static string LogLevel(this string logLine)
    {
        return logLine.SubstringBetween("[", "]");
    }
}