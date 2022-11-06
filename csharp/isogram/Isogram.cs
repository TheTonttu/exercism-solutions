using System;

public static class Isogram
{
    public static bool IsIsogram(string word)
    {
        var bitField = new BitField();
        foreach (char letter in word)
        {
            if (letter is >= 'a' and <= 'z')
            {
                int letterBitPosition = letter - 'a';
                if (bitField.IsSet(letterBitPosition))
                {
                    return false;
                }
                bitField.Set(letterBitPosition);
            }
            else if (letter is >= 'A' and <= 'Z')
            {
                int letterBitPosition = letter - 'A';
                if (bitField.IsSet(letterBitPosition))
                {
                    return false;
                }
                bitField.Set(letterBitPosition);
            }
        }
        return true;
    }

    private ref struct BitField
    {
        public int bits;

        public BitField()
        {
            bits = 0;
        }

        public bool IsSet(in int position)
        {
            return (bits & 1 << position) != 0;
        }

        public void Set(in int position)
        {
            bits |= 1 << position;
        }
    }
}
