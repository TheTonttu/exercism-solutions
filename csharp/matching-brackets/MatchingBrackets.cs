using System.Collections.Generic;

public static class MatchingBrackets
{
    public static bool IsPaired(string input)
    {
        var openingBrackets = new Stack<char>();

        foreach (char c in input)
        {
            if (IsClosingWithoutMatchingOpening(c)) { return false; }

            if (c == '{')
            {
                openingBrackets.Push(c);
            }
            else if (c == '(')
            {
                openingBrackets.Push(c);
            }
            else if (c == '[')
            {
                openingBrackets.Push(c);
            }
        }
        return openingBrackets.Count == 0;

        bool IsClosingWithoutMatchingOpening(char c) =>
            c == '}' && !HasMatchingOpening('{') ||
            c == ')' && !HasMatchingOpening('(') ||
            c == ']' && !HasMatchingOpening('[');

        bool HasMatchingOpening(char matchingOpen) => openingBrackets.TryPop(out char opening) && opening == matchingOpen;
    }
}
