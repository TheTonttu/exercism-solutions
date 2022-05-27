using System;

public static class RotationalCipher
{
    private const int ShiftLimit = 26;

    public static string Rotate(string text, int shiftKey)
    {
        shiftKey = Math.Abs(shiftKey);
        if (NoRotation(shiftKey)) { return text; }

        const int stackallocCharLimit = 1024/sizeof(char);
        Span<char> rotatedChars = text.Length <= stackallocCharLimit
            ? stackalloc char[text.Length]
            : new char[text.Length];
        for (int i = 0; i < rotatedChars.Length; i++)
        {
            rotatedChars[i] = RotateLetter(text[i], shiftKey);
        }
        return new string(rotatedChars);
    }

    private static bool NoRotation(int shiftKey) => shiftKey % ShiftLimit == 0;

    private static char RotateLetter(char c, int shiftKey)
    {
        int letterOffset;
        if (c is >= 'a' and <= 'z')
        {
            letterOffset = 'a';
        }
        else if (c is >= 'A' and <= 'Z')
        {
            letterOffset = 'A';
        }
        else
        {
            return c;
        }

        char rotatedChar = (char)(letterOffset + (c - letterOffset + shiftKey) % ShiftLimit);
        return rotatedChar;
    }
}