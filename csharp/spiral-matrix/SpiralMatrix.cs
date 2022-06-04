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
        var direction = (DX: 1, DY: 0);
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
    public static (int X, int Y) Move(this (int, int) position, (int, int) direction)
    {
        (int x, int y) = position;
        (int dX, int dY) = direction;
        return (x + dY, y + dX);
    }

    public static (int DX, int DY) TurnRight(this (int, int) direction)
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
