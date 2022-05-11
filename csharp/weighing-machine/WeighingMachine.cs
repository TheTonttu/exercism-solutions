using System;
using System.Diagnostics;
using System.Globalization;

class WeighingMachine
{
    private readonly NumberFormatInfo _weightNumberFormatInfo;

    private double _weight;

    public int Precision { get; }


    public double Weight
    {
        get => _weight;
        set
        {
            if (value < 0) { throw new ArgumentOutOfRangeException("Weight cannot be negative."); }
            _weight = value;
        }
    }

    public string DisplayWeight {
        get
        {
            double tareAdjustedWeight = _weight - TareAdjustment;
            return String.Format(_weightNumberFormatInfo, "{0:F} kg", tareAdjustedWeight);
        }
    }

    public double TareAdjustment { get; set; }

    public WeighingMachine(int precision)
    {
        Precision = precision;
        _weightNumberFormatInfo = new NumberFormatInfo() { NumberDecimalDigits = precision };
        TareAdjustment = 5.0;
    }
}

#region Nope

class DerpWeighingMachine
{
    private readonly NumberFormatInfo _weightNumberFormatInfo;

    private double _weight;
    private string? _cachedDisplayWeight;
    private double _tareAdjustment;

    public int Precision { get; }

    public double Weight
    {
        get => _weight;
        set
        {
            if (value < 0) { throw new ArgumentOutOfRangeException("Weight cannot be negative."); }
            _weight = value;
            InvalidateCache();
        }
    }

    public string DisplayWeight
    {
        get
        {
            if (_cachedDisplayWeight == null)
            {
                double tareAdjustedWeight = _weight - TareAdjustment;
                _cachedDisplayWeight = FormatWeight(tareAdjustedWeight, _weightNumberFormatInfo);
            }
            return _cachedDisplayWeight;
        }
    }
    public double TareAdjustment
    {
        get => _tareAdjustment;
        set
        {
            _tareAdjustment = value;
            InvalidateCache();
        }
    }

    public DerpWeighingMachine(int precision)
    {
        Precision = precision;
        _weightNumberFormatInfo = new() { NumberDecimalDigits = precision };
        TareAdjustment = 5.0;
    }

    private void InvalidateCache()
    {
        _cachedDisplayWeight = null;
    }

    private static readonly string FormattedWeightKgPostfix = " kg";
    // Arbitrarily chosen max format precision.
    private static readonly int MaxFormatPrecision = 16;

    /// <summary>
    /// Formats weight using stackalloc to avoid intermediate heap allocations.
    /// </summary>
    /// <remarks>
    /// Remember kids, premature optimization is the root of all evil.
    /// </remarks>
    /// <param name="weight"></param>
    /// <param name="weightFormatInfo"></param>
    /// <returns>Formatted weight.</returns>
    /// <exception cref="ArgumentOutOfRangeException">Precision is over 16.</exception>
    /// <exception cref="InvalidOperationException">Formatting failed due to poor coding.</exception>
    private static string FormatWeight(double weight, NumberFormatInfo weightFormatInfo)
    {
        // Arbitrarily chosen max buffer length to fit formatted double.MaxValue.
        const int MaxBufferLength = 320;

        int precision = weightFormatInfo.NumberDecimalDigits;
        if (precision > MaxFormatPrecision) { throw new ArgumentOutOfRangeException($"Precision is too high for formatting. Max supported precision is {MaxFormatPrecision}"); }

        // Might be faster to always just use the max buffer length instead of calculating required length but it should be measured instead of assumed.
        int bufferLength = RequiredBufferLength(weight, precision);
        Debug.Assert(bufferLength < MaxBufferLength, "Formatting max buffer length (320) exceeded.");

        const string WeightFormat = "F";
        Span<char> buffer = stackalloc char[bufferLength];
        if (!weight.TryFormat(buffer, out int weightCharsWritten, WeightFormat, weightFormatInfo))
        {
            throw new InvalidOperationException("Weight formatting failed due to unnecessarily complicated code.");
        }

        WriteKgPostfix(buffer, weightCharsWritten);

        int totalCharsWritten = weightCharsWritten + FormattedWeightKgPostfix.Length;
        var writtenSection = buffer.Slice(0, totalCharsWritten);
        return new string(writtenSection);
    }

    private static void WriteKgPostfix(Span<char> destination, int postfixStartIndex)
    {
        var postfixSection = destination.Slice(postfixStartIndex, FormattedWeightKgPostfix.Length);
        FormattedWeightKgPostfix.CopyTo(postfixSection);
    }

    private static int RequiredBufferLength(double number, int precision)
    {
        // Decimal separator length and negative extra might not apply if InvariantCulture is not used.
        const int DecimalSeparatorLength = 1;

        int negativeExtra = double.IsNegative(number) ? 1 : 0;
        return DigitCount(number)
               + precision
               + DecimalSeparatorLength
               + negativeExtra
               + FormattedWeightKgPostfix.Length;
    }

    private static int DigitCount(double number)
    {
        const int MinimumDigitCount = 1;
        return (int)Math.Floor(Math.Log10(Math.Abs(number))) + MinimumDigitCount;
    }
}

#endregion