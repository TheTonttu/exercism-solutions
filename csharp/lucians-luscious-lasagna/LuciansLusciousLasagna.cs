public class Lasagna
{
    public int ExpectedMinutesInOven() => 40;

    public int RemainingMinutesInOven(int minutesInOven)
    {
        return ExpectedMinutesInOven() - minutesInOven;
    }

    public int PreparationTimeInMinutes(int layerCount)
    {
        const int LayerPreparationTimeInMinutes = 2;
        return layerCount * LayerPreparationTimeInMinutes;
    }

    public int ElapsedTimeInMinutes(int layerCount, int minutesInOven)
    {
        return PreparationTimeInMinutes(layerCount) + minutesInOven;
    }
}
