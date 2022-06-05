use grade_school::School;

fn main() {
    let mut school = School::new();

    // school.add(1, "S1");
    // school.add(2, "S2.0");
    // school.add(2, "S2.1");
    // school.add(2, "S2.2");
    school.add(3, "S3.0");
    school.add(3, "S3.1");
    school.add(3, "S3.2");
    school.add(3, "S3.3");
    school.add(3, "S3.4");
    school.add(3, "S3.5");
    school.add(3, "S3.6");
    school.add(3, "S3.7");
    // school.add(4, "S4");
    // school.add(5, "S5");

    // println!("Grade {}: {:#?}", 1, school.grade(1));
    // println!("Grade {}: {:#?}", 2, school.grade(2));
    // println!("Grade {}: {:#?}", 3, school.grade(3));
    // println!("Grade {}: {:#?}", 4, school.grade(4));
    // println!("Grade {}: {:#?}", 5, school.grade(5));
    println!("Grades: {:#?}", school.grades());
}
