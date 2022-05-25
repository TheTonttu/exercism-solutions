using System;
using System.Collections.Generic;

public struct Coord : IEquatable<Coord>
{
    public Coord(ushort x, ushort y)
    {
        X = x;
        Y = y;
    }

    public ushort X { get; }
    public ushort Y { get; }

    public bool Equals(Coord other) =>
        this.X == other.X &&
        this.Y == other.Y;

    public override bool Equals(object obj) => obj is Coord other && Equals(other);

    public override int GetHashCode() => HashCode.Combine(X, Y);

    public static bool operator ==(Coord left, Coord right) => left.Equals(right);

    public static bool operator !=(Coord left, Coord right) => !(left == right);

    /// <summary>
    /// Calculates distance between left and right coordinate.
    /// </summary>
    /// <param name="left"></param>
    /// <param name="right"></param>
    /// <returns>Distance between given coordinates.</returns>
    public static double operator +(Coord left, Coord right)
    {
        double a2 = Math.Pow(left.X - right.X, 2.0);
        double b2 = Math.Pow(left.Y - right.Y, 2.0);
        return Math.Sqrt(a2 + b2);
    }
}

public struct Plot : IEquatable<Plot>
{
    public Plot(Coord a, Coord b, Coord c, Coord d)
    {
        A = a;
        B = b;
        C = c;
        D = d;
    }

    public Coord A { get; }
    public Coord B { get; }
    public Coord C { get; }
    public Coord D { get; }

    public bool Equals(Plot other) => 
        this.A == other.A &&
        this.B == other.B &&
        this.C == other.C &&
        this.D == other.D;

    public override bool Equals(object obj) => obj is Plot other && Equals(other);

    public override int GetHashCode() => HashCode.Combine(A, B, C, D);

    public static bool operator ==(Plot left, Plot right) => left.Equals(right);

    public static bool operator !=(Plot left, Plot right) => !(left == right);
}


public class ClaimsHandler
{
    private readonly HashSet<Plot> _claims = new();
    private Plot _lastClaim;

    public void StakeClaim(Plot plot)
    {
        if (_claims.Add(plot))
        {
            _lastClaim = plot;
        }
    }

    public bool IsClaimStaked(Plot plot)
    {
        return _claims.Contains(plot);
    }

    public bool IsLastClaim(Plot plot)
    {
        return _lastClaim == plot;
    }

    public Plot GetClaimWithLongestSide()
    {
        Plot claimWithLongestSide = default;
        double longestSide = default;

        foreach (var plot in _claims)
        {
            double plotLongestSide = GetLongestSideLength(plot);
            if (plotLongestSide > longestSide)
            {
                claimWithLongestSide = plot;
                longestSide = plotLongestSide;
            }
        }

        return claimWithLongestSide;
    }

    private static double GetLongestSideLength(Plot plot)
    {
        double sideAB = plot.A + plot.B;
        double sideBC = plot.B + plot.C;
        double sideCD = plot.C + plot.D;
        double sideDA = plot.D + plot.A;
        Span<double> sides = stackalloc[] { sideAB, sideBC, sideCD, sideDA };
        return GetMax(sides);
    }

    private static double GetMax(Span<double> values)
    {
        double maxValue = default;
        foreach (double v in values)
        {
            if (v > maxValue)
            {
                maxValue = v;
            }
        }
        return maxValue;
    }
}
