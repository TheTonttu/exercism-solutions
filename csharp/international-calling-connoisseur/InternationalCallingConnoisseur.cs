using System;
using System.Collections.Generic;
using System.Linq;

public static class DialingCodes
{
    public static Dictionary<int, string> GetEmptyDictionary() => new();

    public static Dictionary<int, string> GetExistingDictionary() =>
        new()
        {
            { 1, "United States of America" },
            { 55, "Brazil" },
            { 91, "India" },
        };

    public static Dictionary<int, string> AddCountryToEmptyDictionary(int CountryCode, string CountryName) =>
        new()
        {
            { CountryCode, CountryName },
        };

    public static Dictionary<int, string> AddCountryToExistingDictionary(
        Dictionary<int, string> existingDictionary, int countryCode, string countryName)
    {
        existingDictionary.Add(countryCode, countryName);
        return existingDictionary;
    }

    public static string GetCountryNameFromDictionary(
        Dictionary<int, string> existingDictionary, int countryCode)
    {
        if (existingDictionary.TryGetValue(countryCode, out string existingCountryCode))
        {
            return existingCountryCode;
        }
        return String.Empty;
    }
    public static Dictionary<int, string> UpdateDictionary(
        Dictionary<int, string> existingDictionary, int countryCode, string countryName)
    {
        if (existingDictionary.ContainsKey(countryCode))
        {
            existingDictionary[countryCode] = countryName;
        }
        return existingDictionary;
    }

    public static Dictionary<int, string> RemoveCountryFromDictionary(
        Dictionary<int, string> existingDictionary, int countryCode)
    {
        existingDictionary.Remove(countryCode);
        return existingDictionary;
    }

    public static bool CheckCodeExists(Dictionary<int, string> existingDictionary, int countryCode) =>
        existingDictionary.ContainsKey(countryCode);

    public static string FindLongestCountryName(Dictionary<int, string> existingDictionary) =>
        existingDictionary.Values.Aggregate(String.Empty, (longest, current) => longest.Length > current.Length ? longest : current);
}
