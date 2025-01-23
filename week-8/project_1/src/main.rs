use std::io;

fn main() {
    // Define the staff levels using vectors
    let levels = vec![
        ("APS 1-2", vec!["Intern", "Paralegal", "Placement"]),
        ("APS 3-5", vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"]),
        ("APS 5-8", vec!["Senior Administrator", "PhD Candidate", "Associate", "Senior Teacher"]),
        ("EL1 8-10", vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"]),
        ("EL2 10-13", vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"]),
        ("SES", vec!["CEO", "Dean", "Partner", "Principal"]),
    ];

    // Prompt the user for inputs
    let mut role = String::new();
    let mut years_of_experience = String::new();

    println!("Enter the staff's job role (e.g., Associate, Dean, Paralegal): ");
    io::stdin().read_line(&mut role).expect("Failed to read input");
    let role = role.trim();

    println!("Enter the staff's years of experience: ");
    io::stdin().read_line(&mut years_of_experience).expect("Failed to read input");
    let years_of_experience: i32 = years_of_experience
        .trim()
        .parse()
        .expect("Please enter a valid number");

    // Validate and determine the staff level
    let mut staff_level = "Unknown";
    for (level, roles) in &levels {
        if roles.contains(&role) {
            staff_level = level;
            break;
        }
    }

    // Print the result
    if staff_level != "Unknown" {
        println!(
            "The staff with the role '{}' and {} years of experience holds the position: {}",
            role, years_of_experience, staff_level
        );
    } else {
        println!("The entered role '{}' does not match any known staff positions.", role);
    }
}
