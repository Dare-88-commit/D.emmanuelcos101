struct Laptop {
    brand: String,
    cost_per_unit: u32,
    quantity: u32,
}

fn calculate_cost(laptop: &Laptop) -> u32 {
    laptop.cost_per_unit * laptop.quantity
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        cost_per_unit: 650_000,
        quantity: 3,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        cost_per_unit: 850_000,
        quantity: 3,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        cost_per_unit: 550_000,
        quantity: 3,
    };

    let macbook = Laptop {
        brand: String::from("Macbook"),
        cost_per_unit: 755_000,
        quantity: 3,
    };

    // Calculate the total cost
    let total_cost = calculate_cost(&hp)
        + calculate_cost(&dell)
        + calculate_cost(&toshiba)
        + calculate_cost(&macbook);

    println!("The total cost of purchasing 3 laptops from each brand is: ₦{}", total_cost);
}
