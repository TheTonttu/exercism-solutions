using System;
using System.Collections.Generic;
using System.Text;

public class Robot : IDisposable
{
    private static readonly RobotNameGenerator NameGenerator = new RobotNameGenerator();

    // Not sure if this has more hash collisions compared to storing the strings directly.
    // Should be more memory efficient due to keeping track of integers instead of the whole 5 char strings.
    private static readonly HashSet<int> NameHashRegistry = new HashSet<int>();

    private bool _isDisposed;

    public string Name { get; private set; }

    public Robot()
    {
        ChangeName();
    }

    public void Reset()
    {
        if (_isDisposed) { throw new ObjectDisposedException("Cannot reset disposed robot."); }
        ChangeName();
    }

    private void ChangeName()
    {
        string newName;
        do
        {
            newName = NameGenerator.Generate();
        } while (!NameHashRegistry.Add(newName.GetHashCode()));

        int oldNameHash = Name?.GetHashCode() ?? default;
        NameHashRegistry.Remove(oldNameHash);
        Name = newName;
    }

    #region IDisposable Members

    protected virtual void Dispose(bool disposing)
    {
        if (!_isDisposed)
        {
            if (disposing)
            {
                if (Name != null)
                {
                    // Unregister name once the robot is out of scope and collected. Otherwise registry is filled with unused names.
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

public class RobotNameGenerator
{
    private static readonly Random RandomGenerator = new Random();

    public string Generate()
    {
        const int LetterCount = 2;
        const int NumberCount = 3;

        var nameBuilder = new StringBuilder();
        for (int i = 0; i < LetterCount; i++)
        {
            char randomLetter = GenerateRandomLetter();
            nameBuilder.Append(randomLetter);
        }

        for (int i = 0; i < NumberCount; i++)
        {
            int randomNumber = RandomGenerator.Next(10);
            nameBuilder.Append(randomNumber);
        }

        return nameBuilder.ToString();
    }

    private char GenerateRandomLetter()
    {
        const int AsciiUpperCaseLetterSectionLength = 26;
        char randomLetter = (char)('A' + RandomGenerator.Next(AsciiUpperCaseLetterSectionLength));
        return randomLetter;
    }
}