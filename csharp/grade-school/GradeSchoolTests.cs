// This file was auto-generated based on version 1.0.1 of the canonical data.

using System;
using System.Collections.Generic;
using System.Linq;
using Xunit;

public class GradeSchoolTests {

    [Fact]
    public void Adding_a_student_adds_them_to_the_sorted_roster() {
        var sut = new GradeSchool();
        sut.Add("Aimee", 2);
        var expected = new[] { "Aimee" };
        Assert.Equal(expected, sut.Roster());
    }

    [Fact]
    public void Adding_more_student_adds_them_to_the_sorted_roster() {
        var sut = new GradeSchool();
        sut.Add("Blair", 2);
        sut.Add("James", 2);
        sut.Add("Paul", 2);
        var expected = new[] { "Blair", "James", "Paul" };
        Assert.Equal(expected, sut.Roster());
    }

    [Fact]
    public void Adding_students_to_different_grades_adds_them_to_the_same_sorted_roster() {
        var sut = new GradeSchool();
        sut.Add("Chelsea", 3);
        sut.Add("Logan", 7);
        var expected = new[] { "Chelsea", "Logan" };
        Assert.Equal(expected, sut.Roster());
    }

    [Fact]
    public void Roster_returns_an_empty_list_if_there_are_no_students_enrolled() {
        var sut = new GradeSchool();
        var expected = Array.Empty<string>();
        Assert.Empty(sut.Roster());
    }

    [Fact]
    public void Student_names_with_grades_are_displayed_in_the_same_sorted_roster() {
        var sut = new GradeSchool();
        sut.Add("Peter", 2);
        sut.Add("Anna", 1);
        sut.Add("Barb", 1);
        sut.Add("Zoe", 2);
        sut.Add("Alex", 2);
        sut.Add("Jim", 3);
        sut.Add("Charlie", 1);
        var expected = new[] { "Anna", "Barb", "Charlie", "Alex", "Peter", "Zoe", "Jim" };
        Assert.Equal(expected, sut.Roster());
    }

    [Fact]
    public void Grade_returns_the_students_in_that_grade_in_alphabetical_order() {
        var sut = new GradeSchool();
        sut.Add("Franklin", 5);
        sut.Add("Bradley", 5);
        sut.Add("Jeff", 1);
        var expected = new[] { "Bradley", "Franklin" };
        Assert.Equal(expected, sut.Grade(5));
    }

    [Fact]
    public void Grade_returns_an_empty_list_if_there_are_no_students_in_that_grade() {
        var sut = new GradeSchool();
        var expected = Array.Empty<string>();
        Assert.Empty(sut.Grade(1));
    }

    // Additional tests

    [Theory]
    // Separate the data generation so the generation duration is not included in the execution duration.
    [MemberData(nameof(GenerateLoadsAData))]
    public void Loads_a_students(IEnumerable<(int Grade, string StudentName)> data, IEnumerable<string> expected) {
        var sut = new GradeSchool();

        foreach ((int Grade, string StudentName) in data) {
            sut.Add(StudentName, Grade);
        }
        Assert.Equal(expected, sut.Roster());
    }

    public static IEnumerable<object[]> GenerateLoadsAData() {
        const int GradeCount = 100;
        const int StudentsPerGrade = 100;

        var students = new List<(int Grade, string StudentName)>();
        for (int grade = GradeCount; grade >= 0; grade--) {
            for (int studentIter = 0; studentIter < StudentsPerGrade; studentIter++) {
                int studentNumber = studentIter + grade * 1;
                students.Add((grade, $"Student {studentNumber}"));
            }
        }
        var expected = students.OrderBy(t => t.Grade)
                           .ThenBy(t => t.StudentName)
                           .Select(t => t.StudentName)
                           .ToArray();

        return new[] { new object[] { students, expected } };
    }
}