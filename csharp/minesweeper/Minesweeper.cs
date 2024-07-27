using System.Text;

public static class Minesweeper
{
    private const char Mine = '*';

    private static readonly (int X, int Y)[] AdjacentOffsets =
    [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), /*(0, 0),*/ (1, 0),
        (-1, 1), (0, 1), (1, 1),
    ];

    public static string[] Annotate(string[] input)
    {
        string[] output = new string[input.Length];
        var columnBuilder = new StringBuilder();
        for (int y = 0; y < input.Length; y++)
        {
            columnBuilder.Clear();
            string inputColumn = input[y];
            for (int x = 0; x < inputColumn.Length; x++)
            {
                char cell = inputColumn[x];
                if (cell == Mine)
                {
                    columnBuilder.Append(cell);
                    continue;
                }

                int adjacentMines = CountAdjacentMines(input, x, y);
                columnBuilder.Append(AnnotateCell(adjacentMines));
            }
            output[y] = columnBuilder.ToString();
        }
        return output;
    }

    private static char AnnotateCell(int adjacentMines) =>
        adjacentMines switch
        {
            0 => ' ',
            _ => (char)('0' + adjacentMines),
        };

    private static int CountAdjacentMines(string[] minefield, int x, int y)
    {
        int width = minefield[0].Length;
        int height = minefield.Length;

        int adjacentMines = 0;
        foreach ((int xOffset, int yOffset) in AdjacentOffsets)
        {
            int xD = x + xOffset;
            int yD = y + yOffset;
            if (InsideField(xD, yD, width, height) &&
                minefield[yD][xD] is char adjacentCell &&
                adjacentCell == Mine)
            {
                adjacentMines++;
            }
        }
        return adjacentMines;
    }

    private static bool InsideField(int x, int y, int width, int height) =>
        0 <= x && x < width &&
        0 <= y && y < height;
}
