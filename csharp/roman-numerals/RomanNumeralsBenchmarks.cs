using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;
using System.Collections.Generic;
using System.Text;

namespace RomanNumerals
{
    [MemoryDiagnoser]
    public class RomanNumeralsBenchmarks
    {
        [Benchmark(Baseline = true)]
        [ArgumentsSource(nameof(Numbers))]
        public string StringBuilderReplace(int number) {
            return StringBuilderReplaceImplementation.ToRoman(number);
        }

        [Benchmark]
        [ArgumentsSource(nameof(Numbers))]
        public string StringBuilderReplaceWithoutInitialCapacity(int number)
        {
            return StringBuilderReplaceImplementation.ToRomanWithoutInitialCapacity(number);
        }

        [Benchmark]
        [ArgumentsSource(nameof(Numbers))]
        public string WhileLoopSwitchPattern(int number) {
            return number.ToRoman();
        }


        public static IEnumerable<int> Numbers()
        {
            yield return 1;
            yield return 911;
            yield return 3999;
        }
    }

    /// <summary>
    /// Oof
    /// </summary>
    internal static class StringBuilderReplaceImplementation
    {
        public static string ToRoman(int value)
        {
            var stringBuilder = new StringBuilder(capacity: value);

            stringBuilder
                .Append('I', value)
                // 5
                .Replace("IIIII", "V")
                // 10
                .Replace("VV", "X")
                // 50
                .Replace("XXXXX", "L")
                // 100
                .Replace("LL", "C")
                // 500
                .Replace("CCCCC", "D")
                // 1000
                .Replace("DD", "M")
                // 4
                .Replace("IIII", "IV")
                // 5 + 4 = 9
                .Replace("VIV", "IX")
                // 40
                .Replace("XXXX", "XL")
                // 50 + 40 = 90
                .Replace("LXL", "XC")
                // 400
                .Replace("CCCC", "CD")
                // 500 + 400 = 900
                .Replace("DCD", "CM");

            return stringBuilder.ToString();
        }

        public static string ToRomanWithoutInitialCapacity(int value)
        {
            var stringBuilder = new StringBuilder();

            stringBuilder
                .Append('I', value)
                // 5
                .Replace("IIIII", "V")
                // 10
                .Replace("VV", "X")
                // 50
                .Replace("XXXXX", "L")
                // 100
                .Replace("LL", "C")
                // 500
                .Replace("CCCCC", "D")
                // 1000
                .Replace("DD", "M")
                // 4
                .Replace("IIII", "IV")
                // 5 + 4 = 9
                .Replace("VIV", "IX")
                // 40
                .Replace("XXXX", "XL")
                // 50 + 40 = 90
                .Replace("LXL", "XC")
                // 400
                .Replace("CCCC", "CD")
                // 500 + 400 = 900
                .Replace("DCD", "CM");

            return stringBuilder.ToString();
        }
    }


    public class Program
    {
        public static void Main(string[] _)
        {
            BenchmarkRunner.Run<RomanNumeralsBenchmarks>();
        }
    }
}
