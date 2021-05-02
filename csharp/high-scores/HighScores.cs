using System;
using System.Collections.Generic;
using System.Linq;

public class HighScores
{
    private readonly List<int> _scores;

    public HighScores(List<int> list)
    {
        _scores = new List<int>(list);
    }

    public List<int> Scores()
    {
        return new List<int>(_scores);
    }

    public int Latest()
    {
        return _scores.Last();
    }

    public int PersonalBest()
    {
        return _scores.Max();
    }

    public List<int> PersonalTopThree()
    {
        return _scores
            .OrderByDescending(n => n)
            .Take(3)
            .ToList();
    }
}