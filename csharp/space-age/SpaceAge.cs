using System;
using System.Collections.Generic;

public class SpaceAge
{
    private const double EarthOrbitalPeriodInDays = 365.25;

    private readonly Dictionary<Planet, double> OrbitalPeriodMultipliers = new Dictionary<Planet, double>() {
        { Planet.Earth, 1 },
        { Planet.Mercury, 1/0.2408467 },
        { Planet.Venus, 1/0.61519726 },
        { Planet.Mars, 1/1.8808158 },
        { Planet.Jupiter, 1/11.862615 },
        { Planet.Saturn, 1/29.447498 },
        { Planet.Uranus, 1/84.016846 },
        { Planet.Neptune, 1/164.79132 },
    };

    private double _days;

    public SpaceAge(int seconds)
    {
        _days = TimeSpan.FromSeconds(seconds).TotalDays;
    }

    public double OnEarth()
    {
        return CalculateYears(Planet.Earth);
    }

    public double OnMercury()
    {
        return CalculateYears(Planet.Mercury);
    }

    public double OnVenus()
    {
        throw new NotImplementedException("You need to implement this function.");
    }

    public double OnMars()
    {
        throw new NotImplementedException("You need to implement this function.");
    }

    public double OnJupiter()
    {
        throw new NotImplementedException("You need to implement this function.");
    }

    public double OnSaturn()
    {
        throw new NotImplementedException("You need to implement this function.");
    }

    public double OnUranus()
    {
        throw new NotImplementedException("You need to implement this function.");
    }

    public double OnNeptune()
    {
        throw new NotImplementedException("You need to implement this function.");
    }

    private double CalculateYears(Planet planet)
    {
        return _days / EarthOrbitalPeriodInDays * OrbitalPeriodMultipliers[planet];
    }
}

public enum Planet
{
    Earth,
    Mercury,
    Venus,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune
}