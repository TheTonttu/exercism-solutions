using System;
using System.Collections.Generic;
using System.Linq;

public static class Languages
{
    public static List<string> NewList() => new();

    public static List<string> GetExistingLanguages() => new() { "C#", "Clojure", "Elm" };

    public static List<string> AddLanguage(List<string> languages, string language) =>
        new(languages.Append(language));

    public static int CountLanguages(List<string> languages) => languages.Count();

    public static bool HasLanguage(List<string> languages, string language) =>
        languages.Contains(language, StringComparer.OrdinalIgnoreCase);

    public static List<string> ReverseList(List<string> languages) =>
        new(languages.Reverse<string>());

    public static bool IsExciting(List<string> languages) =>
        PositionOnList(languages, "C#") switch
        {
            0 => true,
            1 => languages.Count is 2 or 3,
            _ => false
        };

    public static List<string> RemoveLanguage(List<string> languages, string language) =>
        new(languages.Where(lng => !LanguagesMatch(lng, language)));

    public static bool IsUnique(List<string> languages)
    {
        var uniques = new HashSet<string>();
        return languages.All(lng => uniques.Add(lng));
    }

    private static int PositionOnList(List<string> languages, string language) =>
        languages.FindIndex(lng => LanguagesMatch(lng, language));

    private static bool LanguagesMatch(string language1, string language2) =>
        String.Equals(language1, language2, StringComparison.OrdinalIgnoreCase);
}

#region Alternative List Mutating implementation

public static class ListMutatingLanguages
{
    public static List<string> NewList() => new();

    public static List<string> GetExistingLanguages() => new() { "C#", "Clojure", "Elm" };

    public static List<string> AddLanguage(List<string> languages, string language)
    {
        languages.Add(language);
        return languages;
    }

    public static int CountLanguages(List<string> languages) => languages.Count();

    public static bool HasLanguage(List<string> languages, string language) =>
        languages.Contains(language, StringComparer.OrdinalIgnoreCase);

    public static List<string> ReverseList(List<string> languages) =>
        new(languages.Reverse<string>());

    public static bool IsExciting(List<string> languages) =>
        PositionOnList(languages, "C#") switch
        {
            0 => true,
            1 => languages.Count is 2 or 3,
            _ => false
        };

    public static List<string> RemoveLanguage(List<string> languages, string language)
    {
        languages.Remove(language);
        return languages;
    }

    public static bool IsUnique(List<string> languages)
    {
        var uniques = new HashSet<string>();
        return languages.All(lng => uniques.Add(lng));
    }

    private static int PositionOnList(List<string> languages, string language) =>
        languages.FindIndex(lng => LanguagesMatch(lng, language));

    private static bool LanguagesMatch(string language1, string language2) =>
        String.Equals(language1, language2, StringComparison.OrdinalIgnoreCase);
}

#endregion
