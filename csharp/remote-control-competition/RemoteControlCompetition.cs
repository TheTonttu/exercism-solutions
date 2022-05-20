using System;
using System.Collections.Generic;

public interface IRemoteControlCar
{
    int DistanceTravelled { get; }

    void Drive();
}

public class ProductionRemoteControlCar : IRemoteControlCar, IComparable<ProductionRemoteControlCar>
{
    public int DistanceTravelled { get; private set; }
    public int NumberOfVictories { get; set; }

    public void Drive()
    {
        DistanceTravelled += 10;
    }

    public int CompareTo(ProductionRemoteControlCar other)
    {
        const int GreaterThan = 1;
        if (other == null) { return GreaterThan; }
        return this.NumberOfVictories - other.NumberOfVictories;
    }
}

public class ExperimentalRemoteControlCar : IRemoteControlCar
{
    public int DistanceTravelled { get; private set; }

    public void Drive()
    {
        DistanceTravelled += 20;
    }
}

public static class TestTrack
{
    public static void Race(IRemoteControlCar car)
    {
        car.Drive();
    }

    public static List<ProductionRemoteControlCar> GetRankedCars(
        ProductionRemoteControlCar prc1,
        ProductionRemoteControlCar prc2)
    {
        var rankingList = new List<ProductionRemoteControlCar>()
        {
            prc1, prc2
        };
        rankingList.Sort();

        return rankingList;
    }
}
