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

        Numerator = numerator;
        Denominator = denominator;
    }

    public static RationalNumber operator +(RationalNumber r1, RationalNumber r2)
    {
        int numerator = r1.Numerator * r2.Denominator + r2.Numerator * r1.Denominator;
        int denominator = r1.Denominator * r2.Denominator;
        var add = new RationalNumber(numerator, denominator);
        return add.Reduce();
    }

    public static RationalNumber operator -(RationalNumber r1, RationalNumber r2)
    {
        int numerator = r1.Numerator * r2.Denominator - r2.Numerator * r1.Denominator; ;
        int denominator = r1.Denominator * r2.Denominator;
        var sub = new RationalNumber(numerator, denominator);
        return sub.Reduce();
    }

    public static RationalNumber operator *(RationalNumber r1, RationalNumber r2)
    {
        var multi = new RationalNumber(r1.Numerator * r2.Numerator, r1.Denominator * r2.Denominator);
        return multi.Reduce();
    }

    public static RationalNumber operator /(RationalNumber r1, RationalNumber r2) =>
        new(r1.Numerator * r2.Denominator, r2.Numerator * r1.Denominator);

    public RationalNumber Abs() {
        var abs = new RationalNumber(Math.Abs(Numerator), Math.Abs(Denominator));
        return abs.Reduce();
    }

    public RationalNumber Reduce()
    {
        int gcd = GreatestCommonDivisor(Numerator, Denominator);
        return new(Numerator / gcd, Denominator / gcd);
    }

    public RationalNumber Exprational(int power)
    {
        if (power >= 0)
        {
            return new RationalNumber((int)Math.Pow(Numerator, power), (int)Math.Pow(Denominator, power));
        }

        power = Math.Abs(power);
        return new RationalNumber((int)Math.Pow(Denominator, power), (int)Math.Pow(Numerator, power));
    }

    public double Expreal(int baseNumber) => Math.Pow(baseNumber, (double)Numerator / Denominator);

    private int GreatestCommonDivisor(int a, int b)
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
}