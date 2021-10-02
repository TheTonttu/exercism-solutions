using System;

class RemoteControlCar
{
    private int _batteryPercentage = 100;
    private int _driveDistanceInMeters = 0;

    public static RemoteControlCar Buy()
    {
        return new RemoteControlCar();
    }

    public string DistanceDisplay()
    {
        return $"Driven {_driveDistanceInMeters} meters";
    }

    public string BatteryDisplay()
    {
        if (IsBatteryEmpty()) { return "Battery empty"; }

        return $"Battery at {_batteryPercentage}%";
    }

    public void Drive()
    {
        if (IsBatteryEmpty()) { return; }

        _batteryPercentage -= 1;
        _driveDistanceInMeters += 20;
    }

    private bool IsBatteryEmpty()
    {
        return _batteryPercentage <= 0;
    }
}
