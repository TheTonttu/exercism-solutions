using System.Collections.Generic;
using System.Linq;

public class GradeSchool
{
    // Separate set of students for more straightforward checking of existing students
    // at the cost of increased memory usage.
    private readonly HashSet<string> _students = new();
    private readonly Dictionary<int, SortedSet<string>> _grades = new();

    public bool Add(string student, int grade)
    {
        if (!_students.Add(student))
        {
            return false;
        }

        if (_grades.TryGetValue(grade, out var gradeStudents))
        {
            return gradeStudents.Add(student);
        }

        _grades[grade] = new() { student };
        return true;
    }

    public IEnumerable<string> Roster() =>
        _grades
            .OrderBy(kv => kv.Key)
            .SelectMany(kv => kv.Value);

    public IEnumerable<string> Grade(int grade) =>
        _grades.TryGetValue(grade, out var gradeStudents)
            ? gradeStudents
            : Enumerable.Empty<string>();
}