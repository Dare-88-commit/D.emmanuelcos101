use std::fs::File;
use std::io::Write;

fn main() {
    let students = vec![
        ("Oluchi Mordi".to_string(), "ACC012111".to_string(), "Accounting".to_string(), 300),
        ("Adams Aliyu".to_string(), "COE101010".to_string(), "Economics".to_string(), 400),
        ("Shania Bolade".to_string(), "CSC103288".to_string(), "Computer Science".to_string(), 200),
        ("Adekunle Gold".to_string(), "EEE101020".to_string(), "Electrical".to_string(), 300),
        ("Blanca Edeh".to_string(), "MEE102001".to_string(), "Mechanical".to_string(), 100),
    ];

    println!("{:<20} {:<15} {:<20} {:<5}", "Student Name", "Matric Number", "Department", "Level");
    println!("{:-<20} {:-<15} {:-<20} {:-<5}", "-", "-", "-", "-");
    for student in &students {
        println!(
            "{:<20} {:<15} {:<20} {:<5}",
            student.0, student.1, student.2, student.3
        );
    }

    let mut file = File::create("students.txt").expect("Could not create file");
    writeln!(file, "{:<20} {:<15} {:<20} {:<5}", "Student Name", "Matric Number", "Department", "Level").expect("Could not write to file");
    writeln!(file, "{:-<20} {:-<15} {:-<20} {:-<5}", "-", "-", "-", "-").expect("Could not write to file");
    for student in &students {
        writeln!( file,
            "{:<20} {:<15} {:<20} {:<5}",
            student.0, student.1, student.2, student.3).expect("Could not write to file");
    }

    println!("Student details have been saved to students.txt");
}
