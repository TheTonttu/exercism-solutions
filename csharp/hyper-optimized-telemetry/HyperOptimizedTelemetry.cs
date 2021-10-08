using System;

public static class TelemetryBuffer
{
    private const int PrefixByteIndex = 0;
    private const short SignedPrefixIdentifier = 256;
    private const int DataSectionStartIndex = 1;

    public static byte[] ToBuffer(long reading)
    {
        (bool isSigned, byte[] readingBytes) = ExtractBytes(reading);

        var bytes = new byte[9];
        bytes[PrefixByteIndex] = CreatePrefix(isSigned, readingBytes.Length);
        readingBytes.CopyTo(bytes, DataSectionStartIndex);

        return bytes;
    }

    public static long FromBuffer(byte[] buffer)
    {
        const int Invalid = 0;

        (bool isSigned, byte byteCount) = ExtractPrefix(buffer);
        if (byteCount > 8) { return Invalid; }

        if (isSigned)
        {
            if (byteCount == 8)
            {
                return BitConverter.ToInt64(buffer, DataSectionStartIndex);
            }
            else if (byteCount == 4)
            {
                return BitConverter.ToInt32(buffer, DataSectionStartIndex);
            }
            else
            {
                return BitConverter.ToInt16(buffer, DataSectionStartIndex);
            }
        }
        else
        {
            if (byteCount == 4)
            {
                return BitConverter.ToUInt32(buffer, DataSectionStartIndex);
            }
            else
            {
                return BitConverter.ToUInt16(buffer, DataSectionStartIndex);
            }
        }
    }

    private static byte CreatePrefix(bool isSigned, int length) =>
        (byte)(isSigned
            ? SignedPrefixIdentifier - length
            : length);

    private static (bool IsSigned, byte[] Bytes) ExtractBytes(long reading) =>
        reading switch
        {
            > UInt32.MaxValue or < Int32.MinValue => (true, BitConverter.GetBytes(reading)),
            > Int32.MaxValue => (false, BitConverter.GetBytes(Convert.ToUInt32(reading))),
            > UInt16.MaxValue or < Int16.MinValue => (true, BitConverter.GetBytes(Convert.ToInt32(reading))),
            > Int16.MaxValue => (false, BitConverter.GetBytes(Convert.ToUInt16(reading))),
            _ => (true, BitConverter.GetBytes(Convert.ToInt16(reading))),
        };

    private static (bool IsSigned, byte ByteCount) ExtractPrefix(byte[] buffer)
    {
        byte byteCount = buffer[PrefixByteIndex];
        bool isSigned = byteCount > 8;

        return (
            isSigned,
            isSigned
                ? (byte)(SignedPrefixIdentifier - byteCount)
                : byteCount);
    }
}
