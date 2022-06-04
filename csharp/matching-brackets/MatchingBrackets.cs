using System.Collections.Generic;

public static class MatchingBrackets
{
    public static bool IsPaired(string input)
    {
        var openingBrackets = new Stack<char>();

        foreach (char c in input)
        {
            if (c == '{')
            {
                openingBrackets.Push(c);
            }
            else if (c == '}')
            {
                if (!HasMatchingOpening('{'))
                {
                    return false;
                }
            }
            else if (c == '(')
            {
                openingBrackets.Push(c);
            }
            else if (c == ')')
            {
                if (!HasMatchingOpening('('))
                {
                    return false;
                }
            }
            else if (c == '[')
            {
                openingBrackets.Push(c);
            }
            else if (c == ']')
            {
                if (!HasMatchingOpening('['))
                {
                    return false;
                }
            }
        }
        return openingBrackets.Count == 0;

        bool HasMatchingOpening(char matchingOpen) => openingBrackets.TryPop(out char opening) && opening == matchingOpen;
    }
}
