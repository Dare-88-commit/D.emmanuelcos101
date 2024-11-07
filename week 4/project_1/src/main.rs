use std::io;
use std::f64;

fn main() {
    // Input values for a, b, and c
    println!("Enter the coefficient a:");
    let a = read_input();

    println!("Enter the coefficient b:");
    let b = read_input();

    println!("Enter the coefficient c:");
    let c = read_input();

    // Calculate the discriminant
    let discriminant = b * b - 4.0 * a * c;

    // Check the nature of the roots based on the discriminant
    if discriminant > 0.0 {
        // Two real and distinct roots
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The roots are real and distinct:");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    } else if discriminant == 0.0 {
        // One real and repeated root
        let root = -b / (2.0 * a);
        println!("The root is real and repeated:");
        println!("Root = {}", root);
    } else {
        // Complex roots
        let real_part = -b / (2.0 * a);
        let imaginary_part = (discriminant.abs().sqrt()) / (2.0 * a);
        println!("The roots are complex and imaginary:");
        println!("Root 1 = {} + {}i", real_part, imaginary_part);
        println!("Root 2 = {} - {}i", real_part, imaginary_part);
    }
}

// Helper function to read input and parse it as a f64 number
fn read_input() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}
