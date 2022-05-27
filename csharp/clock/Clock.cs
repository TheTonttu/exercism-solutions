using System;

public class Clock : IEquatable<Clock>
{
    // Would otherwise use record and TimeOnly but that does not seem like the point of the exercise. :-P

    private const int MinutesInHour = 60;

    public int Hours { get; }
    public int Minutes { get; }

    public Clock(int hours, int minutes)
    {
        int totalMinutesInDay = TotalMinutesInDay(hours, minutes);
        Hours = totalMinutesInDay / MinutesInHour;
        Minutes = totalMinutesInDay % MinutesInHour;
    }

    public Clock Add(int minutesToAdd) => new(Hours, Minutes + minutesToAdd);
    public Clock Subtract(int minutesToSubtract) => new(Hours, Minutes - minutesToSubtract);

    public override bool Equals(object obj) =>
        obj is Clock other &&
        Equals(other);

    public override int GetHashCode() => HashCode.Combine(Hours, Minutes);

    public bool Equals(Clock other) =>
        ReferenceEquals(this, other) ||
        (this.Hours == other.Hours &&
        this.Minutes == other.Minutes);

    public override string ToString() => $"{Hours:00}:{Minutes:00}";

    public static bool operator ==(Clock left, Clock right) =>
        left is null
            ? right is null
            : left.Equals(right);

    public static bool operator !=(Clock left, Clock right) => !(left == right);

    private static int TotalMinutesInDay(int hours, int minutes)
    {
        const int HoursInDay = 24;
        const int MinutesInDay = HoursInDay * MinutesInHour;

        int totalMinutesInDay = ((hours * MinutesInHour) + minutes) % MinutesInDay;
        return totalMinutesInDay < 0
            ? totalMinutesInDay + MinutesInDay
            : totalMinutesInDay;
    }
}
