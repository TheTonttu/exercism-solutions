using System;
using System.Collections.Generic;
using System.Linq;

class BirdCount
{
    private readonly int[] _birdsPerDay;

    public BirdCount(int[] birdsPerDay)
    {
        _birdsPerDay = birdsPerDay;
    }

    public static int[] LastWeek()
    {
        return new[] { 0, 2, 5, 3, 7, 8, 4 };
    }

    public int Today()
    {
        return _birdsPerDay.Last();
    }

    public void IncrementTodaysCount()
    {
        _birdsPerDay[^1] += 1;
    }

    public bool HasDayWithoutBirds()
    {
        return _birdsPerDay.Any(bc => bc == 0);
    }

    public int CountForFirstDays(int numberOfDays)
    {
        return _birdsPerDay.Take(numberOfDays).Sum();
    }

    public int BusyDays()
    {
        const int BusyCount = 5;
        return _birdsPerDay.Count(bc => bc >= BusyCount);
    }
}
