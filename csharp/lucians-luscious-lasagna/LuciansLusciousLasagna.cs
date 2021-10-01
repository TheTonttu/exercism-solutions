public class Lasagna
{
    private const int LayerPreparationTimeInMinutes = 2;

    public int ExpectedMinutesInOven() => 40;

    public int RemainingMinutesInOven(int minutesInOven) => ExpectedMinutesInOven() - minutesInOven;

    public int PreparationTimeInMinutes(int layerCount) => layerCount * LayerPreparationTimeInMinutes;

    public int ElapsedTimeInMinutes(int layerCount, int minutesInOven) => PreparationTimeInMinutes(layerCount) + (ExpectedMinutesInOven() - RemainingMinutesInOven(minutesInOven));
}
