using System;
using System.Collections.Generic;

public class SpaceAge
{
    private const double EarthOrbitalPeriodInDays = 365.25;

    private static readonly Dictionary<Planet, double> OrbitalPeriodMultipliers = new Dictionary<Planet, double>() {
        { Planet.Earth, 1 },
        { Planet.Mercury, 0.2408467 },
        { Planet.Venus, 0.61519726 },
        { Planet.Mars, 1.8808158 },
        { Planet.Jupiter, 11.862615 },
        { Planet.Saturn, 29.447498 },
        { Planet.Uranus, 84.016846 },
        { Planet.Neptune, 164.79132 },
    };

    private double _earthYears;

    public SpaceAge(int seconds)
    {
        _earthYears = TimeSpan.FromSeconds(seconds).TotalDays / EarthOrbitalPeriodInDays;
    }

    public double OnEarth()
    {
        return _earthYears;
    }

    public double OnMercury()
    {
        return CalculateYears(Planet.Mercury);
    }

    public double OnVenus()
    {
        return CalculateYears(Planet.Venus);
    }

    public double OnMars()
    {
        return CalculateYears(Planet.Mars);
    }

    public double OnJupiter()
    {
        return CalculateYears(Planet.Jupiter);
    }

    public double OnSaturn()
    {
        return CalculateYears(Planet.Saturn);
    }

    public double OnUranus()
    {
        return CalculateYears(Planet.Uranus);
    }

    public double OnNeptune()
    {
        return CalculateYears(Planet.Neptune);
    }

    private double CalculateYears(Planet planet)
    {
        return _earthYears / OrbitalPeriodMultipliers[planet];
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