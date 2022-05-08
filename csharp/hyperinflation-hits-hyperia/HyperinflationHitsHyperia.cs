using System;

public static class CentralBank
{
    private const string OverflowMessage = "*** Too Big ***";

    public static string DisplayDenomination(long @base, long multiplier)
    {
        try
        {
            long denomination = checked(@base * multiplier);
            return denomination.ToString();
        }
        catch (OverflowException)
        {
            return OverflowMessage;
        }
    }

    public static string DisplayGDP(float @base, float multiplier)
    {
        try
        {
            float gdp = checked(@base * multiplier);
            if (float.IsInfinity(gdp))
            {
                return OverflowMessage;
            }
            return gdp.ToString();
        }
        catch (OverflowException)
        {
            return OverflowMessage;
        }
    }

    public static string DisplayChiefEconomistSalary(decimal salaryBase, decimal multiplier)
    {
        const string TooBigSalaryMessage = "*** Much Too Big ***";
        try
        {
            decimal salary = checked(salaryBase * multiplier);
            return salary.ToString();
        }
        catch (OverflowException)
        {
            return TooBigSalaryMessage;
        }
    }
}
