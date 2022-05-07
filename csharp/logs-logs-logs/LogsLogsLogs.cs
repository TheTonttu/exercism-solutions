using System;

public enum LogLevel
{
    Unknown = 0,
    Trace = 1,
    Debug = 2,
    Info = 4,
    Warning = 5,
    Error = 6,
    Fatal = 42,
}

static class LogLine
{

    private static readonly (string Text, LogLevel LogLevel)[] LogLevelCandidates =
    {
        ("TRC", LogLevel.Trace),
        ("DBG", LogLevel.Debug),
        ("INF", LogLevel.Info),
        ("WRN", LogLevel.Warning),
        ("ERR", LogLevel.Error),
        ("FTL", LogLevel.Fatal),
    };

    public static LogLevel ParseLogLevel(string logLine)
    {
        const int MinimumLevelContainingLineLength = 4;
        if (logLine is null || logLine.Length < MinimumLevelContainingLineLength)
        {
            return LogLevel.Unknown;
        }

        var logLevelPart = logLine.AsSpan(1, 3);
        foreach (var candidate in LogLevelCandidates)
        {
            if (logLevelPart.Equals(candidate.Text, StringComparison.OrdinalIgnoreCase))
            {
                return candidate.LogLevel;
            }
        }
        return LogLevel.Unknown;
    }

    public static string OutputForShortLog(LogLevel logLevel, string message) =>
        $"{(int)logLevel}:{message}";
}
