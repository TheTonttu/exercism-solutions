using System;
using System.Linq;

[Flags]
public enum Allergen
{
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128
}

public class Allergies
{
    private Allergen _allergen;

    public Allergies(int mask)
    {
        _allergen = (Allergen)mask;
    }

    public bool IsAllergicTo(Allergen allergen)
    {
        return _allergen.HasFlag(allergen);
    }

    public Allergen[] List()
    {
        // Could also precalculate array in constructor and then return copy of the array.
        return Enum.GetValues(typeof(Allergen))
            .Cast<Allergen>()
            .Where(e => _allergen.HasFlag(e))
            .ToArray();
    }
}