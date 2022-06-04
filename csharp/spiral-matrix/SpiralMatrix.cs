using System;

public class SpiralMatrix
{
    private static readonly int[,] ZeroMatrix = new int[0, 0];

    public static int[,] GetMatrix(int size)
    {
        if (size == 0) { return ZeroMatrix; }

        int width = size;
        int height = size;
        var spiral = new int[width,height];

        int valueLimit = size*size;
        int value = 1;
        (sbyte DX, sbyte DY) direction = (1, 0);
        var position = (X: 0, Y: 0);
        while (value < valueLimit)
        {
            spiral[position.X, position.Y] = value++;

            var nextPos = position.Move(direction);
            while (
                // Overboard
                (nextPos.X < 0 || nextPos.X >= width || nextPos.Y < 0 || nextPos.Y >= height)
                // Already assigned
                || spiral[nextPos.X, nextPos.Y] != 0)
            {
                direction = direction.TurnRight();
                nextPos = position.Move(direction);
            }
            position = nextPos;
        }
        var lastPosition = position;
        int lastValue = value;
        spiral[lastPosition.X, lastPosition.Y] = lastValue;

        return spiral;
    }
}

internal static class MovementTupleExtensions
{
    public static (int X, int Y) Move(this (int, int) position, (sbyte, sbyte) direction)
    {
        var (x, y) = position;
        var (dX, dY) = direction;
        return (x + dY, y + dX);
    }

    private static readonly (sbyte, sbyte) North = (0, -1);
    private static readonly (sbyte, sbyte) East = (1, 0);
    private static readonly (sbyte, sbyte) South = (0, 1);
    private static readonly (sbyte, sbyte) West = (-1, 0);

    public static (sbyte DX, sbyte DY) TurnRight(this (sbyte, sbyte) direction) =>
        direction switch
        {
            var n when n == North => East,
            var e when e == East => South,
            var s when s == South => West,
            var w when w == West => North,
            _ => throw new NotSupportedException($"Direction {direction} is not supported."),
        };
}
