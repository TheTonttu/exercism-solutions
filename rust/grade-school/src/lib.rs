use std::collections::{HashMap, HashSet};

pub struct School {
    roster: HashMap<String, u32>,
}

impl School {
    pub fn new() -> Self {
        School {
            roster: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let student_name = student.to_string();
        self.roster.insert(student_name, grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let unique_grades = self.roster.values().collect::<HashSet<_>>();
        let mut grades = unique_grades
            .iter()
            // kek
            .copied()
            .copied()
            .collect::<Vec<_>>();

        grades.sort_unstable();
        grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students = self
            .roster
            .iter()
            .filter(|(_k, v)| **v == grade)
            .map(|(k, _v)| k.clone())
            .collect::<Vec<_>>();

        students.sort();
        students
    }
}

impl Default for School {
    fn default() -> Self {
        School::new()
    }
}
