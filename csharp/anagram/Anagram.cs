using System;
using System.Linq;

public class Anagram
{
    const int AlphabetCount = 26; // A-Z

    private readonly string _baseWord;
    private readonly int[] _baseWordLetterCounts = new int[AlphabetCount];

    public Anagram(string baseWord)
    {
        if (string.IsNullOrWhiteSpace(baseWord))
        {
            throw new ArgumentException($"'{nameof(baseWord)}' cannot be null or whitespace.", nameof(baseWord));
        }

        _baseWord = baseWord.Trim();

        int[] letterCounts = new int[AlphabetCount];
        foreach (char letter in _baseWord)
        {
            if (char.IsWhiteSpace(letter))
            {
                continue;
            }

            int letterIndex = AlphabetIndex(letter);
            letterCounts[letterIndex] += 1;
        }
        _baseWordLetterCounts = letterCounts;
    }

    public string[] FindAnagrams(string[] potentialMatches)
    {
        return potentialMatches
            .Where(IsAnagram)
            .ToArray();
    }

    private bool IsAnagram(string candidate)
    {
        if (string.IsNullOrEmpty(candidate)
            || string.Equals(candidate, _baseWord, StringComparison.InvariantCultureIgnoreCase))
        {
            return false;
        }

        Span<int> remainingMatches = stackalloc int[AlphabetCount];
        _baseWordLetterCounts.CopyTo(remainingMatches);

        foreach (char letter in candidate)
        {
            if (char.IsWhiteSpace(letter))
            {
                continue;
            }

            int letterIndex = AlphabetIndex(letter);
            int remainingLetterCount = remainingMatches[letterIndex] -= 1;
            if (remainingLetterCount < 0)
            {
                return false;
            }
        }

        for (int i = 0; i < remainingMatches.Length; i++)
        {
            if (remainingMatches[i] != 0)
            {
                return false;
            }
        }

        return true;
    }

    private int AlphabetIndex(char letter)
    {
        if (letter is >= 'a' and <= 'z')
        {
            return letter - 'a';
        }
        else if (letter is >= 'A' and <= 'Z')
        {
            return letter - 'A';
        }
        throw new ArgumentOutOfRangeException(nameof(letter), $"Letter '{letter}' is outside alphabet (a-z, A-Z) index range.");
    }
}