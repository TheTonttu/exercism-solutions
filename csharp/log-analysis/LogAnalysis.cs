using System;

public static class LogAnalysis
{
    public static string SubstringAfter(this string line, string delimiter)
    {
        if (delimiter is null) { throw new ArgumentNullException(nameof(delimiter)); }
        if (String.IsNullOrEmpty(line)) { return line; }

        int delimiterIndex = line.IndexOf(delimiter);
        if (!IndexFound(delimiterIndex)) { return String.Empty; }

        int substringIndex = delimiterIndex + delimiter.Length;
        return line[substringIndex..];
    }

    public static string SubstringBetween(this string line, string startDelimiter, string endDelimiter)
    {
        if (startDelimiter is null) { throw new ArgumentNullException(nameof(startDelimiter)); }
        if (endDelimiter is null) { throw new ArgumentNullException(nameof(endDelimiter)); }
        if (String.IsNullOrEmpty(line)) { return line; }

        int startDelimiterIndex = line.IndexOf(startDelimiter);
        if (!IndexFound(startDelimiterIndex)) { return String.Empty; }

        int substringStartIndex = startDelimiterIndex + startDelimiter.Length;
        int endDelimiterIndex = line.IndexOf(endDelimiter, substringStartIndex);
        if (!IndexFound(endDelimiterIndex)) { return String.Empty; }

        int substringEndIndex = endDelimiterIndex;
        return line[substringStartIndex..substringEndIndex];
    }

    public static string Message(this string logLine)
    {
        return logLine.SubstringAfter(":")?.Trim();
    }

    public static string LogLevel(this string logLine)
    {
        return logLine.SubstringBetween("[", "]");
    }

    private static bool IndexFound(int index)
    {
        const int NotFoundIndex = -1;
        return index > NotFoundIndex;
    }
}