using System.Collections.Generic;

public static class MatchingBrackets
{
    public static bool IsPaired(string input)
    {
        if (string.IsNullOrEmpty(input)) { return true; }

        var openingBrackets = new Stack<char>();
        foreach (char c in input)
        {
            if (IsClosingWithoutMatchingOpening(c)) { return false; }
            if (!IsOpening(c)) { continue; }
            openingBrackets.Push(c);
        }
        return openingBrackets.Count == 0;

        bool IsOpening(char c) => c is '{' or '(' or '[';

        bool IsClosingWithoutMatchingOpening(char c) =>
            c == '}' && !HasMatchingOpening('{') ||
            c == ')' && !HasMatchingOpening('(') ||
            c == ']' && !HasMatchingOpening('[');

        bool HasMatchingOpening(char matchingOpen) => openingBrackets.TryPop(out char opening) && opening == matchingOpen;
    }
}
