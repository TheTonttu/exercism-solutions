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
        // Word is a bit misleading name if it can contain multiple words.
        ReadOnlySpan<char> phrase = word.AsSpan().Trim();
        if (phrase.IsEmpty)
        {
            return string.Empty;
        }

        const int MaxWordCount = 50;
        Span<Range> wordRangeBuffer = stackalloc Range[MaxWordCount];
        int wordCount = phrase.Split(wordRangeBuffer, ' ', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries);

        ReadOnlySpan<Range> words = wordRangeBuffer[..wordCount];

        if (words.Length == 0)
        {
            return string.Empty;
        }

        if (words.Length == 1)
        {
            return TranslateWord(word);
        }

        StringBuilder pigLatinBuilder = new();
        foreach (Range wordRange in words)
        {
            ReadOnlySpan<char> actualWord = phrase[wordRange];
            TranslateAndAppendWord(actualWord, pigLatinBuilder);
            pigLatinBuilder.Append(' ');
        }

        return pigLatinBuilder
            // Trim any leftover whitespace.
            .TrimEnd()
            .ToString();
    }

    private static void TranslateAndAppendWord(ReadOnlySpan<char> word, StringBuilder builder)
    {
        if (Vowels.Contains(word[0]) ||
            word.StartsWith("xr", StringComparison.OrdinalIgnoreCase) ||
            word.StartsWith("yt", StringComparison.OrdinalIgnoreCase))
        {
            builder.Append(word);
            builder.Append(PigLatinWordEnding);
            return;
        }

        ReadOnlySpan<char> remainingWord = word;
        Span<char> suffixBuffer = stackalloc char[100];
        Span<char> remainingSuffixBuffer = suffixBuffer;

        ReadOnlySpan<char> consonantPrefix = remainingWord[..CountConsecutiveConsonants(remainingWord)];
        consonantPrefix.CopyTo(remainingSuffixBuffer);
        remainingSuffixBuffer = remainingSuffixBuffer[consonantPrefix.Length..];
        remainingWord = remainingWord[consonantPrefix.Length..];

        ReadOnlySpan<char> suffix = suffixBuffer[..^remainingSuffixBuffer.Length];
        builder.Append(remainingWord);
        builder.Append(suffix);
        builder.Append(PigLatinWordEnding);
    }

    private static string TranslateWord(ReadOnlySpan<char> word)
    {
        // Rule 1
        if (Vowels.Contains(word[0]) ||
            word.StartsWith("xr", StringComparison.OrdinalIgnoreCase) ||
            word.StartsWith("yt", StringComparison.OrdinalIgnoreCase))
        {
            return string.Concat(word, PigLatinWordEnding);
        }

        ReadOnlySpan<char> remainingWord = word;
        Span<char> suffixBuffer = stackalloc char[100];
        Span<char> remainingSuffixBuffer = suffixBuffer;

        ReadOnlySpan<char> consonantPrefix = remainingWord[..CountConsecutiveConsonants(remainingWord)];
        consonantPrefix.CopyTo(remainingSuffixBuffer);
        remainingSuffixBuffer = remainingSuffixBuffer[consonantPrefix.Length..];
        remainingWord = remainingWord[consonantPrefix.Length..];

        ReadOnlySpan<char> suffix = suffixBuffer[..^remainingSuffixBuffer.Length];
        return string.Concat(remainingWord, suffix, PigLatinWordEnding);
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