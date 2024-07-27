using System;
using System.Collections.Generic;
using System.Diagnostics.CodeAnalysis;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Text;

using static Color;
using static Drink;
using static Nationality;
using static Pet;
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
        var h0 = builders[0] = norwegian;
        // Kools are smoked in the yellow house.
        h0.Color = Yellow;
        h0.Drink = Water;
        h0.Pet = Fox;
        h0.Smoke = Kools;

        var h1 = builders[1] = ukrainian;
        // The Norwegian lives next to the blue house.
        h1.Color = Blue;
        // Kools are smoked in the house next to the house where the horse is kept.
        h1.Pet = Horse;
        // The man who smokes Chesterfields lives in the house next to the man with the fox.
        h1.Smoke = Chesterfields;

        var h2 = builders[2] = english;
        // Milk is drunk in the middle house.
        h2.Drink = Milk;
        // The Old Gold smoker owns snails.
        h2.Pet = Snails;
        h2.Smoke = OldGold;

        var h3 = builders[3] = spanish;
        // The green house is immediately to the right of the ivory house.
        h3.Color = Ivory;
        // The Lucky Strike smoker drinks orange juice.
        h3.Drink = OrangeJuice;
        h3.Smoke = LuckyStrike;

        var h4 = builders[4] = japanese;
        // Coffee is drunk in the green house.
        h4.Color = Green;
        h4.Drink = Coffee;
        h4.Pet = Zebra;

        var street = BuildStreet(builders);

        return street;
    }

    private static House[] BuildStreet(HouseBuilder[] builders)
    {
        House[] houses = new House[builders.Length];

        var failures = new List<Exception>();
        var failureMessageBuilder = new StringBuilder();
        for (int i = 0; i < builders.Length; i++)
        {
            if (!builders[i].TryBuild(out House? house, out var missingProperties))
            {
                failureMessageBuilder.Clear();
                failureMessageBuilder.AppendLine($"Building house #{i} failed.");
                failureMessageBuilder.AppendLine($"Missing characteristics:");
                foreach (string propertyName in missingProperties)
                {
                    failureMessageBuilder.AppendLine($"- {propertyName}");
                }

                failures.Add(new InvalidOperationException(failureMessageBuilder.ToString()));
                continue;
            }
            houses[i] = house;
        }

        return failures.Count == 0
            ? houses
            : throw new AggregateException(failures);
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
                GuardNotSet(_color);
                _color = value;
            }
        }
        public Nationality? Nationality
        {
            get => _nationality;
            set
            {
                GuardNotSet(_nationality);
                _nationality = value;
            }
        }
        public Pet? Pet
        {
            get => _pet;
            set
            {
                GuardNotSet(_pet);
                _pet = value;
            }
        }
        public Drink? Drink
        {
            get => _drink;
            set
            {
                GuardNotSet(_drink);
                _drink = value;
            }
        }
        public Smoke? Smoke
        {
            get => _smoke;
            set
            {
                GuardNotSet(_smoke);
                _smoke = value;
            }
        }

        public bool TryBuild([NotNullWhen(true)] out House? house, out IEnumerable<string> missingProperties)
        {
            var missingPropertiesList = new List<string>();
            missingProperties = missingPropertiesList;

            if (Color == null)
            {
                missingPropertiesList.Add(nameof(House.Color));
            }

            if (Nationality == null)
            {
                missingPropertiesList.Add(nameof(House.Nationality));
            }

            if (Pet == null)
            {
                missingPropertiesList.Add(nameof(House.Pet));
            }

            if (Drink == null)
            {
                missingPropertiesList.Add(nameof(House.Drink));
            }

            if (Smoke == null)
            {
                missingPropertiesList.Add(nameof(House.Smoke));
            }

            if (missingPropertiesList.Count != 0)
            {
                house = null;
                return false;
            }

            house = new House(
                Color ?? throw new InvalidOperationException("Color missing"),
                Nationality ?? throw new InvalidOperationException("Nationality missing"),
                Pet ?? throw new InvalidOperationException("Pet missing"),
                Drink ?? throw new InvalidOperationException("Drink missing"),
                Smoke ?? throw new InvalidOperationException("Smoke missing")
            );
            return true;
        }

        private void GuardNotSet<T>(T? value, [CallerMemberName] string valueName = "")
        {
            if (value != null)
            {
                throw new InvalidOperationException($"{valueName} is already set");
            }
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