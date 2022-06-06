pub mod school_btreemap;
pub mod school_hashmap;

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School<'a> {
    grades: Vec<u32>,
    students: Vec<Student<'a>>,
}

#[derive(Debug)]
struct Student<'a> {
    grade: u32,
    student: &'a str,
}

impl<'a> School<'a> {
    pub fn new() -> School<'a> {
        School {
            grades: Vec::new(),
            students: Vec::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        self.grades.push(grade);
        self.students.push(Student { grade, student })
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut _grades = self.grades.clone();
        _grades.sort();
        _grades.dedup();
        _grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let _self = self.clone();
        let _students = _self
            .students
            .iter()
            .filter(|_student| _student.grade == grade);

        for _student in _students {
            result.push(String::from(_student.student));
        }
        result.sort();
        result
    }
}
