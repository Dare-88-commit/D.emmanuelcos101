fn main() {
    let a = 20;
    let b = 30;

    // Logical AND (&&)
    if (a > 10) && (b > 10) {
        println!("True");
    }

    let c = 0;
    let d = 30;

    // Logical OR (||)
    if (c > 10) || (d > 10) {
        println!("True");
    }

    // Logical NOT (!)
    let is_elder = false;
    if !is_elder {
        println!("Not Elder");
    }
}
