using System;
using System.Buffers.Binary;

public static class TelemetryBuffer
{
    private const int PrefixByteIndex = 0;
    private const short SignedPrefixIdentifier = 256;
    private const int ReadingDataStartIndex = 1;
    private const int MaxByteCount = sizeof(Int64);

    public static byte[] ToBuffer(long reading)
    {
        var buffer = new byte[9];
        WriteReading(buffer, reading);
        return buffer;
    }

    public static long FromBuffer(byte[] buffer)
    {
        byte prefix = buffer[PrefixByteIndex];
        (bool isSigned, byte byteCount) = ParsePrefix(prefix);
        if (byteCount > MaxByteCount) { return default; }
        var readingData = buffer.AsSpan(ReadingDataStartIndex, length: byteCount);
        return ComposeReading(readingData, isSigned);
    }

    private static void WriteReading(Span<byte> destination, long reading)
    {
        ref byte prefixWriteLocation = ref destination[PrefixByteIndex];
        var dataWriteLocation = destination[ReadingDataStartIndex..];
        if (reading is > UInt32.MaxValue or < Int32.MinValue)
        {
            prefixWriteLocation = SignedPrefixIdentifier - sizeof(Int64);
            BinaryPrimitives.WriteInt64LittleEndian(dataWriteLocation, reading);
        }
        else if (reading > Int32.MaxValue)
        {
            prefixWriteLocation = sizeof(UInt32);
            BinaryPrimitives.WriteUInt32LittleEndian(dataWriteLocation, (UInt32)reading);
        }
        else if (reading is > UInt16.MaxValue or < Int16.MinValue)
        {
            prefixWriteLocation = SignedPrefixIdentifier - sizeof(Int32);
            BinaryPrimitives.WriteInt32LittleEndian(dataWriteLocation, (Int32)reading);
        }
        else if (reading is > Int16.MaxValue or >= 0)
        {
            prefixWriteLocation = sizeof(UInt16);
            BinaryPrimitives.WriteUInt16LittleEndian(dataWriteLocation, (UInt16)reading);
        }
        else
        {
            prefixWriteLocation = SignedPrefixIdentifier - sizeof(Int16);
            BinaryPrimitives.WriteInt16LittleEndian(dataWriteLocation, (Int16)reading);
        }
    }

    private static long ComposeReading(ReadOnlySpan<byte> readingData, bool isSigned) =>
        (isSigned, readingData.Length) switch
        {
            (true, sizeof(Int64)) => BinaryPrimitives.ReadInt64LittleEndian(readingData),
            (true, sizeof(Int32)) => BinaryPrimitives.ReadInt32LittleEndian(readingData),
            (true, sizeof(Int16)) => BinaryPrimitives.ReadInt16LittleEndian(readingData),
            (false, sizeof(UInt32)) => BinaryPrimitives.ReadUInt32LittleEndian(readingData),
            (false, sizeof(UInt16)) => BinaryPrimitives.ReadUInt16LittleEndian(readingData),
            _ => 0
        };

    private static (bool IsSigned, byte ByteCount) ParsePrefix(byte prefix)
    {
        bool isSigned = prefix > MaxByteCount;

        byte byteCount =
            (byte)(isSigned
                ? SignedPrefixIdentifier - prefix
                : prefix);

        return (isSigned, byteCount);
    }
}
