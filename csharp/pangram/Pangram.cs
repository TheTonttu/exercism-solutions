using System;

public static class Pangram
{
    // First 26 bits set to 1.
    // A-Z = 26 letters.
    private const int PangramMask = 67108863;

    public static bool IsPangram(string input)
    {
        if (string.IsNullOrEmpty(input))
        {
            return false;
        }

        int sentenceMask = 0;
        foreach (char letter in input)
        {
            if (letter is >= 'a' and <= 'z')
            {
                sentenceMask |= 1 << (letter - 'a');
            } else if (letter is >= 'A' and <= 'Z')
            {
                sentenceMask |= 1 << (letter - 'A');
            }
        }
        return sentenceMask == PangramMask;
    }
}
