use std::io;

fn main() {
    let office_admin = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL 1-2", "EL 2-13", "SES"];
    let academic = vec!["-", "APS 3-5", "APS 5-8", "EL 1-2", "EL 2-13", "SES"];
    let lawyer = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL 1-2", "EL 2-13", "SES"];
    let teacher = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL 1-2", "EL 2-13", "SES"];

    println!("Enter the profession (Office Administrator, Academic, Lawyer, Teacher): ");
    let mut profession = String::new();
    io::stdin().read_line(&mut profession).expect("Failed to read input");
    let profession = profession.trim();

    println!("Enter years of experience: ");
    let mut years_input = String::new();
    io::stdin().read_line(&mut years_input).expect("Failed to read input");
    let years_of_experience: u8 = years_input.trim().parse().expect("Not a valid number");

    let aps_level = if profession.eq_ignore_ascii_case("Office Administrator") {
        get_aps_level(&office_admin, years_of_experience)
    } else if profession.eq_ignore_ascii_case("Academic") {
        get_aps_level(&academic, years_of_experience)
    } else if profession.eq_ignore_ascii_case("Lawyer") {
        get_aps_level(&lawyer, years_of_experience)
    } else if profession.eq_ignore_ascii_case("Teacher") {
        get_aps_level(&teacher, years_of_experience)
    } else {
        "Invalid profession"
    };

    println!(
        "The staff member with profession '{}' and {} years of experience holds position '{}'.",
        profession, years_of_experience, aps_level
    );
}

fn get_aps_level<'a>(levels: &'a Vec<&'a str>, years: u8) -> &'a str {
    if years <= 2 {
        levels[0]
    } else if years <= 5 {
        levels[1]
    } else if years <= 8 {
        levels[2]
    } else if years <= 10 {
        levels[3]
    } else if years <= 13 {
        levels[4]
    } else {
        levels[5]
    }
}
