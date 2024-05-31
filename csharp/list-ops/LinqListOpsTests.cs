using System;
using System.Collections.Generic;
using Xunit;

public class LinqListOpsTests
{
    [Fact]
    public void Append_entries_to_a_list_and_return_the_new_list_empty_lists()
    {
        List<int> list1 = [];
        List<int> list2 = [];
        Assert.Empty(LinqListOps.Append(list1, list2));
    }

    [Fact]
    public void Append_entries_to_a_list_and_return_the_new_list_list_to_empty_list()
    {
        List<int> list1 = [];
        List<int> list2 = [1, 2, 3, 4];
        List<int> expected = [1, 2, 3, 4];
        Assert.Equal(expected, LinqListOps.Append(list1, list2));
    }

    [Fact]
    public void Append_entries_to_a_list_and_return_the_new_list_empty_list_to_list()
    {
        List<int> list1 = [1, 2, 3, 4];
        List<int> list2 = [];
        List<int> expected = [1, 2, 3, 4];
        Assert.Equal(expected, LinqListOps.Append(list1, list2));
    }

    [Fact]
    public void Append_entries_to_a_list_and_return_the_new_list_non_empty_lists()
    {
        List<int> list1 = [1, 2];
        List<int> list2 = [2, 3, 4, 5];
        List<int> expected = [1, 2, 2, 3, 4, 5];
        Assert.Equal(expected, LinqListOps.Append(list1, list2));
    }

    [Fact]
    public void Concatenate_a_list_of_lists_empty_list()
    {
        List<List<int>> lists = [];
        Assert.Empty(LinqListOps.Concat(lists));
    }

    [Fact]
    public void Concatenate_a_list_of_lists_list_of_lists()
    {
        List<List<int>> lists = [[1, 2], [3], [], [4, 5, 6]];
        List<int> expected = [1, 2, 3, 4, 5, 6];
        Assert.Equal(expected, LinqListOps.Concat(lists));
    }

    [Fact]
    public void Concatenate_a_list_of_lists_list_of_nested_lists()
    {
        List<List<List<int>>> lists = [[[1], [2]], [[3]], [[]], [[4, 5, 6]]];
        List<List<int>> expected = [[1], [2], [3], [], [4, 5, 6]];
        Assert.Equal(expected, LinqListOps.Concat(lists));
    }

    [Fact]
    public void Filter_list_returning_only_values_that_satisfy_the_filter_function_empty_list()
    {
        List<int> list = [];
        Func<int, bool> function = (x) => x % 2 == 1;
        Assert.Empty(LinqListOps.Filter(list, function));
    }

    [Fact]
    public void Filter_list_returning_only_values_that_satisfy_the_filter_function_non_empty_list()
    {
        List<int> list = [1, 2, 3, 5];
        Func<int, bool> function = (x) => x % 2 == 1;
        List<int> expected = [1, 3, 5];
        Assert.Equal(expected, LinqListOps.Filter(list, function));
    }

    [Fact]
    public void Returns_the_length_of_a_list_empty_list()
    {
        List<int> list = [];
        Assert.Equal(0, LinqListOps.Length(list));
    }

    [Fact]
    public void Returns_the_length_of_a_list_non_empty_list()
    {
        List<int> list = [1, 2, 3, 4];
        Assert.Equal(4, LinqListOps.Length(list));
    }

    [Fact]
    public void Return_a_list_of_elements_whose_values_equal_the_list_value_transformed_by_the_mapping_function_empty_list()
    {
        List<int> list = [];
        Func<int, int> function = (x) => x + 1;
        Assert.Empty(LinqListOps.Map(list, function));
    }

    [Fact]
    public void Return_a_list_of_elements_whose_values_equal_the_list_value_transformed_by_the_mapping_function_non_empty_list()
    {
        List<int> list = [1, 3, 5, 7];
        Func<int, int> function = (x) => x + 1;
        List<int> expected = [2, 4, 6, 8];
        Assert.Equal(expected, LinqListOps.Map(list, function));
    }

    [Fact]
    public void Folds_reduces_the_given_list_from_the_left_with_a_function_direction_dependent_function_applied_to_non_empty_list()
    {
        List<int> list = [2, 5];
        int initial = 5;
        Func<int, int, int> function = (x, y) => x / y;
        Assert.Equal(0, LinqListOps.Foldl(list, initial, function));
    }

    [Fact]
    public void Folds_reduces_the_given_list_from_the_left_with_a_function_empty_list()
    {
        List<int> list = [];
        int initial = 2;
        Func<int, int, int> function = (acc, el) => el * acc;
        Assert.Equal(2, LinqListOps.Foldl(list, initial, function));
    }

    [Fact]
    public void Folds_reduces_the_given_list_from_the_left_with_a_function_direction_independent_function_applied_to_non_empty_list()
    {
        List<int> list = [1, 2, 3, 4];
        int initial = 5;
        Func<int, int, int> function = (acc, el) => el + acc;
        Assert.Equal(15, LinqListOps.Foldl(list, initial, function));
    }

    [Fact]
    public void Folds_reduces_the_given_list_from_the_right_with_a_function_direction_dependent_function_applied_to_non_empty_list()
    {
        List<int> list = [2, 5];
        int initial = 5;
        Func<int, int, int> function = (x, y) => x / y;
        Assert.Equal(2, LinqListOps.Foldr(list, initial, function));
    }

    [Fact]
    public void Folds_reduces_the_given_list_from_the_right_with_a_function_empty_list()
    {
        List<int> list = [];
        int initial = 2;
        Func<int, int, int> function = (acc, el) => el * acc;
        Assert.Equal(2, LinqListOps.Foldr(list, initial, function));
    }

    [Fact]
    public void Folds_reduces_the_given_list_from_the_right_with_a_function_direction_independent_function_applied_to_non_empty_list()
    {
        List<int> list = [1, 2, 3, 4];
        int initial = 5;
        Func<int, int, int> function = (acc, el) => el + acc;
        Assert.Equal(15, LinqListOps.Foldr(list, initial, function));
    }

    [Fact]
    public void Reverse_the_elements_of_the_list_empty_list()
    {
        List<int> list = [];
        Assert.Empty(LinqListOps.Reverse(list));
    }

    [Fact]
    public void Reverse_the_elements_of_the_list_non_empty_list()
    {
        List<int> list = [1, 3, 5, 7];
        List<int> expected = [7, 5, 3, 1];
        Assert.Equal(expected, LinqListOps.Reverse(list));
    }

    [Fact]
    public void Reverse_the_elements_of_the_list_list_of_lists_is_not_flattened()
    {
        List<List<int>> list = [[1, 2], [3], [], [4, 5, 6]];
        List<List<int>> expected = [[4, 5, 6], [], [3], [1, 2]];
        Assert.Equal(expected, LinqListOps.Reverse(list));
    }
}
