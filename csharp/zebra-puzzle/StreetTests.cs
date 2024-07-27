using System;
using System.Linq;
using Xunit;

using static ZebraPuzzle;
using static Color;
using static Drink;
using static Nationality;
using static Pet;
using static Smoke;

public class StreetTests
{
    // There are five houses.
    [Fact]
    public void ThereAreFiveHouses()
    {
        var street = GenerateStreet();
        Assert.Equal(5, street.Length);
    }

    // The Englishman lives in the red house.
    [Fact]
    public void EnglishManLivesInRedHouse()
    {
        var street = GenerateStreet();
        var redHouse = street.First(h => h.Color == Red);
        Assert.Equal(Englishman, redHouse.Nationality);
    }

    // The Spaniard owns the dog.
    [Fact]
    public void SpaniardOwnsDog()
    {
        var street = GenerateStreet();
        var dogOwner = street.First(h => h.Pet == Dog);
        Assert.Equal(Spaniard, dogOwner.Nationality);
    }

    // Coffee is drunk in the green house.
    [Fact]
    public void CoffeeIsDrunkInGreenHouse()
    {
        var street = GenerateStreet();
        var greenHouse = street.First(h => h.Color == Green);
        Assert.Equal(Coffee, greenHouse.Drink);
    }

    // The green house is immediately to the right of the ivory house.
    [Fact]
    public void GreenHouseIsRightOfIvoryHouse()
    {
        var street = GenerateStreet();
        int ivoryHouseNumber = IndexOf(street, h => h.Color == Ivory);
        var houseRightOfIvoryHouse = street[ivoryHouseNumber+1];
        Assert.Equal(Green, houseRightOfIvoryHouse.Color);
    }

    // The Ukrainian drinks tea.
    [Fact]
    public void UkrainianDrinksTea()
    {
        var street = GenerateStreet();
        var teaDrinker = street.First(h => h.Drink == Tea);
        Assert.Equal(Ukranian, teaDrinker.Nationality);
    }

    // The Old Gold smoker owns snails.
    [Fact]
    public void OldGoldSmokerOwnsSnails()
    {
        var street = GenerateStreet();
        var snailOwner = street.First(h => h.Pet == Snails);
        Assert.Equal(OldGold, snailOwner.Smoke);
    }

    // Kools are smoked in the yellow house.
    [Fact]
    public void KoolsAreSmokedInYellowHouse()
    {
        var street = GenerateStreet();
        var yellowHouse = street.First(h => h.Color == Yellow);
        Assert.Equal(Kools, yellowHouse.Smoke);
    }

    // Milk is drunk in the middle house.
    [Fact]
    public void MilkIsDrunkInMiddleHouse()
    {
        var street = GenerateStreet();
        var middleHouse = street[street.Length/2];
        Assert.Equal(Milk, middleHouse.Drink);
    }

    // The Norwegian lives in the first house.
    [Fact]
    public void NorwegianLivesInFirstHouse()
    {
        var street= GenerateStreet();
        var firstHouse = street[0];
        Assert.Equal(Norwegian, firstHouse.Nationality);
    }

    // The man who smokes Chesterfields lives in the house next to the man with the fox.
    [Fact]
    public void ChesterfieldsSmokerLivesNextToFoxOwner()
    {
        var street = GenerateStreet();
        int chesterfieldsSmokerHouseNumber = IndexOf(street, h => h.Smoke == Chesterfields);
        int foxOwnerHouseNumber = IndexOf(street, h => h.Pet == Fox);

        Assert.True(LiveNextToEachOther(chesterfieldsSmokerHouseNumber, foxOwnerHouseNumber),
            $"""
            Chesterfields smoker should live next to fow owner.
            Chesterfields h#={chesterfieldsSmokerHouseNumber}, Fox h#={foxOwnerHouseNumber}
            """);

    }

    // Kools are smoked in the house next to the house where the horse is kept.
    [Fact]
    public void KoolsSmokerLivesNextToHorseOwner()
    {
        var street = GenerateStreet();
        int koolsSmokerHouseNumber = IndexOf(street, h => h.Smoke == Kools);
        int horseOwnerHouseNumber = IndexOf(street, h => h.Pet == Horse);

        Assert.True(LiveNextToEachOther(koolsSmokerHouseNumber, horseOwnerHouseNumber),
            $"""
            Kools smoker should live next to horse owner.
            Chesterfields h#={koolsSmokerHouseNumber}, Fox h#={horseOwnerHouseNumber}
            """);
    }

    // The Lucky Strike smoker drinks orange juice.
    [Fact]
    public void LuckyStrikeSmokerDrinksOrangeJuice()
    {
        var street = GenerateStreet();
        var orangeJuiceDrinker = street.First(h => h.Drink == OrangeJuice);
        Assert.Equal(LuckyStrike, orangeJuiceDrinker.Smoke);
    }

    // The Japanese smokes Parliaments.
    [Fact]
    public void JapaneseSmokesParliaments()
    {
        var street = GenerateStreet();
        var parliamentsSmoker = street.First(h => h.Smoke == Parliaments);
        Assert.Equal(Japanese, parliamentsSmoker.Nationality);
    }

    // The Norwegian lives next to the blue house.
    [Fact]
    public void NorwegianLivesNextToBlueHouse()
    {
        var street = GenerateStreet();
        var norwegianHouseNumber = IndexOf(street, h => h.Nationality == Norwegian);
        var blueHouseNumber = IndexOf(street, h => h.Color == Blue);
        Assert.Equal(blueHouseNumber, norwegianHouseNumber + 1);
    }

    [Fact]
    public void HousesAreUnique()
    {
        var street = GenerateStreet();
        AssertHouseUniqueness(street);
    }

    private bool LiveNextToEachOther(int houseNumber1, int houseNumber2) =>
        (houseNumber2 == houseNumber1 - 1) ||
        (houseNumber2 == houseNumber1 + 1);

    private static void AssertHouseUniqueness(House[] street)
    {
        for (int i = 0; i < street.Length; i++)
        {
            House house = street[i];
            for (int nhi = i + 1; nhi < street.Length; nhi++)
            {
                var otherHouse = street[nhi];
                Assert.False(house.HasSameCharacteristics(otherHouse), $"House {house} (h#{i}) and {otherHouse} (h#{nhi}) share characteristics.");
            }
        }
    }

    private static int IndexOf(House[] street, Predicate<House> predicate)
    {
        for (int i = 0; i <= street.Length; i++)
        {
            if (predicate(street[i]))
            {
                return i;
            }
        }
        return -1;
    }
}