using Xunit;
using Exercism.Tests;
using System;

public class LogAnalysisTests
{
    [Fact]
    public void SubstringAfter_WithDelimeterOfLength1()
    {
        Assert.Equal(" am the 1st test", "I am the 1st test".SubstringAfter("I"));
    }

    [Fact]
    public void SubstringAfter_WithDelimeterOfLengthLongerThan1()
    {
        Assert.Equal(" test", "I am the 2nd test".SubstringAfter("2nd"));
    }

    [Fact]
    public void SubstringBetween()
    {
        Assert.Equal("INFO", "[INFO]: File Deleted.".SubstringBetween("[", "]"));
    }

    [Fact]
    public void Message()
    {
        var log = "[WARNING]: Library is deprecated.";
        Assert.Equal("Library is deprecated.", log.Message());
    }

    [Fact]
    public void LogLevel()
    {
        var log = "[WARNING]: Library is deprecated.";
        Assert.Equal("WARNING", log.LogLevel()); ;
    }

    #region Own unit tests

    [Fact]
    public void SubstringAfter_NoDelimiterMatch()
    {
        Assert.Equal(String.Empty, "I am not the 1st test".SubstringAfter("bonk"));
    }

    [Fact]
    public void SubstringAfter_DelimiterIsNull()
    {
        Assert.Throws<ArgumentNullException>("delimiter", () => "I am not the 1st test".SubstringAfter(null));
    }

    [Fact]
    public void SubstringAfter_EmptyDelimiter()
    {
        Assert.Equal("I am not the 1st test", "I am not the 1st test".SubstringAfter(String.Empty));
    }

    [Fact]
    public void SubstringBetween_NoStartDelimiterMatch()
    {
        Assert.Equal(String.Empty, "{INFO]: File Deleted.".SubstringBetween("[", "]"));
    }

    [Fact]
    public void SubstringBetween_StartDelimiterIsNull()
    {
        Assert.Throws<ArgumentNullException>("startDelimiter", () => "[INFO]: File Deleted.".SubstringBetween(null, "]"));
    }

    [Fact]
    public void SubstringBetween_EmptyStartDelimiterMatch()
    {
        Assert.Equal("[INFO", "[INFO]: File Deleted.".SubstringBetween(String.Empty, "]"));
    }

    [Fact]
    public void SubstringBetween_NoEndDelimiterMatch()
    {
        Assert.Equal(String.Empty, "[INFO}: File Deleted.".SubstringBetween("[", "]"));
    }

    [Fact]
    public void SubstringBetween_EndDelimiterIsNull()
    {
        Assert.Throws<ArgumentNullException>("endDelimiter", () => "[INFO]: File Deleted.".SubstringBetween("[", null));
    }

    [Fact]
    public void SubstringBetween_EmptyEndDelimiter()
    {
        Assert.Equal(String.Empty, "[INFO]: File Deleted.".SubstringBetween("[", String.Empty));
    }

    [Fact]
    public void SubstringBetween_EmptyDelimiters()
    {
        Assert.Equal(String.Empty, "[INFO]: File Deleted.".SubstringBetween(String.Empty, String.Empty));
    }

    [Fact]
    public void Message_LineIsNull()
    {
        string log = null;
        Assert.Equal(String.Empty, log.Message());
    }

    [Fact]
    public void LogLevel_LineIsNull()
    {
        string log = null;
        Assert.Equal(String.Empty, log.LogLevel());
    }

    #endregion
}
