using System;
using System.Collections.Generic;
using System.Linq;

public class GradeSchool
{
    private readonly Dictionary<int, SortedSet<string>> _grades = new Dictionary<int, SortedSet<string>>();

    public void Add(string student, int grade)
    {
        if (_grades.TryGetValue(grade, out var gradeStudentList))
        {
            gradeStudentList.Add(student);
        }
        else
        {
            _grades.Add(grade, new SortedSet<string>() { student });
        }
    }

    public IEnumerable<string> Roster()
    {
        return _grades
            .OrderBy(kv => kv.Key)
            .SelectMany(kv => kv.Value)
            .ToArray();
    }

    public IEnumerable<string> Grade(int grade)
    {
        if (!_grades.TryGetValue(grade, out var gradeStudentList))
        {
            return Enumerable.Empty<string>();
        }
        return gradeStudentList.ToArray();
    }
}