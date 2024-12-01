use std::io;

fn main() {
    let calculations = [
        ("1", "Area of Trapezium"),
        ("2", "Area of Rhombus"),
        ("3", "Area of Parallelogram"),
        ("4", "Area of Cube"),
        ("5", "Volume of Cylinder"),
    ];

    println!("Select a calculation:");
    for (code, calculation) in &calculations {
        println!("{}, {}", code, calculation);
    }

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    match choice.trim() {
        "1" => {
            // Area of Trapezium
            let height = read_input("Enter height");
            let base1 = read_input("Enter base1");
            let base2 = read_input("Enter base2");
            let area = (height * (base1 + base2)) / 2.0;
            println!("The area of the trapezium is: {:.2}", area);
        }
        "2" => {
            // Area of Rhombus
            let diagonal1 = read_input("Enter diagonal1");
            let diagonal2 = read_input("Enter diagonal2");
            let area = 0.5 * diagonal1 * diagonal2;
            println!("The area of the rhombus is: {:.2}", area);
        }
        "3" => {
            // Area of Parallelogram
            let base = read_input("Enter base");
            let height = read_input("Enter height");
            let area = base * height;
            println!("The area of the parallelogram is: {:.2}", area);
        }
        "4" => {
            // Area of Cube
            let side = read_input("Enter side length");
            let area = 6.0 * side * side;
            println!("The area of the cube is: {:.2}", area);
        }
        "5" => {
            // Volume of Cylinder
            let radius = read_input("Enter radius");
            let height = read_input("Enter height");
            let volume = std::f64::consts::PI * radius * radius * height;
            println!("The volume of the cylinder is: {:.2}", volume);
        }
        _ => println!("Invalid choice! Please select a valid option."),
    }
}

fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Not a valid number")
}
