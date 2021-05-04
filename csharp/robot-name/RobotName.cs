using System;
using System.Collections.Generic;
using System.Text;

public class Robot : IDisposable
{
    private static readonly Random RandomGenerator = new Random();
    private static readonly HashSet<int> NameHashRegistry = new HashSet<int>();
    private bool _isDisposed;

    public string Name { get; private set; }

    public Robot()
    {
        ReplaceName();
    }

    public void Reset()
    {
        if (_isDisposed) { throw new ObjectDisposedException("Cannot reset disposed robot."); }

        ReplaceName();
    }

    const string Alphabets = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    private void ReplaceName()
    {
        string newName;
        do
        {
            newName = GenerateRobotName();
        } while (!NameHashRegistry.Add(newName.GetHashCode()));

        int oldNameHash = Name?.GetHashCode() ?? default;
        NameHashRegistry.Remove(oldNameHash);
        Name = newName;
    }

    private string GenerateRobotName()
    {
        const int LetterCount = 2;
        const int NumberCount = 3;

        var nameBuilder = new StringBuilder();
        for (int i = 0; i < LetterCount; i++)
        {
            int randomIndex = RandomGenerator.Next(0, Alphabets.Length - 1);
            char randomChar = Alphabets[randomIndex];
            nameBuilder.Append(randomChar);
        }

        for (int i = 0; i < NumberCount; i++)
        {
            int randomNumber = RandomGenerator.Next(0, 9);
            nameBuilder.Append(randomNumber);
        }

        return nameBuilder.ToString();
    }

    #region IDispoable Members

    protected virtual void Dispose(bool disposing)
    {
        if (!_isDisposed)
        {
            if (disposing)
            {
                if (Name != null)
                {
                    // Unregister name once the robot is out of scope and collected. Otherwise name registry will just keep collecting unused names.
                    NameHashRegistry.Remove(Name.GetHashCode());
                    Name = null;
                }
            }
            _isDisposed = true;
        }
    }

    public void Dispose()
    {
        // Do not change this code. Put cleanup code in 'Dispose(bool disposing)' method
        Dispose(disposing: true);
        GC.SuppressFinalize(this);
    }

    #endregion
}