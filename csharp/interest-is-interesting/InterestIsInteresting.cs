using System;

static class SavingsAccount
{
    public static float InterestRate(decimal balance)
    {
        if (balance >= 5000) { return 2.475f; }
        if (balance >= 1000) { return 1.621f; }
        if (balance >= 0) { return 0.5f; }
        return -3.213f;
    }

    public static decimal Interest(decimal balance)
    {
        return balance * InterestPercentage(balance);
    }

    public static decimal AnnualBalanceUpdate(decimal balance)
    {
        return balance + Interest(balance);
    }

    public static int YearsBeforeDesiredBalance(decimal balance, decimal targetBalance)
    {
        int years = 0;
        while (balance < targetBalance)
        {
            balance = AnnualBalanceUpdate(balance);
            years++;
        }
        return years;
    }

    private static decimal InterestPercentage(decimal balance)
    {
        return (decimal)Math.Abs(InterestRate(balance) / 100.0f);
    }
}
