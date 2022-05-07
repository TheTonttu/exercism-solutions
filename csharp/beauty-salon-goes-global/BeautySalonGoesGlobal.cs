using System;
using System.Globalization;

public enum Location
{
    NewYork,
    London,
    Paris
}

public enum AlertLevel
{
    Early,
    Standard,
    Late
}

public static class Appointment
{
    private static readonly DateTime BadFormatDate = new(1, 1, 1, 0, 0, 0);
    private static readonly TimeSpan DaylightSavingTimeChangeCheckOffset = TimeSpan.FromDays(7);
    private static readonly ILocationTimeZoneProvider LocationTimeZoneProvider = new LocationTimeZoneProvider();

    public static DateTime ShowLocalTime(DateTime dtUtc)
    {
        return dtUtc.ToLocalTime();
    }

    public static DateTime Schedule(string appointmentDateDescription, Location location)
    {
        var locationAppointmentDate = DateTime.Parse(appointmentDateDescription);
        var locationTimeZone = LocationTimeZoneProvider.GetTimeZone(location);
        var utcAppointmentTime = TimeZoneInfo.ConvertTimeToUtc(locationAppointmentDate, locationTimeZone);
        return utcAppointmentTime;
    }

    public static DateTime GetAlertTime(DateTime appointment, AlertLevel alertLevel)
    {
        var alertTimeOffset = GetAlertTimeOffset(alertLevel);
        return appointment - alertTimeOffset;
    }

    public static bool HasDaylightSavingChanged(DateTime dt, Location location)
    {
        var locationTimeZone = LocationTimeZoneProvider.GetTimeZone(location);
        bool isDst = locationTimeZone.IsDaylightSavingTime(dt);

        return locationTimeZone.IsDaylightSavingTime(dt - DaylightSavingTimeChangeCheckOffset) != isDst
            || locationTimeZone.IsDaylightSavingTime(dt + DaylightSavingTimeChangeCheckOffset) != isDst;
    }

    public static DateTime NormalizeDateTime(string dtStr, Location location)
    {
        var locationCultureInfo = GetCultureInfo(location);
        if (DateTime.TryParse(dtStr, locationCultureInfo, DateTimeStyles.None, out DateTime parsedDateTime))
        {
            return parsedDateTime;
        }
        return BadFormatDate;
    }

    private static TimeSpan GetAlertTimeOffset(AlertLevel alertLevel) => alertLevel switch
    {
        AlertLevel.Early => TimeSpan.FromDays(1),
        AlertLevel.Standard => new TimeSpan(hours: 1, minutes: 45, seconds: 0),
        AlertLevel.Late => TimeSpan.FromMinutes(30),
        _ => throw new NotSupportedException(alertLevel.ToString())
    };

    private static CultureInfo GetCultureInfo(Location location) => location switch
    {
        Location.NewYork => CultureInfo.GetCultureInfo("en-US"),
        Location.London => CultureInfo.GetCultureInfo("en-GB"),
        Location.Paris => CultureInfo.GetCultureInfo("fr-FR"),
        _ => throw new NotSupportedException(location.ToString())
    };
}

public sealed class LocationTimeZoneProvider : ILocationTimeZoneProvider
{
    private static readonly ILocationTimeZoneProvider OsSpecificProvider =
        OperatingSystem.IsWindows()
            ? new WindowsTimeZoneProvider()
            : new UnixTimeZoneProvider();

    public TimeZoneInfo GetTimeZone(Location location) => OsSpecificProvider.GetTimeZone(location);
}

internal class UnixTimeZoneProvider : ILocationTimeZoneProvider
{
    public TimeZoneInfo GetTimeZone(Location location)
    {
        string ianaId = LocationIanaTimeZoneIds.GetIanaId(location);
        return TimeZoneInfo.FindSystemTimeZoneById(ianaId);
    }
}

internal class WindowsTimeZoneProvider : ILocationTimeZoneProvider
{
    public TimeZoneInfo GetTimeZone(Location location)
    {
        string ianaId = LocationIanaTimeZoneIds.GetIanaId(location);
        if (TimeZoneInfo.TryConvertIanaIdToWindowsId(ianaId, out string windowsTzId))
        {
            return TimeZoneInfo.FindSystemTimeZoneById(windowsTzId);
        }
        throw new TimeZoneNotFoundException($"Time zone not found for {location}.");
    }
}

public interface ILocationTimeZoneProvider
{
    TimeZoneInfo GetTimeZone(Location location);
}

public static class LocationIanaTimeZoneIds
{
    public static string GetIanaId(Location location) => location switch
    {
        Location.NewYork => "America/New_York",
        Location.London => "Europe/London",
        Location.Paris => "Europe/Paris",
        _ => throw new NotSupportedException(location.ToString())
    };
}
