using System;
using System.Globalization;

public static class HighSchoolSweethearts
{

    public static string DisplaySingleLine(string studentA, string studentB)
    {
        const int lineLength = 61;
        const char centerpiece = '♡';
        const int centerpieceIndex = lineLength / 2;
        const int nameOffsetFromCenter = 2;

        Span<char> lineBuffer = stackalloc char[lineLength];
        lineBuffer.Fill(' ');
        lineBuffer[centerpieceIndex] = centerpiece;

        WriteNamesAroundCenter(lineBuffer, studentA, studentB, centerpieceIndex, nameOffsetFromCenter);

        return new string(lineBuffer);
    }

    private const string BannerTemplate = @"
     ******       ******
   **      **   **      **
 **         ** **         **
**            *            **
**                         **
**            +            **
 **                       **
   **                   **
     **               **
       **           **
         **       **
           **   **
             ***
              *
";

    private static readonly int CenterpieceIndex = BannerTemplate.IndexOf('+');
    private static readonly int BannerNameOffsetFromCenter = 3;
    private static readonly int BannerLeftNameIndex = CenterpieceIndex - BannerNameOffsetFromCenter;
    private static readonly int BannerRightNameStartIndex = CenterpieceIndex + BannerNameOffsetFromCenter;

    public static string DisplayBanner(string studentA, string studentB)
    {
        var trimmedStudentA = studentA.AsSpan().Trim();
        var trimmedStudentB = studentB.AsSpan().Trim();

        Span<char> bannerBuffer = stackalloc char[BannerTemplate.Length];
        BannerTemplate.CopyTo(bannerBuffer);

        WriteNamesAroundCenter(bannerBuffer, trimmedStudentA, trimmedStudentB, CenterpieceIndex, BannerNameOffsetFromCenter);

        return new string(bannerBuffer);
    }

    public static string DisplayGermanExchangeStudents(string studentA
        , string studentB, DateTime start, float hours)
    {
        FormattableString text = $"{studentA} and {studentB} have been dating since {start:d} - that's {hours:n2} hours";
        var germanCultureInfo = CultureInfo.GetCultureInfo("de-DE");
        return text.ToString(germanCultureInfo);
    }

    private static void WriteNamesAroundCenter(Span<char> writingSpace, ReadOnlySpan<char> leftName, ReadOnlySpan<char> rightName, int centerIndex, int nameOffsetFromCenter)
    {
        var leftNameSection = GetLeftNameSection(writingSpace, leftName, centerIndex, nameOffsetFromCenter);
        leftName.CopyTo(leftNameSection);

        var rightNameSection = GetRightNameSection(writingSpace, rightName, centerIndex, nameOffsetFromCenter);
        rightName.CopyTo(rightNameSection);
    }

    private static Span<char> GetLeftNameSection(Span<char> writingSpace, ReadOnlySpan<char> name, int centerIndex, int nameOffsetFromCenter)
    {
        int leftNameStartIndex = centerIndex - nameOffsetFromCenter - (name.Length - 1);
        var leftNameSection = writingSpace.Slice(leftNameStartIndex, name.Length);
        return leftNameSection;
    }

    private static Span<char> GetRightNameSection(Span<char> writingSpace, ReadOnlySpan<char> name, int centerIndex, int nameOffsetFromCenter)
    {
        int rightNameStartIndex = centerIndex + nameOffsetFromCenter;
        var rightNameSection = writingSpace.Slice(rightNameStartIndex, name.Length);
        return rightNameSection;
    }
}
