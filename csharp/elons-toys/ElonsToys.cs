using System;

class RemoteControlCar
{
    private int _batteryPercentage = 100;
    private int _driveDistanceInMeters = 0;

    public static RemoteControlCar Buy() => new();

    public string DistanceDisplay() =>
        $"Driven {_driveDistanceInMeters} meters";

    public string BatteryDisplay() =>
        IsBatteryEmpty()
        ? "Battery empty"
        : $"Battery at {_batteryPercentage}%";

    public void Drive()
    {
        if (IsBatteryEmpty()) { return; }

        _batteryPercentage -= 1;
        _driveDistanceInMeters += 20;
    }

    private bool IsBatteryEmpty() =>
        _batteryPercentage <= 0;
}
