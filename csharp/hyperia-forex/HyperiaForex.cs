using System;

public struct CurrencyAmount
{
    private readonly decimal amount;
    private readonly string currency;

    public CurrencyAmount(decimal amount, string currency)
    {
        this.amount = amount;
        this.currency = currency;
    }

    public override bool Equals(object obj) => 
        obj is CurrencyAmount other &&
        this == other;

    public override int GetHashCode() => HashCode.Combine(amount, currency);

    public static bool operator ==(CurrencyAmount left, CurrencyAmount right)
    {
        GuardCurrency(left, right);
        return left.amount == right.amount
            && left.currency == right.currency;
    }

    public static bool operator !=(CurrencyAmount left, CurrencyAmount right) => !(left == right);

    public static bool operator <(CurrencyAmount left, CurrencyAmount right)
    {
        GuardCurrency(left, right);
        return left.amount < right.amount;
    }

    public static bool operator >(CurrencyAmount left, CurrencyAmount right)
    {
        GuardCurrency(left, right);
        return left.amount > right.amount;
    }

    public static bool operator <=(CurrencyAmount left, CurrencyAmount right)
    {
        GuardCurrency(left, right);
        return left.amount <= right.amount;
    }

    public static bool operator >=(CurrencyAmount left, CurrencyAmount right)
    {
        GuardCurrency(left, right);
        return left.amount >= right.amount;
    }

    public static CurrencyAmount operator +(CurrencyAmount left, CurrencyAmount right)
    {
        GuardCurrency(left, right);
        return new(left.amount + right.amount, left.currency);
    }

    public static CurrencyAmount operator -(CurrencyAmount left, CurrencyAmount right)
    {
        GuardCurrency(left, right);
        return new(left.amount - right.amount, left.currency);
    }

    public static CurrencyAmount operator *(CurrencyAmount left, CurrencyAmount right)
    {
        GuardCurrency(left, right);
        return new(left.amount * right.amount, left.currency);
    }

    public static CurrencyAmount operator /(CurrencyAmount left, CurrencyAmount right)
    {
        GuardCurrency(left, right);
        return new(left.amount / right.amount, left.currency);
    }

    public static CurrencyAmount operator *(decimal left, CurrencyAmount right) => new(left * right.amount, right.currency);
    public static CurrencyAmount operator *(CurrencyAmount left, decimal right) => new(left.amount * right, left.currency);
    public static CurrencyAmount operator /(decimal left, CurrencyAmount right) => new(left / right.amount, right.currency);
    public static CurrencyAmount operator /(CurrencyAmount left, decimal right) => new(left.amount / right, left.currency);

    public static explicit operator double(CurrencyAmount currencyAmount) => (double)currencyAmount.amount;
    public static implicit operator decimal(CurrencyAmount currencyAmount) => currencyAmount.amount;

    private static void GuardCurrency(CurrencyAmount left, CurrencyAmount right)
    {
        if (left.currency != right.currency)
        {
            throw new ArgumentException("Currencies are different.");
        }
    }
}
