using System;
using System.Text;

public static class RnaTranscription
{
    public static string ToRna(string dnaStrand)
    {
        var rnaStrandBuilder = new StringBuilder();
        foreach (char dnaNucleotide in dnaStrand)
        {
            char rnaNucleotide = dnaNucleotide switch
            {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => throw new InvalidOperationException($"{dnaNucleotide} is invalid DNA nucleotide.")
            };
            rnaStrandBuilder.Append(rnaNucleotide);
        }
        return rnaStrandBuilder.ToString();
    }
}