public class RemoteControlCar
{
    private int _batteryPercentage = 100;
    private int _distanceDrivenInMeters = 0;
    private string[] _sponsors = new string[0];
    private int _latestSerialNum = 0;

    public void Drive()
    {
        if (_batteryPercentage > 0)
        {
            _batteryPercentage -= 10;
            _distanceDrivenInMeters += 2;
        }
    }

    public void SetSponsors(params string[] sponsors)
    {
        var defensiveCopy = (string[])sponsors.Clone();
        _sponsors = defensiveCopy;
    }

    public string DisplaySponsor(int sponsorNum)
    {
        return sponsorNum >= 0 && sponsorNum < _sponsors.Length
            ? _sponsors[sponsorNum]
            : string.Empty;
    }

    public bool GetTelemetryData(ref int serialNum,
        out int batteryPercentage, out int distanceDrivenInMeters)
    {
        if (serialNum < _latestSerialNum)
        {
            const int InvalidTelemetryData = -1;
            serialNum = _latestSerialNum;
            batteryPercentage = InvalidTelemetryData;
            distanceDrivenInMeters = InvalidTelemetryData;
            return false;
        }

        _latestSerialNum = serialNum;
        batteryPercentage = _batteryPercentage;
        distanceDrivenInMeters = _distanceDrivenInMeters;
        return true;
    }

    public static RemoteControlCar Buy()
    {
        return new RemoteControlCar();
    }
}

public class TelemetryClient
{

    private readonly RemoteControlCar _car;

    public TelemetryClient(RemoteControlCar car)
    {
        _car = car;
    }

    public string GetBatteryUsagePerMeter(int serialNum)
    {
        if (_car.GetTelemetryData(ref serialNum, out int batteryPercentage, out int driveDistanceInMeters)
            && driveDistanceInMeters > 0)
        {
            int batteryUsagePerMeter = CalculateBatteryUsagePerMeter(batteryPercentage, driveDistanceInMeters);
            return $"usage-per-meter={batteryUsagePerMeter}";
        }
        return "no data";
    }

    private static int CalculateBatteryUsagePerMeter(int batteryPercentage, int driveDistanceInMeters)
    {
        const int fullBatteryPercentage = 100;
        int batteryUsed = fullBatteryPercentage - batteryPercentage;
        return batteryUsed / driveDistanceInMeters;
    }
}
