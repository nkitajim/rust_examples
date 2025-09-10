use bit_count::*;

fn main() {
	let num1: i32 = 255;
	let num2: f32 = 255.0;

	println!("{:32b} {} {}", num1, i32_count_bit(num1), num1.count_ones());
	println!("{:32b} {} {}", num1, inline_i32_count_bit(num1), num1.count_ones());
	println!("{:32b} {} {}", num2.to_bits(), f32_count_bit(num2), num2.to_bits().count_ones());
}
