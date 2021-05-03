using System;
using System.Collections.Generic;
using System.Linq;

public static class NucleotideCount
{
    private const string NucleotideChars = "ACGT";

    public static IDictionary<char, int> Count(string sequence)
    {
        if (sequence?.Any(c => !NucleotideChars.Contains(c)) ?? false)
        {
            throw new ArgumentException("Sequence contains invalid nucleotide symbols.");
        }

        var nucleotideCounts = sequence?.GroupBy(c => c)
                                       .ToDictionary(g => g.Key, g => g.Count()) ?? new Dictionary<char, int>();

        foreach (char nucleotide in NucleotideChars)
        {
            nucleotideCounts.TryAdd(nucleotide, 0);
        }

        return nucleotideCounts;
    }
}