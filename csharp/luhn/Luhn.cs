using System;

public static class Luhn
{
    public static bool IsValid(string number)
    {
        ReadOnlySpan<char> trimmedNumber = number.AsSpan().Trim();
        if (trimmedNumber.Length < 2)
        {
            return false;
        }

        bool doubleDigit = false;
        int sum = 0;
        for (int i = trimmedNumber.Length - 1; i >= 0; i--)
        {
            char c = trimmedNumber[i];
            if (c is >= '0' and <= '9')
            {
                int digit = c - '0';
                if (doubleDigit)
                {
                    digit *= 2;
                    if (digit > 9)
                    {
                        digit -= 9;
                    }
                }
                sum += digit;
                doubleDigit = !doubleDigit;
            }
            else if (c != ' ')
            {
                return false;
            }
        }
        return sum % 10 == 0;
    }
}