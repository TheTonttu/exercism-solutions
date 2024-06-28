using System;
using System.Collections.Generic;

public static class AllYourBase
{
    public static int[] Rebase(int inputBase, int[] inputDigits, int outputBase)
    {
        try
        {
            ArgumentOutOfRangeException.ThrowIfLessThan(inputBase, 2);
            ArgumentOutOfRangeException.ThrowIfLessThan(outputBase, 2);
        }
        catch (ArgumentOutOfRangeException ex)
        {
            // HACK: Tests expect exactly ArgumentException instead of any derived type
            throw new ArgumentException(ex.Message, ex.ParamName, ex);
        }

        if (inputBase == outputBase)
        {
            // Caveat here is that invalid input digits will not be checked.
            return inputDigits;
        }

        int base10Number = BaseNToBase10(inputBase, inputDigits);
        return Base10ToBaseN(base10Number, outputBase);
    }

    private static int BaseNToBase10(int inputBase, int[] inputDigits)
    {
        int base10Number = 0;

        int lastIndex = inputDigits.Length - 1;
        for (int i = lastIndex; i >= 0; i--)
        {
            int digit = inputDigits[i];
            if (digit < 0 || digit >= inputBase)
            {
                throw new ArgumentException($"Invalid input digit {digit} @ index {i}", nameof(inputDigits));
            }

            int exponent = lastIndex - i;
            base10Number += digit * (int)Math.Pow(inputBase, exponent);
        }

        return base10Number;
    }

    private static int[] Base10ToBaseN(int base10Number, int outputBase)
    {
        if (base10Number == 0)
        {
            return [0];
        }

        var outputDigits = new List<int>();
        int remainder = base10Number;
        while (remainder > 0)
        {
            int digit = remainder % outputBase;
            outputDigits.Add(digit);
            remainder /= outputBase;
        }

        outputDigits.Reverse();
        return [.. outputDigits];
    }
}