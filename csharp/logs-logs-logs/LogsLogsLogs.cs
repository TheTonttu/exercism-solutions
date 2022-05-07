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
    public static LogLevel ParseLogLevel(string logLine) => logLine switch
    {
        string trace when IsMatch("[TRC]", trace) => LogLevel.Trace,
        string debug when IsMatch("[DBG]", debug) => LogLevel.Debug,
        string info when IsMatch("[INF]", info) => LogLevel.Info,
        string warning when IsMatch("[WRN]", warning) => LogLevel.Warning,
        string error when IsMatch("[ERR]", error) => LogLevel.Error,
        string fatal when IsMatch("[FTL]", fatal) => LogLevel.Fatal,
        _ => LogLevel.Unknown,
    };

    public static string OutputForShortLog(LogLevel logLevel, string message) =>
        $"{(int)logLevel}:{message}";

    private static bool IsMatch(string value, string logLine) =>
        logLine.StartsWith(value, StringComparison.OrdinalIgnoreCase);
}
