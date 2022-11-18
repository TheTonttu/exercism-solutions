using Xunit;

public class AnagramTests
{
    [Fact]
    public void No_matches()
    {
        var candidates = new[] { "hello", "world", "zombies", "pants" };
        var sut = new Anagram("diaper");
        Assert.Empty(sut.FindAnagrams(candidates));
    }

    [Fact]
    public void Detects_two_anagrams()
    {
        var candidates = new[] { "lemons", "cherry", "melons" };
        var sut = new Anagram("solemn");
        var expected = new[] { "lemons", "melons" };
        Assert.Equal(expected, sut.FindAnagrams(candidates));
    }

    [Fact]
    public void Does_not_detect_anagram_subsets()
    {
        var candidates = new[] { "dog", "goody" };
        var sut = new Anagram("good");
        Assert.Empty(sut.FindAnagrams(candidates));
    }

    [Fact]
    public void Detects_anagram()
    {
        var candidates = new[] { "enlists", "google", "inlets", "banana" };
        var sut = new Anagram("listen");
        var expected = new[] { "inlets" };
        Assert.Equal(expected, sut.FindAnagrams(candidates));
    }

    [Fact]
    public void Detects_three_anagrams()
    {
        var candidates = new[] { "gallery", "ballerina", "regally", "clergy", "largely", "leading" };
        var sut = new Anagram("allergy");
        var expected = new[] { "gallery", "regally", "largely" };
        Assert.Equal(expected, sut.FindAnagrams(candidates));
    }

    [Fact]
    public void Detects_multiple_anagrams_with_different_case()
    {
        var candidates = new[] { "Eons", "ONES" };
        var sut = new Anagram("nose");
        var expected = new[] { "Eons", "ONES" };
        Assert.Equal(expected, sut.FindAnagrams(candidates));
    }

    [Fact]
    public void Does_not_detect_non_anagrams_with_identical_checksum()
    {
        var candidates = new[] { "last" };
        var sut = new Anagram("mass");
        Assert.Empty(sut.FindAnagrams(candidates));
    }

    [Fact]
    public void Detects_anagrams_case_insensitively()
    {
        var candidates = new[] { "cashregister", "Carthorse", "radishes" };
        var sut = new Anagram("Orchestra");
        var expected = new[] { "Carthorse" };
        Assert.Equal(expected, sut.FindAnagrams(candidates));
    }

    [Fact]
    public void Detects_anagrams_using_case_insensitive_subject()
    {
        var candidates = new[] { "cashregister", "carthorse", "radishes" };
        var sut = new Anagram("Orchestra");
        var expected = new[] { "carthorse" };
        Assert.Equal(expected, sut.FindAnagrams(candidates));
    }

    [Fact]
    public void Detects_anagrams_using_case_insensitive_possible_matches()
    {
        var candidates = new[] { "cashregister", "Carthorse", "radishes" };
        var sut = new Anagram("orchestra");
        var expected = new[] { "Carthorse" };
        Assert.Equal(expected, sut.FindAnagrams(candidates));
    }

    [Fact]
    public void Does_not_detect_an_anagram_if_the_original_word_is_repeated()
    {
        var candidates = new[] { "go Go GO" };
        var sut = new Anagram("go");
        Assert.Empty(sut.FindAnagrams(candidates));
    }

    [Fact]
    public void Anagrams_must_use_all_letters_exactly_once()
    {
        var candidates = new[] { "patter" };
        var sut = new Anagram("tapper");
        Assert.Empty(sut.FindAnagrams(candidates));
    }

    [Fact]
    public void Words_are_not_anagrams_of_themselves()
    {
        var candidates = new[] { "BANANA" };
        var sut = new Anagram("BANANA");
        Assert.Empty(sut.FindAnagrams(candidates));
    }

    [Fact]
    public void Words_are_not_anagrams_of_themselves_even_if_letter_case_is_partially_different()
    {
        var candidates = new[] { "Banana" };
        var sut = new Anagram("BANANA");
        Assert.Empty(sut.FindAnagrams(candidates));
    }

    [Fact]
    public void Words_are_not_anagrams_of_themselves_even_if_letter_case_is_completely_different()
    {
        var candidates = new[] { "banana" };
        var sut = new Anagram("BANANA");
        Assert.Empty(sut.FindAnagrams(candidates));
    }

    [Fact]
    public void Words_other_than_themselves_can_be_anagrams()
    {
        var candidates = new[] { "LISTEN", "Silent" };
        var sut = new Anagram("LISTEN");
        var expected = new[] { "Silent" };
        Assert.Equal(expected, sut.FindAnagrams(candidates));
    }

    #region Extra tests

    [Fact]
    public void Different_word_count()
    {
        var candidates = new[] { "I am a weakish speller", "I am a peakish sweller", "I am not a weakish speller" };
        var sut = new Anagram("William Shakespeare");
        var expected = new[] { "I am a weakish speller", "I am a peakish sweller" };
        Assert.Equal(expected, sut.FindAnagrams(candidates));
    }

    [Theory]
    [InlineData("hydroxydeoxycorticosterones", "hydroxydesoxycorticosterone")]
    [InlineData("abcdefghijklmnopqrstuvwxyz", "zyxwvutsrqponmlkjihgfedcba")]
    [InlineData("conversationalists", "conservationalists")]
    [InlineData("basiparachromatin", "marsipobranchiata")]
    [InlineData("undefinability", "unidentifiably")]
    public void Long_anagrams(string word, string matchingCandidate)
    {
        var candidates = new []{ matchingCandidate };
        var sut = new Anagram(word);
        var expected = new[] { matchingCandidate };
        Assert.Equal(expected, sut.FindAnagrams(candidates));
    }

    #endregion Extra tests
}