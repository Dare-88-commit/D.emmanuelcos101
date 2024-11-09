use std::io;

fn main() {
    let mut age = String::new();
    println!("Input age");
    io::stdin().read_line(&mut age).expect("not a string");
    let a: i32 = age.trim().parse().expect("not an integer");

    let mut experience = String::new();
    println!("Is the employee experienced? (YES or NO)");
    io::stdin().read_line(&mut experience).expect("");
    let experience = experience.trim().to_uppercase();

    let incentive = if experience == "YES" {
        if a >= 40 {
            1_560_600
        }else if a >=30 {
            1_480_000
        }else if a <28 {
            1_300_000
        } else {
        0    
        }
    } else {
        100_000
    };
    println!("Annual incentive of employee is, N{}", incentive);
}