using System;

public static class SimpleCalculator
{
    private enum Operation
    {
        Addition,
        Multiplication,
        Division,
    }

    public static string Calculate(int operand1, int operand2, string operation)
    {
        try
        {
            int calculationResult = PerformCalculation(operand1, operand2, ParseOperation(operation));
            return FormatCalculation(operand1, operand2, operation, calculationResult);
        }
        catch (DivideByZeroException)
        {
            return "Division by zero is not allowed.";
        }
    }

    private static Operation ParseOperation(string operation) => operation switch
    {
        "+" => Operation.Addition,
        "*" => Operation.Multiplication,
        "/" => Operation.Division,
        "" => throw new ArgumentException($"'{nameof(operation)}' cannot be empty.", nameof(operation)),
        null => throw new ArgumentNullException(nameof(operation)),
        _ => throw new ArgumentOutOfRangeException(nameof(operation)),
    };

    private static int PerformCalculation(int operand1, int operand2, Operation operation) => operation switch
    {
        Operation.Addition => SimpleOperation.Addition(operand1, operand2),
        Operation.Multiplication => SimpleOperation.Multiplication(operand1, operand2),
        Operation.Division => SimpleOperation.Division(operand1, operand2),
        _ => throw new NotSupportedException(operation.ToString())
    };

    private static string FormatCalculation(int operand1, int operand2, string operation, int result) =>
        $"{operand1} {operation} {operand2} = {result}";
}
