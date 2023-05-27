using System.Text;

public static class RunLengthEncoding
{
    public static string Encode(string input)
    {
        int consecutives = 0;
        char character = default;
        var stringBuilder = new StringBuilder();
        foreach (char c in input)
        {
            if (character == c)
            {
                consecutives++;
            }
            else
            {
                EncodeRun(stringBuilder, consecutives, character);
                character = c;
                consecutives = 1;
            }
        }

        EncodeRun(stringBuilder, consecutives, character);

        return stringBuilder.ToString();

        static void EncodeRun(StringBuilder stringBuilder, int consecutives, char character)
        {
            if (consecutives > 1)
            {
                stringBuilder.Append(consecutives);
            }

            if (consecutives > 0)
            {
                stringBuilder.Append(character);
            }
        }
    }

    public static string Decode(string input)
    {
        int consecutives = 0;
        var stringBuilder = new StringBuilder();
        foreach (char c in input)
        {
            if (char.IsDigit(c))
            {
                int digit = (int)char.GetNumericValue(c);
                consecutives = (consecutives * 10) + digit;
            }
            else
            {
                int repetition = consecutives > 0 ? consecutives : 1;
                stringBuilder.Append(c, repetition);
                consecutives = 0;
            }
        }

        return stringBuilder.ToString();
    }
}
