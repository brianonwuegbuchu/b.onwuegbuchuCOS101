use std::fs::File;
use std::io::{self, Write};

fn main() {
    // Define a vector to hold student data
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 200),
        ("Shania Bolade", "CSC10328288", "Computer Science", 100),
        ("Adekunle Gold", "EEE11020202", "Electrical", 400),
        ("Blanca Edemon", "MEE10202001", "Mechanical", 100),
    ];

    // Display student details
    println!("PAU SMIS");
    println!("{:<20} {:<15} {:<20} {:<5}", "Student Name", "Matric. Number", "Department", "Level");
    for student in &students {
        println!(
            "{:<20} {:<15} {:<20} {:<5}",
            student.0, student.1, student.2, student.3
        );
    }

    // Save the details to a file
    let file_path = "pau_smis.csv";
    let mut file = File::create(file_path).expect("Failed to create file");

    writeln!(file, "Student Name,Matric. Number,Department,Level").expect("Failed to write header");
    for student in &students {
        writeln!(
            file,
            "{},{},{},{}",
            student.0, student.1, student.2, student.3
        )
        .expect("Failed to write to file");
    }

    println!("Student details saved to '{}'", file_path);
}
