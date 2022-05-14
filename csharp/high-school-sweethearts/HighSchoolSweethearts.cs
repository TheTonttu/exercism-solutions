using System;using System.Globalization;using System.Linq;public static class HighSchoolSweethearts{    public static string DisplaySingleLine(string studentA, string studentB)    {        const int lineLength = 61;        const string Centerpiece = " ♡ ";        Span<char> lineBuffer = stackalloc char[lineLength];        lineBuffer.Fill(' ');

        int writingLength = Centerpiece.Length + studentA.Length + studentB.Length;        int writingStartIndex = (lineBuffer.Length - writingLength - 1)/2;        var writingSection = lineBuffer.Slice(writingStartIndex, writingLength);

        WriteWriting(writingSection, studentA, studentB, Centerpiece);        return new string(lineBuffer);    }

    const string HeartBannerTemplate = @"     ******       ******   **      **   **      ** **         ** **         ****            *            ****                         ****{                       }** **                       **   **                   **     **               **       **           **         **       **           **   **             ***              *";    private static readonly int BannerWritingSectionStartIndex = HeartBannerTemplate.IndexOf('{');    private static readonly int BannerWritingSectionEndIndex = HeartBannerTemplate.IndexOf('}');    public static string DisplayBanner(string studentA, string studentB)    {        const string Centerpiece = "  +  ";        var trimmedStudentA = studentA.AsSpan().Trim();        var trimmedStudentB = studentB.AsSpan().Trim();        Span<char> bannerBuffer = stackalloc char[HeartBannerTemplate.Length];        HeartBannerTemplate.CopyTo(bannerBuffer);        bannerBuffer[BannerWritingSectionStartIndex] = ' ';        bannerBuffer[BannerWritingSectionEndIndex] = ' ';        int lineSectionLength = BannerWritingSectionEndIndex - BannerWritingSectionStartIndex + 1;        var lineSection = bannerBuffer.Slice(BannerWritingSectionStartIndex, lineSectionLength);        int writingLength = Centerpiece.Length + trimmedStudentA.Length + trimmedStudentB.Length;        int writingStartIndex = (lineSection.Length - writingLength + 1)/2;        var writingSection = lineSection.Slice(writingStartIndex, writingLength);

        WriteWriting(writingSection, trimmedStudentA, trimmedStudentB, Centerpiece);        return new string(bannerBuffer);    }    public static string DisplayGermanExchangeStudents(string studentA        , string studentB, DateTime start, float hours)    {        FormattableString text = $"{studentA} and {studentB} have been dating since {start:d} - that's {hours:n2} hours";        var germanCultureInfo = CultureInfo.GetCultureInfo("de-DE");        return text.ToString(germanCultureInfo);    }

    private static void WriteWriting(Span<char> writingSection, ReadOnlySpan<char> studentA, ReadOnlySpan<char> studentB, string centerpiece)
    {
        var studentASection = writingSection[..studentA.Length];        studentA.CopyTo(studentASection);        var centerpieceSection = writingSection[studentA.Length..^studentB.Length];        centerpiece.CopyTo(centerpieceSection);        var studentBSection = writingSection[^studentB.Length..];        studentB.CopyTo(studentBSection);
    }

    private static int RoundToEven(int number)
    {
        return (int)Math.Round(number / 2.0, MidpointRounding.ToZero) * 2;
    }
}