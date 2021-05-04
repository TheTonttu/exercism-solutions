using System;
using System.Linq;
using System.Text;

public class Robot
{
    private static readonly Random RandomGenerator = new Random();

    private string m_name;

    public string Name
    {
        get
        {
            return m_name;
        }
    }

    public Robot()
    {
        m_name = GenerateRobotName();
    }

    public void Reset()
    {
        m_name = GenerateRobotName();
    }

    const string Alphabets = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

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
}