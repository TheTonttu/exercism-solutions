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
        return CalculateAgeInPlanetYears(Planet.Mercury);
    }

    public double OnVenus()
    {
        return CalculateAgeInPlanetYears(Planet.Venus);
    }

    public double OnMars()
    {
        return CalculateAgeInPlanetYears(Planet.Mars);
    }

    public double OnJupiter()
    {
        return CalculateAgeInPlanetYears(Planet.Jupiter);
    }

    public double OnSaturn()
    {
        return CalculateAgeInPlanetYears(Planet.Saturn);
    }

    public double OnUranus()
    {
        return CalculateAgeInPlanetYears(Planet.Uranus);
    }

    public double OnNeptune()
    {
        return CalculateAgeInPlanetYears(Planet.Neptune);
    }

    private double CalculateAgeInPlanetYears(Planet planet)
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