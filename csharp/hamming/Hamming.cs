using System;
using System.Linq;

public static class Hamming
{
    public static int Distance(string firstStrand, string secondStrand)
    {
        if (firstStrand?.Length != secondStrand?.Length)
        {
            throw new ArgumentException("Strands are not equal length");
        }

        return firstStrand
            .Zip(secondStrand)
            .Count(zip => zip.First != zip.Second);
    }
}