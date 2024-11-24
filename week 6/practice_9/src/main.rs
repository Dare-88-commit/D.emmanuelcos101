fn main() {
    let a: i32 = 10;
    let b: i32 = 20;

    println!("Value of A: {}", a);
    println!("Value of B: {}", b);

    // Greater than
    let mut res = a > b;
    println!("A greater than B: {}", res);

    // Less than
    res = a < b;
    println!("A less than B: {}", res);

    // Greater than or equal to
    res = a >= b;
    println!("A greater than or equal to B: {}", res);

    // Less than or equal to
    res = a <= b;
    println!("A less than or equal to B: {}", res);

    // Equal to
    res = a == b;
    println!("A is equal to B: {}", res);

    // Not equal to
    res = a != b;
    println!("A is not equal to B: {}", res);
}
