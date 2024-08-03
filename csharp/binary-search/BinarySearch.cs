using System;

public static class BinarySearch
{
    public static int Find(int[] input, int value)
    {
        return Find(input.AsSpan(), value);
    }

    private static int Find(ReadOnlySpan<int> input, int value)
    {
        if (input.IsEmpty)
        {
            return -1;
        }

        int middlePointIndex = input.Length / 2;

        int middlePointValue = input[middlePointIndex];
        if (middlePointValue == value)
        {
            return middlePointIndex;
        }

        if (middlePointValue > value)
        {
            var left = input[..middlePointIndex];
            return Find(left, value);
        }

        var right = input[(middlePointIndex + 1)..];
        int rightIndex = Find(right, value);
        if (rightIndex > -1)
        {
            return middlePointIndex + 1 + rightIndex;
        }

        return -1;
    }
}