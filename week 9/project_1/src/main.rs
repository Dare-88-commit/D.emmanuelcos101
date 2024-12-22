use std::fs::File;
use std::io::Write;

fn main() {
    let drinks = vec![
        ("Lager", "33 Export"),
        ("Lager", "Desperados"),
        ("Lager", "Goldberg"),
        ("Lager", "Gulder"),
        ("Lager", "Heineken"),
        ("Lager", "Star"),
        ("Stout", "Legend"),
        ("Stout", "Turbo King"),
        ("Stout", "Williams"),
        ("Non-Alcoholic", "Maltina"),
        ("Non-Alcoholic", "Amstel Malta"),
        ("Non-Alcoholic", "Malta Gold"),
        ("Non-Alcoholic", "Fayrouz"),
    ];

    let mut file = File::create("drinks.txt").expect("Could not create file");

    writeln!(file, "{:<15} {:<25}", "Category", "Drink").expect("Could not write to file");
    writeln!(file, "{: <15} {: <25}", " ", " ").expect("Could not write to file");

    for (category, drink) in drinks {
        writeln!(file, "{:<15} {:<25}", category, drink).expect("Could not write to file");
    }

    println!("Data has been saved to drinks.txt in tabular format");
}
