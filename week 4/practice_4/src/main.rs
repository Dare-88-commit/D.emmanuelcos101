// Rust program to determine age pass
use std::io;

fn main() {
    let mut input1 = String::new();

    println!("Enter your age: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:i32 = input1.trim().parse().expect("Not a valid number");

    if age >= 18{
        println!("Welcome to the party {}!", input1);
    } else {println!("Oops, you are not of age to enter the party {}", input1);
    }
}
