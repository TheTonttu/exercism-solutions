using System;

public class Player
{
    private Random _random = new();

    public int RollDie()
    {
        return _random.Next(1, 18);
    }

    public double GenerateSpellStrength()
    {
        return _random.NextDouble() * 100.0;
    }
}
