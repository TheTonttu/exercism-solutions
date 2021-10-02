using System;

public class Player
{
    private Random _random = new();

    public int RollDie() =>
        _random.Next(1, 18);

    public double GenerateSpellStrength() =>
        _random.NextDouble() * 100.0;
}
