using System;
using static Direction;

public class SpiralMatrix
{
    private static readonly int[,] ZeroMatrix = new int[0, 0];

    public static int[,] GetMatrix(int size)
    {
        if (size == 0) { return ZeroMatrix; }

        int width = size;
        int height = size;
        // [row, col]
        var spiral = new int[height,width];

        int maxValue = height*width;
        int value = 1;
        var direction = East;
        var position = (X: 0, Y: 0);
        while (value < maxValue)
        {
            spiral[position.Y, position.X] = value++;

            var nextPos = position.Move(direction);
            while (IsMoveBlocked(nextPos))
            {
                direction = direction.TurnRight();
                nextPos = position.Move(direction);
            }
            position = nextPos;
        }
        var lastPosition = position;
        int lastValue = value;
        spiral[lastPosition.Y, lastPosition.X] = lastValue;

        return spiral;

        bool IsMoveBlocked((int X, int Y) movePos) =>
            // Overboard
            (movePos.X < 0 || movePos.X >= width || movePos.Y < 0 || movePos.Y >= height)
            // Already assigned
            || spiral[movePos.Y, movePos.X] != 0;
    }
}

internal static class Direction
{
    public static readonly (sbyte DX, sbyte DY) North = (0, -1);
    public static readonly (sbyte DX, sbyte DY) East = (1, 0);
    public static readonly (sbyte DX, sbyte DY) South = (0, 1);
    public static readonly (sbyte DX, sbyte DY) West = (-1, 0);
}

internal static class MovementTupleExtensions
{

    public static (int X, int Y) Move(this (int, int) position, (sbyte, sbyte) direction)
    {
        var (x, y) = position;
        var (dX, dY) = direction;
        return (x + dX, y + dY);
    }


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
