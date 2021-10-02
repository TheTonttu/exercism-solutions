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

    #region Additional unit tests

    [Theory]
    // Delimiter, no match
    [InlineData("I am not the 1st test", "2nd", "")]
    // Delimiter, empty
    [InlineData("I am not the 1st test", "", "I am not the 1st test")]
    // Line, null
    [InlineData(null, ":", null)]
    // Line, empty
    [InlineData("", ":", "")]
    public void SubstringAfter_SafeScenarios(string line, string delimiter, string expectedSubstring)
    {
        string substring = line.SubstringAfter(delimiter);

        if (expectedSubstring == null)
        {
            Assert.Null(substring);
        }
        else
        {
            Assert.Equal(expectedSubstring, substring);
        }
    }

    [Theory]
    // Delimiter, null
    [InlineData("I am not the 1st test", null, typeof(ArgumentNullException))]
    public void SubstringAfter_ErrorScenarios(string line, string delimiter, Type expectedExceptionType)
    {
        Assert.Throws(expectedExceptionType, () => line.SubstringAfter(delimiter));
    }


    [Theory]
    // Start delimiter, no match
    [InlineData("[INFO]: File Deleted.", "{", "]", "")]
    // Start delimiter, empty
    [InlineData("[INFO]: File Deleted.", "", "]", "[INFO")]
    // End delimiter, no match
    [InlineData("[INFO]: File Deleted.", "[", "}", "")]
    // End delimiter, empty
    [InlineData("[INFO]: File Deleted.", "[", "", "")]
    // Delimiters, empty
    [InlineData("[INFO]: File Deleted.", "", "", "")]
    // Line, null
    [InlineData(null, "[", "]", null)]
    // Line, empty
    [InlineData("", "[", "]", "")]
    public void SubstringBetween_SafeScenarios(string line, string startDelimiter, string endDelimiter, string expectedSubstring)
    {
        string substring = line.SubstringBetween(startDelimiter, endDelimiter);

        if (expectedSubstring == null)
        {
            Assert.Null(substring);
        }
        else
        {
            Assert.Equal(expectedSubstring, substring);
        }
    }

    [Theory]
    // Start delimiter, null
    [InlineData("[INFO]: File Deleted.", null, "]", typeof(ArgumentNullException))]
    // End delimiter, null
    [InlineData("[INFO]: File Deleted.", "[", null, typeof(ArgumentNullException))]
    public void SubstringBetween_ErrorScenarios(string line, string startDelimiter, string endDelimiter, Type expectedExceptionType)
    {
        Assert.Throws(expectedExceptionType, () => line.SubstringBetween(startDelimiter, endDelimiter));
    }


    [Theory]
    // Surrounding whitespace is trimmed from message
    [InlineData("[INFO]:  File Deleted. ", "File Deleted.")]
    [InlineData("[INFO]: \n \r\t File Deleted. \t\r \n", "File Deleted.")]
    // Log line, null => message, null
    [InlineData(null, null)]
    // Log line, empty => message, empty
    [InlineData("", "")]
    public void MessageScenarios(string logLine, string expectedMessage)
    {
        string message = logLine.Message();

        if (expectedMessage == null)
        {
            Assert.Null(message);
        }
        else
        {
            Assert.Equal(expectedMessage, message);
        }
    }


    [Theory]
    // Log line, null => log level, null
    [InlineData(null, null)]
    // Log line, empty => log level, empty
    [InlineData("", "")]
    public void LogLevelScenarios(string logLine, string expectedLogLevel)
    {
        string logLevel = logLine.LogLevel();

        if (expectedLogLevel == null)
        {
            Assert.Null(logLevel);
        }
        else
        {
            Assert.Equal(expectedLogLevel, logLevel);
        }
    }

    #endregion
}
