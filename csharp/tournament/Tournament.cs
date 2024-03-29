using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

public static class Tournament
{
    public static void Tally(Stream inStream, Stream outStream)
    {
        var statistics = TournamentStatistics.ReadFrom(inStream);
        var outcomeReport = OutcomeReport.BasedOn(statistics);
        outcomeReport.WriteTo(outStream);
    }
}

internal enum MatchResult
{
    Win,
    Loss,
    Draw
}

internal class TournamentMatch
{
    public string HomeTeamName { get; }
    public string VisitingTeamName { get; }
    public MatchResult Result { get; }

    private TournamentMatch(string homeTeamName, string visitingTeamName, MatchResult result)
    {
        HomeTeamName = homeTeamName;
        VisitingTeamName = visitingTeamName;
        Result = result;
    }

    public static TournamentMatch FromRawData(string rawMatchData)
    {
        var dataSpan = rawMatchData.AsSpan();
        int teamNamesSeparatorIndex = dataSpan.IndexOf(';');
        string homeTeam = dataSpan[..teamNamesSeparatorIndex].ToString();

        int resultSeparatorIndex = dataSpan.LastIndexOf(';');
        int visitingTeamStartIndex = teamNamesSeparatorIndex + 1;
        int visitingTeamNameLength = resultSeparatorIndex - (visitingTeamStartIndex);
        string visitingTeam = dataSpan.Slice(visitingTeamStartIndex, visitingTeamNameLength).ToString();

        int resultStartIndex = resultSeparatorIndex + 1;
        var rawResult = dataSpan[resultStartIndex..];
        var parsedResult = Enum.Parse<MatchResult>(rawResult, ignoreCase: true);

        return new(homeTeam, visitingTeam, parsedResult);
    }
}

internal class TournamentStatistics
{
    private readonly Dictionary<string, Team> _teams = new();

    internal TournamentStatistics() { }

    public IReadOnlyList<TeamStatistics> StatisticsPerTeam() =>
        _teams
            .Values
            .Select(t => t.GetStatistics())
            .OrderByDescending(ts => ts.Points)
            .ThenBy(ts => ts.Name)
            .ToList()
            .AsReadOnly();

    internal void Record(TournamentMatch match)
    {
        switch (match.Result)
        {
            case MatchResult.Win:
                Win(match.HomeTeamName, match.VisitingTeamName);
                break;
            case MatchResult.Loss:
                Loss(match.HomeTeamName, match.VisitingTeamName);
                break;
            case MatchResult.Draw:
                Draw(match.HomeTeamName, match.VisitingTeamName);
                break;
            default: throw new NotSupportedException($"Unsupported result: {match.Result}");
        }
    }

    private void Win(string homeTeamName, string visitingTeamName)
    {
        var (home, visiting) = GetMatchTeams(homeTeamName, visitingTeamName);
        home.Win();
        visiting.Loss();
    }

    private void Loss(string homeTeamName, string visitingTeamName)
    {
        var (home, visiting) = GetMatchTeams(homeTeamName, visitingTeamName);
        home.Loss();
        visiting.Win();
    }

    private void Draw(string homeTeamName, string visitingTeamName)
    {
        var (home, visiting) = GetMatchTeams(homeTeamName, visitingTeamName);
        home.Draw();
        visiting.Draw();
    }

    private (Team Home, Team Visiting) GetMatchTeams(string homeTeamName, string visitingTeamName) => (GetTeam(homeTeamName), GetTeam(visitingTeamName));

    private Team GetTeam(string teamName)
    {
        if (!_teams.TryGetValue(teamName, out Team team))
        {
            team = new Team(teamName);
            _teams.Add(teamName, team);
        }
        return team;
    }

    public static TournamentStatistics ReadFrom(Stream inStream)
    {
        using var reader = new StreamReader(inStream, leaveOpen: true);

        var statistics = new TournamentStatistics();
        string rawMatchData;
        while ((rawMatchData = reader.ReadLine()) != null)
        {
            var match = TournamentMatch.FromRawData(rawMatchData);
            statistics.Record(match);
        }

        return statistics;
    }
}

internal record TeamStatistics(string Name, int Plays, int Wins, int Losses, int Draws, int Points);

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

    public TeamStatistics GetStatistics() => new(Name, _plays, _wins, _losses, _draws, _points);
}

internal class OutcomeReport
{
    private static readonly string OutputLineTemplate = "{0,-30} | {1,2} | {2,2} | {3,2} | {4,2} | {5,2}";
    private static readonly string Header = string.Format(OutputLineTemplate, "Team", "MP", "W", "D", "L", "P");

    private readonly TournamentStatistics _statistics;

    private OutcomeReport(TournamentStatistics statistics)
    {
        _statistics = statistics;
    }

    public void WriteTo(Stream output)
    {
        using var writer = new StreamWriter(output, leaveOpen: true);

        writer.Write(Header);
        foreach (var teamStats in _statistics.StatisticsPerTeam())
        {
            writer.Write('\n');
            string formattedLine = string.Format(OutputLineTemplate, teamStats.Name, teamStats.Plays, teamStats.Wins, teamStats.Draws, teamStats.Losses, teamStats.Points);
            writer.Write(formattedLine);
        }
    }

    public static OutcomeReport BasedOn(TournamentStatistics statistics) => new(statistics);
}