static class LogLine
{
    public static string Message(string logLine)
    {
        int separatorIndex = logLine.IndexOf(":");
        int messageStartIndex = separatorIndex + 1;
        return logLine[messageStartIndex..].Trim();
    }

    public static string LogLevel(string logLine)
    {
        int levelStartIndex = 1;
        int levelEndIndex = logLine.IndexOf("]");
        return logLine[levelStartIndex..levelEndIndex].ToLowerInvariant();
    }

    public static string Reformat(string logLine)
    {
        return $"{Message(logLine)} ({LogLevel(logLine)})";
    }
}
