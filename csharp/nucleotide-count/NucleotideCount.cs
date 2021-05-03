using System;
using System.Collections.Generic;
using System.Linq;

public static class NucleotideCount
{
    private const string NucleotideChars = "ACGT";

    public static IDictionary<char, int> Count(string sequence)
    {
        if (!HasValidNucleotideSymbols(sequence))
        {
            throw new ArgumentException("Sequence contains invalid nucleotide symbols.");
        }

        var nucleotideCounts = sequence?.GroupBy(c => char.ToUpperInvariant(c))
                                       .ToDictionary(g => g.Key,
                                                     g => g.Count()) ?? new Dictionary<char, int>();

        foreach (char nucleotide in NucleotideChars)
        {
            nucleotideCounts.TryAdd(nucleotide, 0);
        }

        return nucleotideCounts;
    }

    private static bool HasValidNucleotideSymbols(string sequence) => sequence?.All(c => NucleotideChars.Contains(c, StringComparison.OrdinalIgnoreCase)) ?? true;
}