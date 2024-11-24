use std::io;

fn main() {
    // Menu items as an array
    let menu = [
        ("P", "Poundo Yam/Edinkaiko Soup", 3200),
        ("F", "Fried Rice & Chicken", 3000),
        ("A", "Amala & Ewedu Soup", 2500),
        ("E", "Eba & Egusi Soup", 2000),
        ("W", "White Rice & Stew", 2500),
    ];

    println!("Welcome to our restaurant! Here is the menu:");
    for (code, name, price) in &menu {
        println!("{}: {} - ₦{}", code, name, price);
    }

    let mut total = 0;

    loop {
        // Get the food code from the user
        println!("\nEnter the code of the food you want to order (or type 'exit' to finish):");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let food_code = input1.trim().to_uppercase();

        if food_code == "EXIT" {
            break;
        }

        // Find the selected menu item
        let selected_item = menu.iter().find(|&&(code, _, _)| code == food_code);

        match selected_item {
            Some((_, name, price)) => {
                println!("You selected: {}", name);

                // Get the quantity
                println!("Enter the quantity:");
                let mut input2 = String::new();
                io::stdin().read_line(&mut input2).expect("Failed to read input");
                let quantity: i32 = match input2.trim().parse() {
                    Ok(q) => q,
                    Err(_) => {
                        println!("Invalid quantity! Please try again.");
                        continue;
                    }
                };

                // Calculate the total for this item
                let item_total = price * quantity;
                total += item_total;

                println!(
                    "Added {} x ₦{} = ₦{} to your order.",
                    quantity, price, item_total
                );
            }
            None => {
                println!("Invalid food code. Please try again.");
            }
        }
    }

    // Apply discount if applicable
    if total > 10000 {
        let discount = total as f64 * 0.05;
        total = (total as f64 - discount) as i32;
        println!("\nYour order qualifies for a 5% discount! Discount applied: ₦{:.2}", discount);
    }

    // Display the total charges
    println!("\nTotal charges for your order: ₦{}", total);
    println!("Thank you for ordering!");
}
