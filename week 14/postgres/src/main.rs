use std::fs::File;
use std::io::Read;

fn read_sql_file(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("SQL File Contents:\n{}\n", contents);
}

fn main() {
    read_sql_file("globacom_dbase.sql");
}
