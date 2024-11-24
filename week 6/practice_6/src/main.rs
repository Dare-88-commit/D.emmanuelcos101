fn main() {
    let n1 = "Electrical".to_string();
    let n2 = "Electronic".to_string();
    let n3 = "Engineering".to_string();
    let m1 = n1 + " & " + &n2 + " " + &n3; // n2 and n3 references are passed
    // About Electrical/Electronic
    println!(
        "While {} is informed by the aspiration to train electrical/electronic \
        engineering professionals in the areas of design, building, and maintenance of \
        electrical control systems.", 
        m1
    );

    let w1 = "Computer".to_string();
    let w2 = "Science".to_string();
    let m3 = w1 + " & " + &w2; // w2 reference is passed
    println!();
    println!(
        "{} is aimed at developing competent, creative, innovative, entrepreneurial, and \
        ethically-minded persons capable of creating value in the diverse fields of \
        Computer Science.", 
        m3
    );
}
