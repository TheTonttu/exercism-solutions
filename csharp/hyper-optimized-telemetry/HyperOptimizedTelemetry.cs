using System;
using System.Buffers.Binary;

public static class TelemetryBuffer
{
    private const int PrefixByteIndex = 0;
    private const short SignedPrefixIdentifier = 256;
    private const int DataSectionStartIndex = 1;

    public static byte[] ToBuffer(long reading)
    {
        var buffer = new byte[9];
        Type fittedType = DetermineFittedWriteType(reading);

        if (fittedType == typeof(Int64))
        {
            WriteReading<Int64>(buffer, reading);
        }
        else if (fittedType == typeof(UInt32))
        {
            WriteReading<UInt32>(buffer, reading);
        }
        else if (fittedType == typeof(Int32))
        {
            WriteReading<Int32>(buffer, reading);
        }
        else if (fittedType == typeof(UInt16))
        {
            WriteReading<UInt16>(buffer, reading);
        }
        else
        {
            WriteReading<Int16>(buffer, reading);
        }
        return buffer;
    }

    public static long FromBuffer(byte[] buffer)
    {
        byte prefix = buffer[PrefixByteIndex];
        (bool isSigned, byte byteCount) = ParsePrefix(prefix);

        return ComposeReading(buffer, isSigned, byteCount);
    }

    private static Type DetermineFittedWriteType(long reading) =>
    reading switch
    {
        > UInt32.MaxValue or < Int32.MinValue => typeof(Int64),
        > Int32.MaxValue => typeof(UInt32),
        > UInt16.MaxValue or < Int16.MinValue => typeof(Int32),
        > short.MaxValue or >= 0 => typeof(UInt16),
        _ => typeof(Int16),
    };

    private static void WriteReading<T>(Span<byte> destination, long reading)
    {
        ref byte prefixWriteLocation = ref destination[PrefixByteIndex];
        var dataWriteLocation = destination[DataSectionStartIndex..];
        Type t = typeof(T);
        if (t == typeof(Int64))
        {
            prefixWriteLocation = SignedPrefixIdentifier - sizeof(Int64);
            BinaryPrimitives.WriteInt64LittleEndian(dataWriteLocation, reading);
        }
        else if (t == typeof(UInt32))
        {
            prefixWriteLocation = sizeof(UInt32);
            BinaryPrimitives.WriteUInt32LittleEndian(dataWriteLocation, (UInt32)reading);
        }
        else if (t == typeof(Int32))
        {
            prefixWriteLocation = SignedPrefixIdentifier - sizeof(Int32);
            BinaryPrimitives.WriteInt32LittleEndian(dataWriteLocation, (Int32)reading);
        }
        else if (t == typeof(UInt16))
        {
            prefixWriteLocation = sizeof(UInt16);
            BinaryPrimitives.WriteUInt16LittleEndian(dataWriteLocation, (UInt16)reading);
        }
        else if (t == typeof(Int16))
        {
            prefixWriteLocation = SignedPrefixIdentifier - sizeof(Int16);
            BinaryPrimitives.WriteInt16LittleEndian(dataWriteLocation, (Int16)reading);
        }
    }

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
