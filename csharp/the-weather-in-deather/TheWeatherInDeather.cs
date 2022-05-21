using System;
using System.Collections.Generic;

public class WeatherStation
{
    private static readonly Reading MissingReading = new();

    private readonly List<DateTime> _recordDates = new();
    private readonly List<decimal> _temperatures = new();
    private Reading _latestReading;

    public void AcceptReading(Reading reading)
    {
        _latestReading = reading;
        _recordDates.Add(DateTime.Now);
        _temperatures.Add(reading.Temperature);
    }

    public void ClearAll()
    {
        _latestReading = MissingReading;
        _recordDates.Clear();
        _temperatures.Clear();
    }

    public decimal LatestTemperature => _latestReading.Temperature;
    public decimal LatestPressure => _latestReading.Pressure;
    public decimal LatestRainfall => _latestReading.Rainfall;
    public bool HasHistory => _recordDates.Count > 1;

    public Outlook ShortTermOutlook =>
        IsLatestReadingMissing()
            ? throw new ArgumentException()
            : ShortTermOutlookFromLatestReading();

    public Outlook LongTermOutlook =>
        (_latestReading.WindDirection, _latestReading.Temperature) switch
        {
            (WindDirection.Southerly, _) => Outlook.Good,
            (WindDirection.Northerly, _) => Outlook.Cool,
            (WindDirection.Westerly, _) => Outlook.Rainy,
            (WindDirection.Easterly, > 20m) => Outlook.Good,
            (WindDirection.Easterly, <= 20m) => Outlook.Warm,
            _ => throw new ArgumentException()
        };

    public State RunSelfTest() =>
        IsLatestReadingMissing()
            ? State.Bad
            : State.Good;

    private bool IsLatestReadingMissing() =>
        _latestReading.Equals(MissingReading);

    private Outlook ShortTermOutlookFromLatestReading() =>
        (_latestReading.Pressure, _latestReading.Temperature) switch
        {
            ( < 10m, < 30m) => Outlook.Cool,
            (_, > 50m) => Outlook.Good,
            _ => Outlook.Warm
        };
}

/*** Please do not modify this struct ***/
public struct Reading
{
    public decimal Temperature { get; }
    public decimal Pressure { get; }
    public decimal Rainfall { get; }
    public WindDirection WindDirection { get; }

    public Reading(decimal temperature, decimal pressure,
        decimal rainfall, WindDirection windDirection)
    {
        Temperature = temperature;
        Pressure = pressure;
        Rainfall = rainfall;
        WindDirection = windDirection;
    }
}

/*** Please do not modify this enum ***/
public enum State
{
    Good,
    Bad
}

/*** Please do not modify this enum ***/
public enum Outlook
{
    Cool,
    Rainy,
    Warm,
    Good
}

/*** Please do not modify this enum ***/
public enum WindDirection
{
    Unknown, // default
    Northerly,
    Easterly,
    Southerly,
    Westerly
}
