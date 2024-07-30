using System;
using System.Collections.Generic;

public enum YachtCategory
{
    Ones = 1,
    Twos = 2,
    Threes = 3,
    Fours = 4,
    Fives = 5,
    Sixes = 6,
    FullHouse = 7,
    FourOfAKind = 8,
    LittleStraight = 9,
    BigStraight = 10,
    Choice = 11,
    Yacht = 12,
}

public static class YachtGame
{
    private const int DieSides = 6;
    // 1:1 die to index mapping. Extra element due to 0-based indexing.
    private const int RollArraySize = DieSides + 1;

    public static int Score(int[] dice, YachtCategory category)
    {
        return category switch
        {
            YachtCategory.Ones => ScoreDiceRolls(dice, 1),
            YachtCategory.Twos => ScoreDiceRolls(dice, 2),
            YachtCategory.Threes => ScoreDiceRolls(dice, 3),
            YachtCategory.Fours => ScoreDiceRolls(dice, 4),
            YachtCategory.Fives => ScoreDiceRolls(dice, 5),
            YachtCategory.Sixes => ScoreDiceRolls(dice, 6),
            YachtCategory.FullHouse => ScoreFullHouse(dice),
            YachtCategory.FourOfAKind => ScoreFourOfAKind(dice),
            YachtCategory.LittleStraight => ScoreLittleStraight(dice),
            YachtCategory.BigStraight => ScoreBigStraight(dice),
            YachtCategory.Choice => ScoreChoice(dice),
            YachtCategory.Yacht => ScoreYacht(dice),
            _ => throw new NotImplementedException($"{category}"),
        };
    }

    private static int ScoreDiceRolls(int[] dice, int referenceDie)
    {
        int count = 0;
        foreach (int die in dice)
        {
            if (die == referenceDie)
            {
                count++;
            }
        }
        return count * referenceDie;
    }

    private static int ScoreFullHouse(int[] dice)
    {
        List<int> countCriteria = [2, 3];
        var dictionary = new Dictionary<int, int>();
        foreach (int die in dice)
        {
            if (!dictionary.TryGetValue(die, out int count))
            {
                count = 0;
            }
            dictionary[die] = ++count;
        }

        int score = 0;
        foreach ((int die, int count) in dictionary)
        {
            if (!countCriteria.Remove(count))
            {
                return 0;
            }
            score += die * count;
        }

        if (countCriteria.Count != 0)
        {
            return 0;
        }

        return score;
    }

    private static int ScoreFourOfAKind(int[] dice)
    {
        var dictionary = new Dictionary<int, int>();
        foreach (int die in dice)
        {
            if (!dictionary.TryGetValue(die, out int count))
            {
                count = 0;
            }

            count++;
            if (count == 4)
            {
                return die * count;
            }
            dictionary[die] = count;
        }
        return 0;
    }

    private static int ScoreLittleStraight(int[] dice)
    {
        const int LittleStraightScore = 30;

        Span<int> rolls = stackalloc int[RollArraySize];

        foreach (var die in dice)
        {
            rolls[die]++;
        }

        // 1-5
        ReadOnlySpan<int> littleStraightRolls = rolls.Slice(1, 5);
        foreach (int count in littleStraightRolls)
        {
            if (count != 1)
            {
                return 0;
            }
        }

        return LittleStraightScore;
    }

    private static int ScoreBigStraight(int[] dice)
    {
        const int BigStraightScore = 30;

        Span<int> rolls = stackalloc int[RollArraySize];

        foreach (var die in dice)
        {
            rolls[die]++;
        }

        // 2-6
        ReadOnlySpan<int> bigStraightRolls = rolls.Slice(2, 5);
        foreach (int count in bigStraightRolls)
        {
            if (count != 1)
            {
                return 0;
            }
        }

        return BigStraightScore;
    }

    private static int ScoreChoice(int[] dice)
    {
        int sum = 0;
        foreach (int die in dice)
        {
            sum += die;
        }
        return sum;
    }

    private static int ScoreYacht(int[] dice)
    {
        const int YachtScore = 50;

        int totalRolls = dice.Length;
        Span<int> rolls = stackalloc int[RollArraySize];
        foreach (var die in dice)
        {
            if (++rolls[die] == totalRolls)
            {
                return YachtScore;
            }
        }

        return 0;
    }
}

