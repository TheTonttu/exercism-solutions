using System;

public static class Gigasecond
{
    private static readonly TimeSpan GigasecondTimeSpan = TimeSpan.FromSeconds(1_000_000_000);

    public static DateTime Add(DateTime moment)
    {
        return moment.Add(GigasecondTimeSpan);
    }
}