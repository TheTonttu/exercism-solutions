using System;

static class AssemblyLine
{
    public static double ProductionRatePerHour(int speed)
    {
        const int BaseProductionRatePerHour = 221;
        return speed * BaseProductionRatePerHour * SuccessRate(speed);
    }

    public static int WorkingItemsPerMinute(int speed)
    {
        const int MinutesInHour = 60;
        return (int)(ProductionRatePerHour(speed) / MinutesInHour);
    }

    private static double SuccessRate(int speed)
    {
        const double UnsupportedSuccessRate = 0.0;

        if (1 <= speed && speed <= 4)
        {
            return 1.0;
        }
        else if (5 <= speed && speed <= 8)
        {
            return 0.9;
        }
        else if (speed == 9)
        {
            return 0.8;
        }
        else if (speed == 10)
        {
            return 0.77;
        }
        return UnsupportedSuccessRate;
    }

    #region Alternative Success Rate Table Lookup

    private static readonly double[] SuccessRateLookupTable =
    {
        0.0,
        // 1-4
        1.0, 1.0, 1.0, 1.0,
        // 5-8
        0.9, 0.9, 0.9, 0.9,
        // 9
        0.8,
        // 10
        0.77
    };

    private static double SuccessRateFromLookupTable(int speed)
    {
        const double UnsupportedSuccessRate = 0.0;

        int maxSpeed = SuccessRateLookupTable.Length - 1;
        if (0 <= speed && speed <= maxSpeed)
        {
            return SuccessRateLookupTable[speed];
        }
        return UnsupportedSuccessRate;
    }

    #endregion 
}
