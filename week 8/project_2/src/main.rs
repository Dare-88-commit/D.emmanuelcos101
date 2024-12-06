use std::io;

fn main() {
    println!("Enter the number of developers you want to evaluate: ");
    let mut num_developers_input = String::new();
    io::stdin().read_line(&mut num_developers_input).expect("Failed to read input");
    let num_developers: u32 = num_developers_input.trim().parse().expect("Please enter a valid number");

    let mut developers = Vec::new();

    for i in 1..=num_developers {
        println!("Enter details for Developer {}:", i);

        println!("Enter name of Developer {}: ", i);
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name = name.trim();

        println!("Enter years of experience for Developer {}: ", i);
        let mut experience_input = String::new();
        io::stdin().read_line(&mut experience_input).expect("Failed to read input");
        let experience: u8 = experience_input.trim().parse().expect("Invalid number");

        developers.push((name.to_string(), experience));
    }

    let mut highest_experience = 0;
    let mut selected_developer = String::new();

    for developer in developers {
        if developer.1 > highest_experience {
            highest_experience = developer.1;
            selected_developer = developer.0;
        }
    }

    println!(
        "The developer with the highest experience is {} with {} years of experience.",
        selected_developer, highest_experience
    );
}
