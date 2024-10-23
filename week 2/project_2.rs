fn main() {
	//toshiba
	let qty:f64 = 2.00;
	let amt:f64 = 450_000.00;
	let t = qty * amt;
	println!("Total amount is {}", t);

	//mac
	let qty:f64 = 1.00;
	let amt:f64 = 1_500_000.00;
	let t = qty * amt;
	println!("Total amount is {}", t);

	//hp
	let qty:f64 = 3.00;
	let amt:f64 = 750_000.00;
	let b = qty * amt;
	println!("Total amount is {}", b);

	//dell
	let qty:f64 = 3.00;
	let amt:f64 = 2_850_000.00;
	let c = qty * amt;
	println!("Total amount is {}", c);

	//acer
	let qty:f64 = 1.00;
	let amt:f64 = 250_000.00;
	let d = qty * amt;
	println!("Total amount is {}", d);

	let s = t + b + c + d;
	println!("Sum is {}", s);
	let a = (t + b + c + d)/4.00;
	println!("Average is {}", a);
	
}