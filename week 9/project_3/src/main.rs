fn main() {
    let names = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];
    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];
    let geopolitical_zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    println!("S/N\tName\t\t\t\t\tMinistry\t\t\t\tGeopolitical Zone");
    println!("-------------------------------------------------------------");

    for i in 0..names.len() {
        println!(
            "{}\t{}\t\t{}\t\t{}",
            i + 1,
            names[i],
            ministries[i],
            geopolitical_zones[i]
        );
    }
}