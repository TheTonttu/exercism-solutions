using System;

public static class PlayAnalyzer
{
    public static string AnalyzeOnField(int shirtNum) => shirtNum switch
    {
        1 => "goalie",
        2 => "left back",
        3 or 4 => "center back",
        5 => "right back",
        >= 6 and <= 8 => "midfielder",
        9 => "left wing",
        10 => "striker",
        11 => "right wing",
        _ => throw new ArgumentOutOfRangeException(nameof(shirtNum)),
    };

    public static string AnalyzeOffField(object report) => report switch
    {
        int supporterCount => $"There are {supporterCount} supporters at the match.",
        string reportMessage => reportMessage,
        Manager manager => GetManagerDescription(manager),
        Injury injury => $"Oh no! {injury.GetDescription()} Medics are on the field.",
        Incident incident => incident.GetDescription(),
        null => throw new ArgumentNullException(nameof(report)),
        _ => throw new ArgumentException("Unsupported type", nameof(report)),
    };

    private static string GetManagerDescription(Manager manager)
    {
        return String.IsNullOrWhiteSpace(manager.Club)
            ? manager.Name
            : $"{manager.Name} ({manager.Club})";
    }
}
