// use std::cmp::Eq;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SchoolHashMap<'a> {
    school: HashMap<u32, Vec<&'a str>>,
}

impl<'a> SchoolHashMap<'a> {
    pub fn new() -> SchoolHashMap<'a> {
        // unimplemented!()
        SchoolHashMap {
            school: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        if self.school.contains_key(&grade) {
            SchoolHashMap::add_student(self, grade, student);
        } else {
            SchoolHashMap::add_grade(self, grade);
            SchoolHashMap::add_student(self, grade, student);
        }
    }

    fn add_grade(&mut self, grade: u32) {
        self.school.insert(grade, Vec::new());
    }

    fn add_student(&mut self, grade: u32, student: &'a str) {
        let students = self.school.get_mut(&grade);

        match students {
            Some(_students) => {
                _students.push(student);
            }
            None => panic!("Grade is not in range!"),
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let _grades: HashMap<u32, Vec<&str>> = self.school.clone();
        _grades.into_keys().sorted().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let students = self.school.get(&grade);

        match students {
            Some(_student) => _student
                .iter()
                .map(|_student| _student.to_string())
                .sorted()
                .collect(),
            None => panic!("Value not in range"),
        }
    }
}
