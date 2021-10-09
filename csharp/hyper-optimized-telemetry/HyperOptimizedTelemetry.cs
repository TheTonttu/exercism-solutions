using System;

public static class TelemetryBuffer
{
    private const int PrefixByteIndex = 0;
    private const short SignedPrefixIdentifier = 256;
    private const int DataSectionStartIndex = 1;

    public static byte[] ToBuffer(long reading)
    {
        (bool isSigned, byte[] readingBytes) = ParseSignAndBytes(reading);

        var buffer = new byte[9];
        buffer[PrefixByteIndex] = ComposePrefix(isSigned, readingBytes.Length);
        readingBytes.CopyTo(buffer, DataSectionStartIndex);

        return buffer;
    }

    public static long FromBuffer(byte[] buffer)
    {
        byte prefix = buffer[PrefixByteIndex];
        (bool isSigned, byte byteCount) = ParsePrefix(prefix);

        return ComposeReading(buffer, isSigned, byteCount);
    }

    private static (bool IsSigned, byte[] Bytes) ParseSignAndBytes(long reading) =>
        reading switch
        {
            > UInt32.MaxValue or < Int32.MinValue => (true, BitConverter.GetBytes(reading)),
            > Int32.MaxValue => (false, BitConverter.GetBytes(Convert.ToUInt32(reading))),
            > UInt16.MaxValue or < Int16.MinValue => (true, BitConverter.GetBytes(Convert.ToInt32(reading))),
            > Int16.MaxValue => (false, BitConverter.GetBytes(Convert.ToUInt16(reading))),
            _ => (true, BitConverter.GetBytes(Convert.ToInt16(reading))),
        };

    private static byte ComposePrefix(bool isSigned, int length) =>
        (byte)(isSigned
            ? SignedPrefixIdentifier - length
            : length);

    private static long ComposeReading(byte[] buffer, bool isSigned, int byteCount) =>
        (isSigned, byteCount) switch
        {
            (true, 8) => BitConverter.ToInt64(buffer, DataSectionStartIndex),
            (true, 4) => BitConverter.ToInt32(buffer, DataSectionStartIndex),
            (true, 2) => BitConverter.ToInt16(buffer, DataSectionStartIndex),
            (false, 4) => BitConverter.ToUInt32(buffer, DataSectionStartIndex),
            (false, 2) => BitConverter.ToUInt16(buffer, DataSectionStartIndex),
            _ => 0
        };

    private static (bool IsSigned, byte ByteCount) ParsePrefix(byte prefix)
    {
        const int MaxByteCount = 8;
        bool isSigned = prefix > MaxByteCount;

        byte byteCount =
            (byte)(isSigned
                ? SignedPrefixIdentifier - prefix
                : prefix);

        return (isSigned, byteCount);
    }
}
