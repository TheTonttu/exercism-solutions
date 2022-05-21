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

    public Outlook ShortTermOutlook
    {
        get
        {
            if (_latestReading.Equals(MissingReading))
            {
                throw new ArgumentException();
            }

            if (_latestReading.Pressure < 10m && _latestReading.Temperature < 30m)
            {
                return Outlook.Cool;
            }
            else if (_latestReading.Temperature > 50)
            {
                return Outlook.Good;
            }
            else
            {
                return Outlook.Warm;
            }
        }
    }

    public Outlook LongTermOutlook
    {
        get
        {
            if (_latestReading.WindDirection == WindDirection.Southerly
                || _latestReading.WindDirection == WindDirection.Easterly
                && _latestReading.Temperature > 20)
            {
                return Outlook.Good;
            }
            if (_latestReading.WindDirection == WindDirection.Northerly)
            {
                return Outlook.Cool;
            }
            if (_latestReading.WindDirection == WindDirection.Easterly
                && _latestReading.Temperature <= 20)
            {
                return Outlook.Warm;
            }
            if (_latestReading.WindDirection == WindDirection.Westerly)
            {
                return Outlook.Rainy;
            }
            throw new ArgumentException();
        }
    }

    public State RunSelfTest()
    {
        return _latestReading.Equals(MissingReading)
            ? State.Bad
            : State.Good;
    }
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
