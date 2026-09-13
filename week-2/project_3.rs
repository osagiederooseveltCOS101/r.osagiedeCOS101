fn main(){
	let price: f64 = 210000.0;
	let rate: f64 = 5.0;
	let years: i32 = 3;

	let value = price * (1.0 - rate / 100.0).powi(years);
	println!("Value after {} years = ${:.2}", years, value);

}