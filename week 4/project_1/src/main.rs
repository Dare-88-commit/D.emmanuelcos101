use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("enter coefficient a");
    io::stdin().read_line(&mut input1).expect("not a string");
    let a: f32 = input1.trim().parse().expect("not a number");

    println!("enter coefficient b");
    io::stdin().read_line(&mut input2).expect("not a string");
    let b: f32 = input2.trim().parse().expect("not a number");

    println!("enter coefficient c");
    io::stdin().read_line(&mut input3).expect("not a string");
    let c: f32 = input3.trim().parse().expect("not a number");

    let d: f32 = b * b - 4.0 * a * c;

    if d >0.0{
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    } else if d ==0.0{
        let root = -b / (2.0 * a);
        println!("Root = {}", root);
    } else if d <0.0 {
        println!("There are no real roots")
    }
}