static class AssemblyLine
{
    private const int BaseProductionRatePerHour = 221;
    private const int MinutesInHour = 60;

    public static double SuccessRate(int speed) =>
        speed switch
        {
            >= 1 and <= 4 => 1.0,
            >= 5 and <= 8 => 0.9,
            9 => 0.8,
            10 => 0.77,
            _ => 0.0
        };

    public static double ProductionRatePerHour(int speed) => speed * BaseProductionRatePerHour * SuccessRate(speed);
    public static int WorkingItemsPerMinute(int speed) => (int)(ProductionRatePerHour(speed) / MinutesInHour);
}
