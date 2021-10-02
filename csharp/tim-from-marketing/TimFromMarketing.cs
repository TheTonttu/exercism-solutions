using System.Text;

static class Badge
{
    public static string Print(int? id, string name, string? department)
    {
        var badgePrintBuilder = new StringBuilder();
        if (id != null)
        {
            badgePrintBuilder.Append($"[{id}] - ");
        }
        badgePrintBuilder.Append($"{name} - ");

        string formattedDepartment = department?.ToUpperInvariant() ?? "OWNER";
        badgePrintBuilder.Append($"{formattedDepartment}");

        return badgePrintBuilder.ToString();
    }
}
