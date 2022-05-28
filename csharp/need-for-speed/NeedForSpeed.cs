using System;

class RemoteControlCar
{
    private const int FullBatteryLevel = 100;

    private readonly int _speed;
    private readonly int _batteryDrain;

    private int _distanceDriven = 0;
    private int _batteryLevel = FullBatteryLevel;

    public RemoteControlCar(int speed, int batteryDrain)
    {
        _speed = speed;
        _batteryDrain = batteryDrain;
    }

    public bool BatteryDrained() => _batteryLevel < _batteryDrain;
    public int DistanceDriven() => _distanceDriven;

    public void Drive()
    {
        if (BatteryDrained()) { return; }

        _distanceDriven += _speed;
        _batteryLevel -= _batteryDrain;
    }

    public static RemoteControlCar Nitro() => new(speed: 50, batteryDrain: 4);
}

class RaceTrack
{
    private readonly int _distance;

    public RaceTrack(int distance)
    {
        _distance = distance;
    }

    public bool TryFinishTrack(RemoteControlCar car)
    {
        while (CanContinueDriving(car))
        {
            car.Drive();
        }
        return DidCarFinishTrack(car);
    }

    private bool CanContinueDriving(RemoteControlCar car) => car.DistanceDriven() < _distance && !car.BatteryDrained();
    private bool DidCarFinishTrack(RemoteControlCar car) => car.DistanceDriven() >= _distance;
}