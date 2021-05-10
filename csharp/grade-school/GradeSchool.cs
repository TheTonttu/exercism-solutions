using System;
using System.Collections.Generic;
using System.Linq;

public class GradeSchool
{
    private readonly Dictionary<int, List<string>> _grades = new Dictionary<int, List<string>>();

    public void Add(string student, int grade)
    {
        if (_grades.TryGetValue(grade, out var gradeStudentList))
        {
            gradeStudentList.Add(student);
        }
        else
        {
            _grades.Add(grade, new List<string>() { student });
        }
    }

    public IEnumerable<string> Roster()
    {
        var roster = new List<string>();
        foreach (var grade in _grades.OrderBy(kv => kv.Key).Select(kv => kv.Value))
        {
            roster.AddRange(grade.OrderBy(student => student));
        }
        return roster;
    }

    public IEnumerable<string> Grade(int grade)
    {
        if (!_grades.TryGetValue(grade, out var gradeStudentList))
        {
            return Enumerable.Empty<string>();
        }
        return gradeStudentList.OrderBy(student => student).ToArray();
    }
}