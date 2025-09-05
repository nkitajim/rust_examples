fn vec_loop() {
	let data1 = vec![10, 11, 12];
	let data2 = vec![20, 21, 22];
	let mut data3 = vec![30, 31, 32];

	for i in data1 {
		println!("{}", i);
	}
	// println!("{}", data1[0]); // compile error. container type use move

	for i in &data2 {
		println!("{}", i);
	}

	for i in &data3 {
		println!("{}", i);
	}
	for i in &mut data3 {
		*i *= 2;
	}
	for i in &data3 {
		println!("{}", i);
	}
}

fn array_loop() {
	let data1 = [10, 11, 12];
	let data2 = [20, 21, 22];
	let mut data3 = [30, 31, 32];

	for i in data1 {
		println!("{}", i);
	}
	println!("{}", data1[0]); // ok, primitive type is use Copy

	for i in &data2 {
		println!("{}", i);
	}

	for i in &data3 {
		println!("{}", i);
	}
	for i in &mut data3 {
		*i *= 2;
	}
	for i in &data3 {
		println!("{}", i);
	}
}

fn loop_counter() -> i32 {
	let mut counter = 0;
	loop {
		counter += 1;
		if counter > 100 {
			break 100; // return 100
		}
	}
}

fn for_label() {
	'outer: for ix in 0..5 {
		for iy in 0..5 {
			println!("{} {}", ix, iy);
			if ix + iy > 6 {
				break 'outer;
			}
		}
	}
}

fn main() {
	println!("- for vec ---------");
	vec_loop();
	println!("- for array ---------");
	array_loop();
	println!("- loop ---------");
	println!("{}", loop_counter());
	println!("- for label ---------");
	for_label();
}
