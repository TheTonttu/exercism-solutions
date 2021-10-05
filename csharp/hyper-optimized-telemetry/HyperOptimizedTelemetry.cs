using System;

public static class TelemetryBuffer
{
    public static byte[] ToBuffer(long reading)
    {
        const int PrefixByteIndex = 0;
        const short SignedPrefixIdentifier = 256;

        var bytes = new byte[9];
        var extractedBytes = BitConverter.GetBytes(reading);
        extractedBytes.CopyTo(bytes, 1);
        if (reading > 0)
        {
            if (FitsPositiveLong(reading))
            {
                bytes[PrefixByteIndex] = SignedPrefixIdentifier - 8;
            }
            else if (FitsUInt32(reading))
            {
                bytes[PrefixByteIndex] = 4;
            }
            else if (FitPositiveInt32(reading))
            {
                bytes[PrefixByteIndex] = SignedPrefixIdentifier - 4;
            }
            else if (FitsUInt16(reading))
            {
                bytes[PrefixByteIndex] = 2;
            }
            else
            {
                bytes[PrefixByteIndex] = SignedPrefixIdentifier - 2;
            }
        }
        else
        {
            if (FitsNegativeLong(reading))
            {
                bytes[PrefixByteIndex] = SignedPrefixIdentifier - 8;
            }
            else if (FitsNegativeInt32(reading))
            {
                bytes[PrefixByteIndex] = SignedPrefixIdentifier - 4;
                for (int i = 5; i < bytes.Length; i++)
                {
                    bytes[i] = 0;
                }
            }
            else
            {
                bytes[PrefixByteIndex] = 256 - 2;
                for (int i = 3; i < bytes.Length; i++)
                {
                    bytes[i] = 0;
                }
            }
        }

        return bytes;
    }

    private static bool FitsNegativeInt32(long reading)
    {
        return reading < Int16.MinValue;
    }

    private static bool FitsNegativeLong(long reading)
    {
        return reading < Int32.MinValue;
    }

    private static bool FitsUInt16(long reading)
    {
        return reading > Int16.MaxValue;
    }

    private static bool FitPositiveInt32(long reading)
    {
        return reading > UInt16.MaxValue;
    }

    private static bool FitsUInt32(long reading)
    {
        return reading > Int32.MaxValue;
    }

    private static bool FitsPositiveLong(long reading)
    {
        return reading > UInt32.MaxValue;
    }

    public static long FromBuffer(byte[] buffer)
    {
        throw new NotImplementedException("Please implement the static TelemetryBuffer.FromBuffer() method");
    }
}
