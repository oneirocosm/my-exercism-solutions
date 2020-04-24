use std::collections::{BTreeMap, BTreeSet};

pub struct School {
    students_in_grade: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            students_in_grade: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students_in_grade
            .entry(grade)
            .or_insert_with(BTreeSet::new)
            .insert(student.into());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students_in_grade.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.students_in_grade.get(&grade) {
            Some(students) => Some(students.iter().cloned().collect()),
            None => None,
        }
    }
}
