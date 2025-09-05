fn main() {
	let ix = 10; // cound not use method
	let iy: i32 = 10;
	let iz = 10_i32;

	println!("ix:{}, iy: {}, iz: {}", ix, iy, iz);
	println!("iy: {} iy.checked_add(10): {}", iy, iy.checked_add(10).unwrap());
	println!("iy: {} iy.pow(): {}", iy, iy.pow(2));
	println!("iy: {} iy.count_ones(): {}", iy, iy.count_ones());
}
