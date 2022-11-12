using System.Text;

public static class Acronym
{
    public static string Abbreviate(string phrase)
    {
        if (string.IsNullOrEmpty(phrase))
        {
            return string.Empty;
        }

        var acronymBuilder = new StringBuilder();

        bool pickNextLetter = true;
        foreach (char character in phrase)
        {
            if (IsLetter(character))
            {
                if (pickNextLetter)
                {
                    acronymBuilder.Append(char.ToUpperInvariant(character));
                    pickNextLetter = false;
                }
            }
            else if (!IsDiacriticalMark(character))
            {
                pickNextLetter = true;
            }
        }

        return acronymBuilder.ToString();
    }

    private static bool IsLetter(char character) => character is (>= 'a' and <= 'z') or (>= 'A' and <= 'Z');
    private static bool IsDiacriticalMark(char character) => character is '\'';
}