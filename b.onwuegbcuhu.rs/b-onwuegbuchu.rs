use std::io;

fn main() {
    println!("Enter the number of siblings:");

    let mut num_siblings = String::new();

    io::stdin()
        .read_line(&mut num_siblings)
        .expect("Failed to read input");

    let num_siblings: u32 = num_siblings.trim().parse().expect("Please type a number");
}
    for sibling_index in 1..=num_siblings {
        println!("Enter details for sibling {}", sibling_index);

        println!("Enter the sibling's first name:");
        let mut first_name = String::new();
        io::stdin()
            .read_line(&mut first_name)
            .expect("Failed to read input");

        println!("Enter the sibling's age:");
        let mut age = String::new();
        io::stdin()
            .read_line(&mut age)
            .expect("Failed to read input");

        let age: u32 = age.trim().parse().expect("Please type a number");

        println!("Enter the sibling's gender:");
        let mut gender = String::new();
        io::stdin()
            .read_line(&mut gender)
            .expect("Failed to read input");

        println!("Enter the sibling's country of residence:");
        let mut country_of_residence = String::new();
        io::stdin()
            .read_line(&mut country_of_residence)
            .expect("Failed to read input");

        if age >= 18 {
            println!("Is the sibling married, single, or in a relationship?");
            let mut relationship_status = String::new();
            io::stdin()
                .read_line(&mut relationship_status)
                .expect("Failed to read input");

            match relationship_status.trim() {
                "married" => {
                    println!("Does the sibling have any children?");
                    let mut has_children = String::new();
                    io::stdin()
                        .read_line(&mut has_children)
                        .expect("Failed to read input");

                    if has_children.trim().eq_ignore_ascii_case("yes") {
                        println!("Enter the number of children:");
                        let mut num_children = String::new();
                        io::stdin()
                            .read_line(&mut num_children)
                            .expect("Failed to read input");

                        let num_children: u32 = num_children.trim().parse().expect("Please type a number");

                        for child_index in 1..=num_children {
                            println!("Enter details for child {}", child_index);

                            println!("Enter the child's name:");
                            let mut child_name = String::new();
                            io::stdin()
                                .read_line(&mut child_name)
                                .expect("Failed to read input");

                            println!("Enter the child's age:");
                            let mut child_age = String::new();
                            io::stdin()
                                .read_line(&mut child_age)
                                .expect("Failed to read input");

                            println!("Enter the child's school or daycare name:");
                            let mut school_or_daycare_name = String::new();
                            io::stdin()
                                .read_line(&mut school_or_daycare_name)
                                .expect("Failed to read input");
                        }

                        println!("Enter the city where the sibling's family currently lives:");
                        let mut city = String::new();
                        io::stdin()
                            .read_line(&mut city)
                            .expect("Failed to read input");

                        println!("Sibling's family lives in {}", city.trim());
                    }
                }
                "single" => {
                    println!("Is the sibling a student or employed?");
                    let mut employment_status = String::new();
                    io::stdin()
                        .read_line(&mut employment_status)
                        .expect("Failed to read input");

                    match employment_status.trim() {
                        "student" => {
                            println!("Enter the university:");
                            let mut university = String::new();
                            io::stdin()
                                .read_line(&mut university)
                                .expect("Failed to read input");

                            println!("Enter the course of study:");
                            let mut course_of_study = String::new();
                            io::stdin()
                                .read_line(&mut course_of_study)
                                .expect("Failed to read input");

                            println!("Enter the year of study:");
                            let mut year_of_study = String::new();
                            io::stdin()
                                .read_line(&mut year_of_study)
                                .expect("Failed to read input");

                            println!("Is the sibling studying in their home country or abroad?");
                            let mut study_location = String::new();
                            io::stdin()
                                .read_line(&mut study_location)
                                .expect("Failed to read input");

                            if study_location.trim().eq_ignore_ascii_case("abroad") {
                                println!("Enter the country:");
                                let mut country = String::new();
                                io::stdin()
                                    .read_line(&mut country)
                                    .expect("Failed to read input");
                            }
                        }
                        "employed" => {
                            println!("Is the job remote, on-site, or hybrid?");
                            let mut job_type = String::new();
                            io::stdin()
                                .read_line(&mut job_type)
                                .expect("Failed to read input");

                            match job_type.trim() {
                                "on-site" => {
                                    println!("Enter the company name:");
                                    let mut company_name = String::new();
                                    io::stdin()
                                        .read_line(&mut company_name)
                                        .expect("Failed to read input");

                                    println!("Enter the job title:");
                                    let mut job_title = String::new();
                                    io::stdin()
                                        .read_line(&mut job_title)
                                        .expect("Failed to read input");

                                    println!("Enter the industry sector:");
                                    let mut industry_sector = String::new();
                                    io::stdin()
                                        .read_line(&mut industry_sector)
                                        .expect("Failed to read input");
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                "in a relationship" => {
                    println!("Enter the relationship duration (in years):");
                    let mut relationship_duration = String::new();
                    io::stdin()
                        .read_line(&mut relationship_duration)
                        .expect("Failed to read input");

                    let relationship_duration: u32 = relationship_duration.trim().parse().expect("Please type a number");

                    println!("Enter the partner's first name:");
                    let mut partner_first_name = String::new();
                    io::stdin()
                        .read_line(&mut partner_first_name)
                        .expect("Failed to read input");

                    println!("Do they live with their partner?");
                    let mut live_with_partner = String::new();
                    io::stdin()
                        .read_line(&mut live_with_partner)
                        .expect("Failed to read input");

                    if live_with_partner.trim().eq_ignore_ascii_case("yes") {
                        println!("Enter the city where they reside:");
                        let mut city = String::new();
                        io::stdin()
                            .read_line(&mut city)
                            .expect("Failed to read input");

                        println!("They live in {}", city.trim());
                    }
                }
                _ => {}
            }
        } else {
            println!("Has the sibling completed the West African Examinations Council (WAEC) exams?");
            let mut waec_completed = String::new();
            io::stdin()
                .read_line(&mut waec_completed)
                .expect("Failed to read input");

            if waec_completed.trim().eq_ignore_ascii_case("yes") {
                println!("Enter the secondary school attended:");
                let mut secondary_school = String::new();
                io::stdin()
                    .read_line(&mut secondary_school)
                    .expect("Failed to read input");

                println!("Enter the final grade:");
                let mut final_grade = String::new();
                io::stdin()
                    .read_line(&mut final_grade)
                    .expect("Failed to read input");

                println!("Enter the year of completion:");
                let mut year_of_completion = String::new();
                io::stdin()
                    .read_line(&mut year_of_completion)
                    .expect("Failed to read input");
            } else {
                println!("Enter the sibling's current class level:");
                let mut

