using System;

public class DndCharacter
{
    private static readonly Random Rng = new();

    public int Strength { get; }
    public int Dexterity { get; }
    public int Constitution { get; }
    public int Intelligence { get; }
    public int Wisdom { get; }
    public int Charisma { get; }
    public int Hitpoints { get; }

    private DndCharacter(int strength, int dexterity, int constitution, int intelligence, int wisdom, int charisma)
    {
        Strength = strength;
        Dexterity = dexterity;
        Constitution = constitution;
        Intelligence = intelligence;
        Wisdom = wisdom;
        Charisma = charisma;
        Hitpoints = 10 + Modifier(constitution);
    }

    public static int Modifier(int score) => (score / 2) - 5;

    public static int Ability()
    {
        const int TotalRolls = 4;

        int discardedMinRoll = int.MaxValue;
        int statSum = 0;
        for (int i = 0; i < TotalRolls; i++)
        {
            int d6Roll = RollD6();
            if (discardedMinRoll > d6Roll)
            {
                discardedMinRoll = d6Roll;
            }
            statSum += d6Roll;
        }

        return statSum - discardedMinRoll;
    }

    public static DndCharacter Generate() => 
        new(
            strength: Ability(),
            dexterity: Ability(),
            constitution: Ability(),
            intelligence: Ability(),
            wisdom: Ability(),
            charisma: Ability()
        );

    private static int RollD6() =>
        // Upper limit is exclusive.
        Rng.Next(1, 7);
}
