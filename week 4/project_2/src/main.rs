use std::io;

fn main() {
    // Get employee experience (true for experienced, false for inexperienced)
    println!("Is the employee experienced? (yes or no):");
    let experience = read_input().trim().to_lowercase();
    let is_experienced = experience == "yes";

    // Get employee age
    println!("Enter the age of the employee:");
    let age: u32 = read_input().trim().parse().expect("Please enter a valid number");

    // Determine the annual incentive based on experience and age
    let incentive = if is_experienced {
        if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else if age >= 28 {
            1_300_000
        } else {
            1_300_000
        }
    } else {
        100_000
    };

    // Output the incentive
    println!("The annual incentive for the employee is: N{}", incentive);
}

// Helper function to read input from the user
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}
