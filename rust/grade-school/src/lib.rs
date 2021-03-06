use std::collections::HashMap;

pub struct School<'a> {
    roster: HashMap<u32, Vec<&'a str>>,
}

impl<'a> School<'a> {
    pub fn new() -> Self {
        School {
            roster: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        match self.roster.get_mut(&grade) {
            Some(students) => {
                students.push(student);
            }
            None => {
                self.roster.insert(grade, vec![student]);
            }
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self.roster.keys().copied().collect::<Vec<_>>();
        grades.sort_unstable();
        grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.roster.get_key_value(&grade) {
            Some((_, students)) => {
                let mut student_list = students
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();

                student_list.sort();
                student_list
            }
            None => Vec::new(),
        }
    }
}

impl Default for School<'_> {
    fn default() -> Self {
        School::new()
    }
}
