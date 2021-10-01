public class Lasagna
{
    private const int LayerPreparationTimeInMinutes = 2;

    public int ExpectedMinutesInOven() => 40;

    public int RemainingMinutesInOven(int minutesInOven)
    {
        return ExpectedMinutesInOven() - minutesInOven;
    }

    public int PreparationTimeInMinutes(int layerCount)
    {
        return layerCount * LayerPreparationTimeInMinutes;
    }

    public int ElapsedTimeInMinutes(int layerCount, int minutesInOven)
    {
        return PreparationTimeInMinutes(layerCount) + ElapsedMinutesInOven(minutesInOven);
    }

    private int ElapsedMinutesInOven(int minutesInOven)
    {
        return ExpectedMinutesInOven() - RemainingMinutesInOven(minutesInOven);
    }
}
