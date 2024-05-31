using System;
using System.Collections.Generic;

public static class ProteinTranslation
{
    public static string[] Proteins(string strand)
    {
        const string TerminationProtein = "STOP";
        const int CodonLength = 3;

        var proteins = new List<string>();

        ReadOnlySpan<char> remaining = strand.AsSpan();
        while (remaining.Length > 0)
        {
            ReadOnlySpan<char> codon = remaining[..CodonLength];
            string protein = codon switch
            {
                "AUG" => "Methionine",
                "UUU" or "UUC" => "Phenylalanine",
                "UUA" or "UUG" => "Leucine",
                "UCU" or "UCC" or "UCA" or "UCG" => "Serine",
                "UAU" or "UAC" => "Tyrosine",
                "UGU" or "UGC" => "Cysteine",
                "UGG" => "Tryptophan",
                "UAA" or "UAG" or "UGA" => TerminationProtein,
                _ => throw new InvalidOperationException($"Unidentified codon '{codon}'")
            };
            if (string.Equals(protein, TerminationProtein, StringComparison.Ordinal))
            {
                break;
            }
            proteins.Add(protein);
            remaining = remaining[CodonLength..];
        }

        return proteins.ToArray();
    }
}