using System;

public class RemoteControlCar
{
    private Speed _currentSpeed;

    public string CurrentSponsor { get; private set; }
    public ITelemetry Telemetry { get; }

    public RemoteControlCar()
    {
        Telemetry = new InternalTelemetry(this);
    }

    public string GetSpeed()
    {
        return _currentSpeed.ToString();
    }

    private void SetSponsor(string sponsorName)
    {
        CurrentSponsor = sponsorName;
    }

    private void SetSpeed(Speed speed)
    {
        _currentSpeed = speed;
    }

    private class InternalTelemetry : ITelemetry
    {
        private readonly RemoteControlCar _car;

        public InternalTelemetry(RemoteControlCar car)
        {
            _car = car;
        }

        public void Calibrate()
        {
            // TODO: Implement calibration.
        }

        public bool SelfTest()
        {
            return true;
        }

        public void ShowSponsor(string sponsorName)
        {
            _car.SetSponsor(sponsorName);
        }

        public void SetSpeed(decimal amount, string unitsString)
        {
            SpeedUnits speedUnits = ChooseSpeedUnits(unitsString);
            _car.SetSpeed(new Speed(amount, speedUnits));
        }

        private static SpeedUnits ChooseSpeedUnits(string unitsString)
        {
            return unitsString switch
            {
                "cps" => SpeedUnits.CentimetersPerSecond,
                _ => SpeedUnits.MetersPerSecond,
            };
        }
    }

    private enum SpeedUnits
    {
        MetersPerSecond,
        CentimetersPerSecond
    }

    private readonly struct Speed
    {
        public decimal Amount { get; }
        public SpeedUnits SpeedUnits { get; }

        public Speed(decimal amount, SpeedUnits speedUnits)
        {
            Amount = amount;
            SpeedUnits = speedUnits;
        }

        public override string ToString()
        {
            return $"{Amount} {FormatUnits(SpeedUnits)}";
        }

        private static string FormatUnits(SpeedUnits speedUnits)
        {
            return speedUnits switch
            {
                SpeedUnits.CentimetersPerSecond => "centimeters per second",
                SpeedUnits.MetersPerSecond => "meters per second",
                _ => speedUnits.ToString()
            };
        }
    }
}

public interface ITelemetry
{
    void Calibrate();
    bool SelfTest();
    void SetSpeed(decimal amount, string unitsString);
    void ShowSponsor(string sponsorName);
}
