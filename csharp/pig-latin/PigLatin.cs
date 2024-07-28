using System;
using System.Buffers;
using System.Diagnostics.CodeAnalysis;
using System.Text;

public static class PigLatin
{
    private const string PigLatinWordEnding = "ay";

    private static readonly SearchValues<char> Vowels = SearchValues.Create("AEIOUaeiou");
    private static readonly SearchValues<char> Q = SearchValues.Create("Qq");
    private static readonly SearchValues<char> U = SearchValues.Create("Uu");
    private static readonly SearchValues<char> Y = SearchValues.Create("Yy");

    public static string Translate(string word)
    {
        // Word is a bit misleading name when it can contain multiple words.
        ReadOnlySpan<char> phrase = word.AsSpan().Trim();
        if (phrase.IsEmpty)
        {
            return string.Empty;
        }

        // ~1/2 kB stack allocation, Range = 2 * Int32 = 8 B
        const int MaxWordCount = 64;
        Span<Range> wordRangeBuffer = stackalloc Range[MaxWordCount];
        int wordCount = phrase.Split(wordRangeBuffer, ' ', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries);
        ReadOnlySpan<Range> words = wordRangeBuffer[..wordCount];
        if (words.Length == 0)
        {
            return string.Empty;
        }
        
        // ~1/2 kB stack allocation, char = 2 B
        const int MaxWordLength = 256;
        Span<char> translationBuffer = stackalloc char[MaxWordLength];
        if (words.Length == 1)
        {
            int writtenChars = TranslateWord(word, translationBuffer);
            return translationBuffer[..writtenChars].ToString();
        }

        StringBuilder pigLatinBuilder = new();
        foreach (Range wordRange in words)
        {
            ReadOnlySpan<char> actualWord = phrase[wordRange];
            int writtenChars = TranslateWord(actualWord, translationBuffer);
            ReadOnlySpan<char> translatedWord = translationBuffer[..writtenChars].ToString();
            pigLatinBuilder.Append(translatedWord);
            pigLatinBuilder.Append(' ');
        }

        return pigLatinBuilder
            .TrimEnd()
            .ToString();
    }

    private static int TranslateWord(ReadOnlySpan<char> word, Span<char> destination)
    {
        Span<char>remainingDestination = destination;
        if (Vowels.Contains(word[0]) ||
            word.StartsWith("xr", StringComparison.OrdinalIgnoreCase) ||
            word.StartsWith("yt", StringComparison.OrdinalIgnoreCase))
        {
            word.CopyTo(remainingDestination);
            remainingDestination = remainingDestination[word.Length..];
            PigLatinWordEnding.CopyTo(remainingDestination);
            return word.Length + PigLatinWordEnding.Length;
        }

        ReadOnlySpan<char> remainingWord = word;
        ReadOnlySpan<char> consonants = remainingWord[..CountConsecutiveConsonants(remainingWord)];
        remainingWord = remainingWord[consonants.Length..];

        remainingWord.CopyTo(remainingDestination);
        remainingDestination = remainingDestination[remainingWord.Length..];

        consonants.CopyTo(remainingDestination);
        remainingDestination = remainingDestination[consonants.Length..];

        PigLatinWordEnding.CopyTo(remainingDestination);

        return remainingWord.Length + consonants.Length + PigLatinWordEnding.Length;
    }

    private static int CountConsecutiveConsonants(ReadOnlySpan<char> word)
    {
        int firstVowelIndex = word.IndexOfAny(Vowels);
        int consonantCount = firstVowelIndex switch
        {
            -1 => word.Length,
            _ => firstVowelIndex,
        };

        if (consonantCount == 0)
        {
            return 0;
        }

        char lastConsonant = word[consonantCount - 1];

        // QU is consonant.
        if (Q.Contains(lastConsonant) &&
            word[firstVowelIndex] is char firstVowel &&
            U.Contains(firstVowel))
        {
            return consonantCount + 1;
        }

        // Y is vowel when not first character.
        if (consonantCount >= 2)
        {
            if (Y.Contains(lastConsonant))
            {
                return consonantCount - 1;
            }

            ReadOnlySpan<char> consonants = word[..consonantCount];
            int indexOfY = consonants.IndexOfAny(Y);
            if (indexOfY != -1)
            {
                return indexOfY;
            }
        }

        return consonantCount;
    }
}

public static class StringBuilderExtensions
{
    [return: NotNullIfNotNull(nameof(self))]
    public static StringBuilder? TrimEnd(this StringBuilder? self)
    {
        if (self == null || self.Length == 0)
        {
            return self;
        }

        int trimIndex = self.Length - 1;
        for (; trimIndex >= 0; trimIndex--)
        {
            if (!char.IsWhiteSpace(self[trimIndex]))
            {
                break;
            }
        }

        if (trimIndex < self.Length - 1)
        {
            self.Length = trimIndex + 1;
        }

        return self;
    }
}