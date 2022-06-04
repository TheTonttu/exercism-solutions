using System;

public static class MatchingBrackets
{
    public static bool IsPaired(string input)
    {
        // {}
        int braceBalance = 0;
        // ()
        int parenthesisBalance = 0;
        // []
        int bracketBalance = 0;

        foreach (char c in input)
        {
            if (c == '{')
            {
                braceBalance++;
            }
            else if (c == '}')
            {
                braceBalance--;
            }
            else if (c == '(')
            {
                parenthesisBalance++;
            }
            else if (c == ')')
            {
                parenthesisBalance--;
            }
            else if (c == '[')
            {
                bracketBalance++;
            }
            else if (c == ']')
            {
                bracketBalance--;
            }
        }

        return braceBalance == 0
            && parenthesisBalance == 0
            && bracketBalance == 0;
    }
}
