using System;

public static class PhoneNumber
{
    public static (bool IsNewYork, bool IsFake, string LocalNumber) Analyze(string phoneNumber)
    {
        var parsedPhoneNumber = ParsePhoneNumber(phoneNumber);
        return (IsNewYorkDialingCode(parsedPhoneNumber.DialingCode), IsFakePrefixCode(parsedPhoneNumber.PrefixCode), parsedPhoneNumber.LocalNumber);
    }

    public static bool IsFake((bool IsNewYork, bool IsFake, string LocalNumber) phoneNumberInfo)
    {
        return phoneNumberInfo.IsFake;
    }

    private static (string DialingCode, string PrefixCode, string LocalNumber) ParsePhoneNumber(string phoneNumber)
    {
        const char PhoneNumberDelimiter = '-';
        var phoneNumberParts = phoneNumber.Split(PhoneNumberDelimiter);
        return (phoneNumberParts[0], phoneNumberParts[1], phoneNumberParts[2]);
    }

    private static bool IsNewYorkDialingCode(string dialingCode)
    {
        return dialingCode == "212";
    }

    private static bool IsFakePrefixCode(string prefixCode)
    {
        return prefixCode == "555";
    }
}
