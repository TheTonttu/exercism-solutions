using System;

public static class RealNumberExtension
{
    public static double Expreal(this int realNumber, RationalNumber r) =>
        r.Expreal(realNumber);
}

public readonly record struct RationalNumber
{
    public readonly int Numerator;
    public readonly int Denominator;

    public RationalNumber(int numerator, int denominator)
    {
        if (denominator < 0)
        {
            numerator = -numerator;
            denominator = -denominator;
        }

        (Numerator, Denominator) = Reduce(numerator, denominator);
    }

    public static RationalNumber operator +(RationalNumber r1, RationalNumber r2)
    {
        int numerator = r1.Numerator * r2.Denominator + r2.Numerator * r1.Denominator;
        int denominator = r1.Denominator * r2.Denominator;
        return new(numerator, denominator);
    }

    public static RationalNumber operator -(RationalNumber r1, RationalNumber r2)
    {
        int numerator = r1.Numerator * r2.Denominator - r2.Numerator * r1.Denominator;
        int denominator = r1.Denominator * r2.Denominator;
        return new(numerator, denominator);
    }

    public static RationalNumber operator *(RationalNumber r1, RationalNumber r2) =>
        new(r1.Numerator * r2.Numerator, r1.Denominator * r2.Denominator);

    public static RationalNumber operator /(RationalNumber r1, RationalNumber r2) =>
        new(r1.Numerator * r2.Denominator, r2.Numerator * r1.Denominator);

    public RationalNumber Abs() => new(Math.Abs(Numerator), Math.Abs(Denominator));

    public RationalNumber Reduce() => Reduce(Numerator, Denominator);

    public RationalNumber Exprational(int power)
    {
        if (power >= 0)
        {
            return new((int)Math.Pow(Numerator, power), (int)Math.Pow(Denominator, power));
        }

        power = Math.Abs(power);
        return new((int)Math.Pow(Denominator, power), (int)Math.Pow(Numerator, power));
    }

    public double Expreal(int baseNumber) => Math.Pow(baseNumber, (double)Numerator / Denominator);

    private static (int Numerator, int Denominator) Reduce(int numerator, int denominator)
    {
        int gcd = GreatestCommonDivisor(numerator, denominator);
        return gcd == 0
            ? (numerator, denominator)
            : (numerator / gcd, denominator / gcd);
    }

    private static int GreatestCommonDivisor(int a, int b)
    {
        a = Math.Abs(a);
        b = Math.Abs(b);

        while (a != 0 && b != 0)
        {
            if (a > b)
            {
                a %= b;
            }
            else
            {
                b %= a;
            }
        }

        return a == 0 ? b : a;
    }

    public static implicit operator RationalNumber((int Numerator, int Denominator) t) =>
        new(t.Numerator, t.Denominator);
}