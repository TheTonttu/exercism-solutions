using System;
using System.Collections.Generic;

// public record FacialFeatures(string EyeColor, decimal PhiltrumWidth);
public class FacialFeatures : IEquatable<FacialFeatures>
{
    public string EyeColor { get; }
    public decimal PhiltrumWidth { get; }

    public FacialFeatures(string eyeColor, decimal philtrumWidth)
    {
        EyeColor = eyeColor;
        PhiltrumWidth = philtrumWidth;
    }

    public bool Equals(FacialFeatures? other)
    {
        return other != null &&
            (
                ReferenceEquals(this, other)
                ||
                this.EyeColor == other.EyeColor &&
                this.PhiltrumWidth == other.PhiltrumWidth
            );
    }

    public override bool Equals(object obj)
    {
        return obj is FacialFeatures other
            && Equals(other);
    }

    public override int GetHashCode()
    {
        return HashCode.Combine(EyeColor, PhiltrumWidth);
    }

    public static bool operator ==(FacialFeatures? lhs, FacialFeatures? rhs)
    {
        return lhs is null && rhs is null
            || Equals(lhs, rhs);
    }

    public static bool operator !=(FacialFeatures? lhs, FacialFeatures? rhs)
    {
        return !(lhs == rhs);
    }
}

// public record Identity(string Email, FacialFeatures FacialFeatures);
public class Identity : IEquatable<Identity>
{
    public string Email { get; }
    public FacialFeatures FacialFeatures { get; }

    public Identity(string email, FacialFeatures facialFeatures)
    {
        Email = email;
        FacialFeatures = facialFeatures;
    }

    public bool Equals(Identity? other)
    {
        return other != null &&
            (
                ReferenceEquals(this, other)
                ||
                this.Email == other.Email &&
                this.FacialFeatures == other.FacialFeatures
            );
    }

    public override bool Equals(object obj)
    {
        return obj is Identity other
            && Equals(other);
    }

    public override int GetHashCode()
    {
        return HashCode.Combine(Email, FacialFeatures);
    }

    public static bool operator ==(Identity? lhs, Identity? rhs)
    {
        return lhs is null && rhs is null
            || Equals(lhs, rhs);
    }

    public static bool operator !=(Identity? lhs, Identity? rhs)
    {
        return !(lhs == rhs);
    }
}

public class Authenticator
{
    private static readonly Identity Admin = new("admin@exerc.ism", new FacialFeatures("green", 0.9m));

    private readonly HashSet<Identity> RegisteredIdentities = new();

    public static bool AreSameFace(FacialFeatures faceA, FacialFeatures faceB)
    {
        return faceA == faceB;
    }

    public bool IsAdmin(Identity identity)
    {
        return identity == Admin;
    }

    public bool Register(Identity identity)
    {
        return RegisteredIdentities.Add(identity);
    }

    public bool IsRegistered(Identity identity)
    {
        return RegisteredIdentities.Contains(identity);
    }

    public static bool AreSameObject(Identity identityA, Identity identityB)
    {
        return ReferenceEquals(identityA, identityB);
    }
}
