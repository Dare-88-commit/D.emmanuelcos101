fn main() {
	let p:f64 = 510_000.00;
	let d:f64 = 5.00;
	let t:u32 = 3;

	//compound interest
	let a = p * (1.00 - (d/100.00)).powi(t as i32);
	println!("Amount is {}", a);
	let ci = a - p;
	println!("Compound interest is {}", ci);
}