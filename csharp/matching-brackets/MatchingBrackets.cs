using System;
using System.Collections.Generic;

public static class MatchingBrackets
{
    public static bool IsPaired(string input)
    {
        var openBrackets = new Stack<char>();

        foreach (char c in input)
        {
            if (c == '{')
            {
                openBrackets.Push(c);
            }
            else if (c == '}')
            {
                if (!openBrackets.TryPop(out char open) || open != '{')
                {
                    return false;
                }
            }
            else if (c == '(')
            {
                openBrackets.Push(c);
            }
            else if (c == ')')
            {
                if (!openBrackets.TryPop(out char open) || open != '(')
                {
                    return false;
                }
            }
            else if (c == '[')
            {
                openBrackets.Push(c);
            }
            else if (c == ']')
            {
                if (!openBrackets.TryPop(out char open) || open != '[')
                {
                    return false;
                }
            }
        }
        return openBrackets.Count == 0;
    }
}
