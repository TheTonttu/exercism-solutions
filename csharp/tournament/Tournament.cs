using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

public static class Tournament
{
    private static readonly string OutputLineTemplate = "{0,-30} | {1,2} | {2,2} | {3,2} | {4,2} | {5,2}";
    private static readonly string Header = string.Format(OutputLineTemplate, "Team", "MP", "W", "D", "L", "P");

    public static void Tally(Stream inStream, Stream outStream)
    {
        var statistics = ReadStatistics(inStream);
        WriteSummary(outStream, statistics);
    }

    private static void WriteSummary(Stream outStream, TournamentStatistics statistics)
    {
        using (var writer = new StreamWriter(outStream, leaveOpen: true))
        {
            writer.Write(Header);
            foreach (var teamStats in statistics.GetStatistics())
            {
                writer.Write('\n');
                string formattedLine = string.Format(OutputLineTemplate, teamStats.Name, teamStats.Plays, teamStats.Wins, teamStats.Draws, teamStats.Losses, teamStats.Points);
                writer.Write(formattedLine);
            }
        }
    }

    private static TournamentStatistics ReadStatistics(Stream inStream)
    {
        var statistics = new TournamentStatistics();

        using (var reader = new StreamReader(inStream, leaveOpen: true))
        {
            string matchData;
            while ((matchData = reader.ReadLine()) != null)
            {
                var match = TournamentMatch.FromData(matchData);

                switch (match.Result)
                {
                    case "win":
                        statistics.Win(match.HomeTeamName, match.VisitingTeamName);
                        break;
                    case "loss":
                        statistics.Loss(match.HomeTeamName, match.VisitingTeamName);
                        break;
                    case "draw":
                        statistics.Draw(match.HomeTeamName, match.VisitingTeamName);
                        break;
                    default: throw new NotSupportedException($"Unsupported result: {match.Result}");
                }
            }
        }

        return statistics;
    }
}

internal class TournamentMatch
{
    public string HomeTeamName { get; }
    public string VisitingTeamName { get; }
    public string Result { get; }

    private TournamentMatch(string homeTeamName, string visitingTeamName, string result)
    {
        HomeTeamName = homeTeamName;
        VisitingTeamName = visitingTeamName;
        Result = result;
    }

    public static TournamentMatch FromData(string matchData)
    {
        var dataSpan = matchData.AsSpan();
        int teamNamesSeparatorIndex = dataSpan.IndexOf(';');
        string homeTeam = dataSpan[..teamNamesSeparatorIndex].ToString();

        int resultSeparatorIndex = dataSpan.LastIndexOf(';');
        int visitingTeamStartIndex = teamNamesSeparatorIndex + 1;
        int visitingTeamNameLength = resultSeparatorIndex - (visitingTeamStartIndex);
        string visitingTeam = dataSpan.Slice(visitingTeamStartIndex, visitingTeamNameLength).ToString();

        int resultStartIndex = resultSeparatorIndex + 1;
        string result = dataSpan[resultStartIndex..].ToString();

        return new(homeTeam, visitingTeam, result);
    }
}

internal class TournamentStatistics
{

    private readonly Dictionary<string, Team> _teams = new();

    public void Win(string homeTeamName, string visitingTeamName)
    {
        var homeTeam = GetTeam(homeTeamName);
        homeTeam.Win();

        var visitingTeam = GetTeam(visitingTeamName);
        visitingTeam.Loss();
    }

    public void Loss(string homeTeamName, string visitingTeamName)
    {
        var homeTeam = GetTeam(homeTeamName);
        homeTeam.Loss();

        var visitingTeam = GetTeam(visitingTeamName);
        visitingTeam.Win();
    }

    public void Draw(string homeTeamName, string visitingTeamName)
    {
        var homeTeam = GetTeam(homeTeamName);
        homeTeam.Draw();

        var visitingTeam = GetTeam(visitingTeamName);
        visitingTeam.Draw();
    }

    private Team GetTeam(string teamName)
    {
        if (!_teams.TryGetValue(teamName, out Team team))
        {
            team = new Team(teamName);
            _teams.Add(teamName, team);
        }
        return team;
    }

    public IReadOnlyList<TeamStatistics> GetStatistics() =>
        _teams
            .Values
            .Select(t => t.GetStatistics())
            .OrderByDescending(ts => ts.Points)
            .ThenBy(ts => ts.Name)
            .ToList()
            .AsReadOnly();
}

internal class Team
{
    private const int WinPoints = 3;
    private const int DrawPoints = 1;

    private int _plays;
    private int _wins;
    private int _losses;
    private int _draws;
    private int _points;

    public string Name { get; }

    public Team(string name)
    {
        Name = name;
    }

    public void Win()
    {
        _plays++;
        _wins++;
        _points += WinPoints;
    }

    public void Loss()
    {
        _plays++;
        _losses++;
    }

    public void Draw()
    {
        _plays++;
        _draws++;
        _points += DrawPoints;
    }

    public TeamStatistics GetStatistics()
    {
        return new TeamStatistics(Name, _plays, _wins, _losses, _draws, _points);
    }
}

internal record TeamStatistics(string Name, int Plays, int Wins, int Losses, int Draws, int Points);