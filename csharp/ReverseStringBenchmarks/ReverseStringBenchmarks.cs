using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Order;
using BenchmarkDotNet.Running;
using System;
using System.Buffers;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using static System.Net.Mime.MediaTypeNames;

namespace ReverseStringBenchmarks
{
    [ShortRunJob]
    [Orderer(SummaryOrderPolicy.FastestToSlowest)]
    [MemoryDiagnoser]
    public class ReverseStringBenchmarks
    {
        [Benchmark(Baseline = true)]
        [ArgumentsSource(nameof(Inputs))]
        public string StringBuilder(string input)
        {
            var stringBuilder = new StringBuilder(input.Length);
            for (int i = input.Length - 1; i >= 0; i--)
            {
                stringBuilder.Append(input[i]);
            }
            return stringBuilder.ToString();
        }

        [Benchmark]
        [ArgumentsSource(nameof(Inputs))]
        public string Linq_ToArray(string input)
        {
            return new string(input.Reverse().ToArray());
        }

        [Benchmark]
        [ArgumentsSource(nameof(Inputs))]
        public string CharArrayReverse(string input)
        {
            char[] chars = input.ToCharArray();
            Array.Reverse(chars);
            return new string(chars);
        }

        // No significant difference to CharArrayReverse
        //[Benchmark]
        [ArgumentsSource(nameof(Inputs))]
        public string CharArraySpanReverse(string input)
        {
            char[] chars = input.ToCharArray();
            chars.AsSpan().Reverse();
            return new string(chars);
        }

        // No significant difference to CharArrayReverse
        //[Benchmark]
        [ArgumentsSource(nameof(Inputs))]
        public string CopyToSpanReverse(string input)
        {
            char[] chars = new char[input.Length];
            input.CopyTo(chars);
            chars.AsSpan().Reverse();
            return new string(chars);
        }

        [Benchmark]
        [ArgumentsSource(nameof(Inputs))]
        public string CopyToSpanStackallocReverse(string input)
        {
            Span<char> chars = input.Length > 256
                ? new char[input.Length]
                : stackalloc char[input.Length];
            input.CopyTo(chars);
            chars.Reverse();
            return new string(chars);
        }

        [Benchmark]
        [ArgumentsSource(nameof(Inputs))]
        public string CopyToSpanOrRentedArrayReverse(string input)
        {
            char[]? rentedArray = null;
            try
            {
                Span<char> buffer = input.Length > 256
                ? (rentedArray = ArrayPool<char>.Shared.Rent(input.Length))
                : stackalloc char[input.Length];
                var slice = buffer[..input.Length];
                input.CopyTo(slice);
                slice.Reverse();
                return new string(slice);
            }
            finally
            {
                if (rentedArray != null)
                {
                    ArrayPool<char>.Shared.Return(rentedArray);
                }
            }
        }

        public IEnumerable<object> Inputs()
        {
            yield return "abc";
            yield return "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent eu massa felis. In sed pretium ipsum.";
            // 256 within stackalloc
            yield return "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent eu massa felis. In sed pretium ipsum. In fermentum ex odio, eu auctor elit placerat quis. Donec consequat orci eget leo volutpat pharetra. Nam pellentesque cursus ligula at interdum. Donec a";
            // 257 over stackalloc
            yield return "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent eu massa felis. In sed pretium ipsum. In fermentum ex odio, eu auctor elit placerat quis. Donec consequat orci eget leo volutpat pharetra. Nam pellentesque cursus ligula at interdum. Donec ac";
            yield return "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent eu massa felis. In sed pretium ipsum. In fermentum ex odio, eu auctor elit placerat quis. Donec consequat orci eget leo volutpat pharetra. Nam pellentesque cursus ligula at interdum. Donec ac dapibus quam. Integer tristique risus velit, in bibendum dolor placerat in.";
        }
    }
}
