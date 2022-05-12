using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;

[MemoryDiagnoser]
public class WeighingMachineBenchmarkPrealloc
{
    private const int PreinitializedPrecision = 5;

    private readonly WeighingMachine weighingMachine = new WeighingMachine(PreinitializedPrecision);
    private readonly PrematureOptimizationWeighingMachine preOptWeighingMachine = new PrematureOptimizationWeighingMachine(PreinitializedPrecision);

    [Benchmark(Baseline = true)]
    public string DisplayWeightNormal() => weighingMachine.DisplayWeight;

    [Benchmark]
    public string DisplayWeightPrematureOptimization() => preOptWeighingMachine.DisplayWeight;
}

[MemoryDiagnoser]
public class WeighingMachineBenchmarkInitializeAndRecall
{
    [Params(2, 5, 10)]
    public int Precision { get; set; }

    [Params(10, 100, 1000)]
    public int RecallCount { get; set; }

    [Benchmark(Baseline = true)]
    public string DisplayWeightNormal()
    {
        var wm = new WeighingMachine(Precision);
        string result = null;
        for (int i = 0; i < RecallCount; i++)
        {
            result = wm.DisplayWeight;
        }
        return result;
    }

    [Benchmark]
    public string DisplayWeightPrematureOptimization()
    {
        var wm = new PrematureOptimizationWeighingMachine(Precision);
        string result = null;
        for (int i = 0; i < RecallCount; i++)
        {
            result = wm.DisplayWeight;
        }
        return result;
    }
}

public class Program
{
    public static void Main(string[] args)
    {
        var summary = BenchmarkRunner.Run(typeof(Program).Assembly);
    }
}
