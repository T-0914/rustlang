use itertools::Itertools;
use std::collections::BTreeMap;

pub struct SchoolBTreeMap<T> {
    school: BTreeMap<T, Vec<String>>,
}

impl<T> SchoolBTreeMap<T>
where
    T: Ord + Copy,
{
    pub fn new() -> SchoolBTreeMap<T> {
        // unimplemented!()
        SchoolBTreeMap {
            school: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: T, student: &str) {
        // unimplemented!()
        if self.school.contains_key(&grade) {
            SchoolBTreeMap::add_student(self, grade, student);
        } else {
            SchoolBTreeMap::add_grade(self, grade);
            SchoolBTreeMap::add_student(self, grade, student);
        }
    }

    fn add_grade(&mut self, grade: T) {
        self.school.insert(grade, Vec::new());
    }

    fn add_student(&mut self, grade: T, student: &str) {
        let students = self.school.get_mut(&grade);

        match students {
            Some(_students) => {
                _students.push(String::from(student));
            }
            None => panic!("Grade is not in range!"),
        }
    }

    pub fn grades(&self) -> Vec<T> {
        // unimplemented!()
        let _grades: BTreeMap<T, Vec<String>> = self.school.clone();
        _grades.into_keys().sorted().collect()
    }

    pub fn grade(&self, grade: T) -> Vec<String> {
        // unimplemented!()
        let students = self.school.get(&grade);

        match students {
            Some(_student) => _student.clone(),
            None => panic!("Value not in range"),
        }
    }
}
