using System;
using System.Collections.Generic;
using System.Linq;

using static Color;
using static Nationality;
using static Pet;
using static Drink;
using static Smoke;

public enum Color { Red, Green, Ivory, Yellow, Blue }
public enum Nationality { Englishman, Spaniard, Ukranian, Japanese, Norwegian }
public enum Pet { Dog, Snails, Fox, Horse, Zebra }
public enum Drink { Coffee, Tea, Milk, OrangeJuice, Water }
public enum Smoke { OldGold, Kools, Chesterfields, LuckyStrike, Parliaments }

public static class ZebraPuzzle
{
    private static readonly House[] Street = GenerateStreet();

    public static Nationality DrinksWater()
    {
        var waterDrinker = Street.First(h => h.Drink == Water);
        return waterDrinker.Nationality;
    }

    public static Nationality OwnsZebra()
    {
        var zebraOwner = Street.First(h => h.Pet == Zebra);
        return zebraOwner.Nationality;
    }

    public static House[] GenerateStreet()
    {
        // Meh, let's do this manually.

        // There are five houses.
        var builders = new HouseBuilder[5];

        // The Englishman lives in the red house.
        var english = new HouseBuilder()
        {
            Color = Red,
            Nationality = Englishman
        };
        // The Spaniard owns the dog.
        var spanish = new HouseBuilder()
        {
            Nationality = Spaniard,
            Pet = Dog
        };
        // The Ukrainian drinks tea.
        var ukrainian = new HouseBuilder()
        {
            Nationality = Ukranian,
            Drink = Tea
        };
        // The Japanese smokes Parliaments.
        var japanese = new HouseBuilder()
        {
            Nationality = Japanese,
            Smoke = Parliaments
        };
        var norwegian = new HouseBuilder()
        {
            Nationality = Norwegian,
        };

        // The Norwegian lives in the first house.
        builders[0] = norwegian;
        if (builders[0] is var h0)
        {
            // Kools are smoked in the yellow house.
            h0.Color = Yellow;
            h0.Drink = Water;
            h0.Pet = Fox;
            h0.Smoke = Kools;
        }
        builders[1] = ukrainian;
        if (builders[1] is var h1)
        {
            // The Norwegian lives next to the blue house.
            h1.Color = Blue;
            // Kools are smoked in the house next to the house where the horse is kept.
            h1.Pet = Horse;
            // The man who smokes Chesterfields lives in the house next to the man with the fox.
            h1.Smoke = Chesterfields;
        }
        builders[2] = english;
        if (builders[2] is var h2)
        {
            // Milk is drunk in the middle house.
            h2.Drink = Milk;
            // The Old Gold smoker owns snails.
            h2.Pet = Snails;
            h2.Smoke = OldGold;
        }
        builders[3] = spanish;
        if (builders[3] is var h3)
        {
            // The green house is immediately to the right of the ivory house.
            h3.Color = Ivory;
            // The Lucky Strike smoker drinks orange juice.
            h3.Drink = OrangeJuice;
            h3.Smoke = LuckyStrike;
        }
        builders[4] = japanese;
        if (builders[4] is var h4)
        {
            // Coffee is drunk in the green house.
            h4.Color = Green;
            h4.Drink = Coffee;
            h4.Pet = Zebra;
        }

        var street = BuildStreet(builders);

        return street;
    }

    private static House[] BuildStreet(HouseBuilder[] builders)
    {
        House[] houses = new House[builders.Length];

        var failures = new List<Exception>();
        for (int i = 0; i < builders.Length; i++)
        {
            try
            {
                houses[i] = builders[i].Build();
            }
            catch (Exception e)
            {
                failures.Add(new InvalidOperationException($"Building house #{i} failed.", e));
            }
        }
        if (failures.Count > 0)
        {
            throw new AggregateException(failures);
        }
        return houses;
    }

    public class HouseBuilder()
    {
        private Color? _color;
        private Nationality? _nationality;
        private Pet? _pet;
        private Drink? _drink;
        private Smoke? _smoke;

        public Color? Color
        {
            get => _color;
            set
            {
                if (_color != null)
                {
                    throw new InvalidOperationException("Color already set");
                }
                _color = value;
            }
        }
        public Nationality? Nationality
        {
            get => _nationality;
            set
            {
                if (_nationality != null)
                {
                    throw new InvalidOperationException("Nationality already set");
                }
                _nationality = value;
            }
        }
        public Pet? Pet
        {
            get => _pet;
            set
            {
                if (_pet != null)
                {
                    throw new InvalidOperationException("Pet already set");
                }
                _pet = value;
            }
        }
        public Drink? Drink
        {
            get => _drink;
            set
            {
                if (_drink != null)
                {
                    throw new InvalidOperationException("Drink already set");
                }
                _drink = value;
            }
        }
        public Smoke? Smoke
        {
            get => _smoke;
            set
            {
                if (_smoke != null)
                {
                    throw new InvalidOperationException("Smoke already set");
                }
                _smoke = value;
            }
        }

        public House Build()
        {
            return new House(
                Color ?? throw new InvalidOperationException("Color missing"),
                Nationality ?? throw new InvalidOperationException("Nationality missing"),
                Pet ?? throw new InvalidOperationException("Pet missing"),
                Drink ?? throw new InvalidOperationException("Drink missing"),
                Smoke ?? throw new InvalidOperationException("Smoke missing")
            );
        }
    }

    public record House(Color Color, Nationality Nationality, Pet Pet, Drink Drink, Smoke Smoke)
    {

        public bool HasSameCharacteristics(House? other)
        {
            if (other == null)
            {
                return false;
            }

            return Color == other.Color ||
                Nationality == other.Nationality ||
                Pet == other.Pet ||
                Drink == other.Drink ||
                Smoke == other.Smoke;
        }
    }
}