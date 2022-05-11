using System;
using System.Diagnostics;
using System.Globalization;

class WeighingMachine
{
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
                _cachedDisplayWeight = FormatWeight(tareAdjustedWeight, Precision);
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

    public WeighingMachine(int precision)
    {
        Precision = precision;
        TareAdjustment = 5.0;
    }

    // Pre-allocated format related stuff to avoid intermediate heap allocations.
    private static readonly string FormatKgPostfix = " kg";
    private static readonly string FloatFormat = "0.0000000000000000";
    // Length of "0." from float format.
    private const int MinFormatLength = 2;
    // Arbitrarily chosen max format precision.
    private static readonly int MaxFormatPrecision = FloatFormat.Length - MinFormatLength;

    /// <summary>
    /// Formats weight using intermediate stackalloc to avoid intermediate heap allocations.
    /// </summary>
    /// <remarks>
    /// Remember kids, premature optimization is the root of all evil.
    /// </remarks>
    /// <param name="weight">Weight to format.</param>
    /// <param name="precision">Precision of formatted weight.</param>
    /// <returns>Formatted weight.</returns>
    /// <exception cref="ArgumentOutOfRangeException">Precision is over 16.</exception>
    /// <exception cref="InvalidOperationException">Formatting failed due to poor coding.</exception>
    private static string FormatWeight(double weight, int precision)
    {
        // Arbitrarily chosen max buffer length to fit formatted double.MaxValue.
        const int MaxBufferLength = 320;

        if (precision > MaxFormatPrecision) { throw new ArgumentOutOfRangeException($"Precision is too high for formatting. Max supported precision is {MaxFormatPrecision}"); }

        int usedFormatLength = precision + MinFormatLength;
        var usedFormat = FloatFormat.AsSpan(0, usedFormatLength);

        // Might be faster to just use the max buffer length instead of calculating required length but it should be measured instead of assumed.
        int bufferLength = RequiredBufferLength(weight, precision);
        Debug.Assert(bufferLength < MaxBufferLength, "Formatting max buffer length (320) exceeded.");

        Span<char> buffer = stackalloc char[bufferLength];
        if (!weight.TryFormat(buffer, out int weightCharsWritten, usedFormat, CultureInfo.InvariantCulture))
        {
            throw new InvalidOperationException("Weight formatting failed due to unnecessarily complicated code.");
        }

        WriteKgPostfix(buffer, weightCharsWritten);

        int totalCharsWritten = weightCharsWritten + FormatKgPostfix.Length;
        var writtenSection = buffer.Slice(0, totalCharsWritten);
        return new string(writtenSection);
    }

    private void InvalidateCache()
    {
        _cachedDisplayWeight = null;
    }

    private static void WriteKgPostfix(Span<char> destination, int postfixStartIndex)
    {
        var postfixSection = destination.Slice(postfixStartIndex, FormatKgPostfix.Length);
        FormatKgPostfix.CopyTo(postfixSection);
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
               + FormatKgPostfix.Length;
    }

    private static int DigitCount(double number)
    {
        const int MinimumDigitCount = 1;
        return (int)Math.Floor(Math.Log10(Math.Abs(number))) + MinimumDigitCount;
    }
}
