using System.Linq;
using System.Text.RegularExpressions;

public class LogParser
{
    private static readonly Regex ValidLineRegex = new(@"^\[(TRC|DBG|INF|WRN|ERR|FTL)\].*$", RegexOptions.Compiled);
    private static readonly Regex SplitRegex = new(@"<[\^\*=-]+>", RegexOptions.Compiled);
    private static readonly Regex QuotedPasswordCountRegex = new("\"[^\"]*password[^\"]*\"", RegexOptions.Compiled | RegexOptions.IgnoreCase);
    private static readonly Regex EndOfLineRemoveRegex = new("end-of-line[0-9]+", RegexOptions.Compiled);
    private static readonly Regex PasswordMatchRegex = new(@"password\S+", RegexOptions.Compiled | RegexOptions.IgnoreCase);

    public bool IsValidLine(string text)
    {
        return ValidLineRegex.IsMatch(text);
    }

    public string[] SplitLogLine(string text)
    {
        return SplitRegex.Split(text);
    }

    public int CountQuotedPasswords(string lines)
    {
        var matches = QuotedPasswordCountRegex.Matches(lines);
        return matches.Count;
    }

    public string RemoveEndOfLineText(string line)
    {
        return EndOfLineRemoveRegex.Replace(line, string.Empty);
    }

    public string[] ListLinesWithPasswords(string[] lines)
    {
        return lines
            .Select(ProcessLine)
            .ToArray();
    }

    private static string ProcessLine(string line)
    {
        var match = PasswordMatchRegex.Match(line);
        string password = match.Value;
        return match.Success
            ? $"{password}: {line}"
            : $"--------: {line}";
    }
}
