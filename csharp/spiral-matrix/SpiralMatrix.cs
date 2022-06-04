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

            var nextMove = position.Move(direction);
            while (
                // Overboard
                (nextMove.X < 0 || nextMove.X >= width || nextMove.Y < 0 || nextMove.Y >= height)
                // Already assigned
                || spiral[nextMove.X, nextMove.Y] != 0)
            {
                direction = direction.TurnRight();
                nextMove = position.Move(direction);
            }
            position = nextMove;
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

    public static (sbyte DX, sbyte DY) TurnRight(this (sbyte, sbyte) direction)
    { 
        return direction switch
        {
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            (0, -1) => (1, 0),
            _ => throw new NotSupportedException($"Direction {direction} is not supported.")
        };
    }
}
