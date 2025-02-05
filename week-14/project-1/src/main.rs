use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!(
        "What is your position in the company? 
        1 - administrator,
        2 - project manager,
        3 - employee,
        4 - customer,
        5 - vendor"
    );

    let mut inputa = String::new();
    io::stdin().read_line(&mut inputa).expect("Failed to read input");

    let user_position = inputa.trim();

    match user_position {
        "1" => {
            println!("Welcome Administrator!");
            administrator_table();
        }
        "2" => {
            println!("Welcome Project Manager!");
            manager_table();
        }
        "3" => {
            println!("Welcome Employee");
            employee_table();
        }
        "4" => {
            println!("Welcome Customer");
            customer_table();
        }
        "5" => {
            println!("Welcome Vendor");
            vendor_table();
        }
        _ => {
            println!("Invalid input, please try again!");
        }
    }
}

// Read the file as raw bytes instead of UTF-8
fn read_file(file_path: &str) {
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file '{}': {}", file_path, e);
            return;
        }
    };

    let mut contents = Vec::new(); // Use a Vec<u8> instead of String
    if let Err(e) = file.read_to_end(&mut contents) {
        eprintln!("Failed to read file '{}': {}", file_path, e);
        return;
    }

    // Convert to UTF-8 lossily (replaces invalid chars with `�`)
    let contents_str = String::from_utf8_lossy(&contents);
    println!("{}", contents_str);
}

fn administrator_table() {
    read_file("globalcom_dbase.sql");
}

fn manager_table() {
    read_file("project_tb.sql");
}

fn employee_table() {
    read_file("staff_tb.sql");
}

fn customer_table() {
    read_file("customer_tb.sql");
}

fn vendor_table() {
    read_file("dataplan_tb.sql");
}
