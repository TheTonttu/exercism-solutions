using System;
using System.Collections.Generic;

public static class ListOps
{
    public static int Length<T>(List<T> input)
    {
        return input.Count;
    }

    public static List<T> Reverse<T>(List<T> input)
    {
        var output = new List<T>(capacity: input.Count);
        for (int i = input.Count - 1; i >= 0; i--)
        {
            output.Add(input[i]);
        }
        return output;

        // Alternative:
        //var output = new List<T>(input);
        //output.Reverse();
        //return output;
    }

    public static List<TOut> Map<TIn, TOut>(List<TIn> input, Func<TIn, TOut> map)
    {
        var output = new List<TOut>(capacity: input.Count);
        foreach (TIn item in input)
        {
            output.Add(map(item));
        }
        return output;
    }

    public static List<T> Filter<T>(List<T> input, Func<T, bool> predicate)
    {
        var output = new List<T>();
        foreach (T item in input)
        {
            if (predicate(item))
            {
                output.Add(item);
            }
        }
        return output;
    }

    public static TOut Foldl<TIn, TOut>(List<TIn> input, TOut start, Func<TOut, TIn, TOut> func)
    {
        TOut output = start;
        foreach (TIn item in input)
        {
            output = func(output, item);
        }
        return output;
    }

    public static TOut Foldr<TIn, TOut>(List<TIn> input, TOut start, Func<TIn, TOut, TOut> func)
    {
        TOut output = start;
        for (int i = input.Count - 1; i >= 0; i--)
        {
            TIn item = input[i];
            output = func(item, output);
        }
        return output;
    }

    public static List<T> Concat<T>(List<List<T>> input)
    {
        var output = new List<T>();
        foreach (var subList in input)
        {
            // output.AddRange(list);
            output.EnsureCapacity(output.Count + subList.Count);
            foreach (T item in subList)
            {
                output.Add(item);
            }
        }
        return output;
    }

    public static List<T> Append<T>(List<T> left, List<T> right)
    {
        var output = new List<T>(capacity: left.Count + right.Count);
        foreach (T item in left)
        {
            output.Add(item);
        }
        foreach (T item in right)
        {
            output.Add(item);
        }
        return output;
    }
}