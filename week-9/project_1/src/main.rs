use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value for A: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a: f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value for B: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b: f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value for C: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c: f64 = input3.trim().parse().expect("Not a valid number");

    
    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The equation has two distinct roots: {:.2} and {:.2}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("The equation has one real root: {:.2}", root);
    } else {
        println!("The equation has no real roots.");
    }
}