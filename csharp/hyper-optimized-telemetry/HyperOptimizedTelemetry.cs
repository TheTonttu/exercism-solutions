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
        var prefix = (sbyte)buffer[PrefixByteIndex];
        if (!IsValidPrefix(prefix)) { return default; }
        var readingData = buffer.AsSpan(ReadingDataStartIndex..);
        return ReadReading(readingData, prefix);
    }

    private static void WriteReading(Span<byte> destination, long reading)
    {
        ref byte prefixWriteLocation = ref destination[PrefixByteIndex];
        var dataWriteLocation = destination[ReadingDataStartIndex..];
        if (reading is > UInt32.MaxValue or < Int32.MinValue)
        {
            prefixWriteLocation = SignedPrefix(sizeof(Int64));
            BinaryPrimitives.WriteInt64LittleEndian(dataWriteLocation, reading);
        }
        else if (reading > Int32.MaxValue)
        {
            prefixWriteLocation = sizeof(UInt32);
            BinaryPrimitives.WriteUInt32LittleEndian(dataWriteLocation, (UInt32)reading);
        }
        else if (reading is > UInt16.MaxValue or < Int16.MinValue)
        {
            prefixWriteLocation = SignedPrefix(sizeof(Int32));
            BinaryPrimitives.WriteInt32LittleEndian(dataWriteLocation, (Int32)reading);
        }
        else if (reading is > Int16.MaxValue or >= 0)
        {
            prefixWriteLocation = sizeof(UInt16);
            BinaryPrimitives.WriteUInt16LittleEndian(dataWriteLocation, (UInt16)reading);
        }
        else
        {
            prefixWriteLocation = SignedPrefix(sizeof(Int16));
            BinaryPrimitives.WriteInt16LittleEndian(dataWriteLocation, (Int16)reading);
        }
    }

    private static byte SignedPrefix(byte unsignedPrefix) => (byte)(SignedPrefixIdentifier - unsignedPrefix);

    private static bool IsValidPrefix(sbyte prefix) => prefix is (<= MaxByteCount and >= (-MaxByteCount));

    private static long ReadReading(ReadOnlySpan<byte> readingData, sbyte prefix) =>
        prefix switch
        {
            (-8) or 4 or 2 => BinaryPrimitives.ReadInt64LittleEndian(readingData),
            (-4) => BinaryPrimitives.ReadInt32LittleEndian(readingData),
            (-2) => BinaryPrimitives.ReadInt16LittleEndian(readingData),
            _ => 0
        };
}
