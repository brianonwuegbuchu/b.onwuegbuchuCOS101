use std::io;

fn main() {
    // Define a vector to hold candidate data
    let mut candidates = vec![
        ("Alice".to_string(), 5),
        ("Bob".to_string(), 7),
        ("Charlie".to_string(), 10),
        ("Diana".to_string(), 3),
        ("Eve".to_string(), 8),
    ];

    // Loop to add new candidates based on user input
    println!("Enter candidate's name (or type 'done' to finish): ");
    loop {
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");
        let name = name.trim();
        if name.eq_ignore_ascii_case("done") {
            break;
        }

        println!("Enter years of programming experience for {}: ", name);
        let mut experience = String::new();
        io::stdin()
            .read_line(&mut experience)
            .expect("Failed to read input");
        let experience: i32 = match experience.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number for experience.");
                continue;
            }
        };

        candidates.push((name.to_string(), experience));
        println!("Enter next candidate's name (or type 'done' to finish): ");
    }

    // Find the candidate with the highest experience
    let mut max_experience = 0;
    let mut top_candidate = String::new();

    for (candidate_name, candidate_experience) in &candidates {
        if *candidate_experience > max_experience {
            max_experience = *candidate_experience;
            top_candidate = candidate_name.clone();
        }
    }

    // Output the result
    if !top_candidate.is_empty() {
        println!(
            "The candidate with the highest programming experience is {} with {} years.",
            top_candidate, max_experience
        );
    } else {
        println!("No candidates were entered.");
    }
}
