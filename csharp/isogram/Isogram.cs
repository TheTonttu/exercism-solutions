using System;

public static class Isogram
{
    public static bool IsIsogram(string word)
    {
        var bitField = new BitField();
        foreach (char letter in word)
        {
            int letterBitPosition;
            if (letter is >= 'a' and <= 'z')
            {
                letterBitPosition = letter - 'a';
            }
            else if (letter is >= 'A' and <= 'Z')
            {
                letterBitPosition = letter - 'A';
            } else
            {
                continue;
            }

            if (bitField.IsBitSet(letterBitPosition))
            {
                return false;
            }
            bitField.SetBit(letterBitPosition);
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

        public bool IsBitSet(in int position)
        {
            return (bits & 1 << position) != 0;
        }

        public void SetBit(in int position)
        {
            bits |= 1 << position;
        }
    }
}
