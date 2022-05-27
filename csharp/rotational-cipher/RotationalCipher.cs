using System;
using System.Runtime.CompilerServices;

public static class RotationalCipher
{
    private const string Alphabets = "abcdefghijklmnopqrstuvwxyz";

    public static string Rotate(string text, int shiftKey)
    {
        if (NoRotation(shiftKey)) { return text; }
        GuardShift(shiftKey);

        const int stackallocCharLimit = 1024/sizeof(char);
        Span<char> rotatedChars = text.Length <= stackallocCharLimit
            ? stackalloc char[text.Length]
            : new char[text.Length];
        for (int i = 0; i < rotatedChars.Length; i++)
        {
            char currChar = text[i];
            rotatedChars[i] = RotateChar(currChar, shiftKey);
        }

        return new string(rotatedChars);
    }

    private static bool NoRotation(int shiftKey) => shiftKey == 0 || shiftKey == Alphabets.Length;

    private static char RotateChar(char c, int shiftKey)
    {
        char normalizedChar;
        bool isUpper = false;
        if (c is >= 'a' and <= 'z')
        {
            normalizedChar = c;
        }
        else if (c is >= 'A' and <= 'Z')
        {
            normalizedChar = char.ToLowerInvariant(c);
            isUpper = true;
        }
        else
        {
            return c;
        }

        const int AsciiIndexOffset = 'a';

        int alphabetIndex = normalizedChar % AsciiIndexOffset;
        int rotatedIndex = (alphabetIndex + shiftKey) % Alphabets.Length;
        char rotatedChar = Alphabets[rotatedIndex];

        return isUpper
            ? char.ToUpperInvariant(rotatedChar)
            : rotatedChar;
    }

    private static void GuardShift(int shiftKey, [CallerArgumentExpression("shiftKey")] string paramName = "")
    {
        if (!IsShiftInRange(shiftKey))
        {
            throw new ArgumentOutOfRangeException(paramName);
        }
    }

    private static bool IsShiftInRange(int shiftKey) => 0 <= shiftKey && shiftKey <= Alphabets.Length;
}