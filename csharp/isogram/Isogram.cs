using System;

public static class Isogram
{
    public static bool IsIsogram(string word)
    {
        int bitField = 0;
        foreach (char letter in word)
        {
            if (letter is >= 'a' and <= 'z')
            {
                if ((bitField & 1 << (letter - 'a')) != 0)
                {
                    return false;
                }

                bitField |= 1 << (letter - 'a');
            }
            else if (letter is >= 'A' and <= 'Z')
            {
                if ((bitField & 1 << (letter - 'A')) != 0)
                {
                    return false;
                }

                bitField |= 1 << (letter - 'A');
            }
        }
        return true;
    }
}
