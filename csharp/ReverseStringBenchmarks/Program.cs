using BenchmarkDotNet.Running;

namespace ReverseStringBenchmarks
{
    internal class Program
    {
        static void Main(string[] args)
        {
            BenchmarkRunner.Run<ReverseStringBenchmarks>();
        }
    }
}