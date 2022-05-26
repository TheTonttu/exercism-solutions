using System.Collections.Generic;
using System.Collections.ObjectModel;

public class Authenticator
{
    private class EyeColor
    {
        public const string Blue = "blue";
        public const string Green = "green";
        public const string Brown = "brown";
        public const string Hazel = "hazel";
        public const string Grey = "grey";
    }
    public Authenticator(Identity admin)
    {
        Admin = admin;
    }

    private readonly Dictionary<string, Identity> _developers = new()
    {
        ["Bertrand"] = new Identity{ Email = "bert@ex.ism", EyeColor = EyeColor.Blue },
        ["Anders"] = new Identity{ Email = "anders@ex.ism", EyeColor = EyeColor.Brown }
    };

    public Identity Admin { get; }

    // Changing return type to IReadOnlyDictionary return type would convey intent better but that would be a breaking change.
    public IDictionary<string, Identity> GetDevelopers()
    {
        return new ReadOnlyDictionary<string, Identity>(_developers);
    }
}

// Immutable record class would probably convey intent better but that would be a breaking change.
public readonly struct Identity
{
    public readonly string Email { get; init; }
    public readonly string EyeColor { get; init; }
}
