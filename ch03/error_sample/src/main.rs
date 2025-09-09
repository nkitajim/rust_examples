fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("divide by zero".into())
    } else {
        Ok(a / b)
    }
}

// ?によりエラーが返されているためReturnが追加
fn main() -> Result<(), Box<dyn std::error::Error>>{
	let i = divide(4, 2).unwrap();
	println!("{}", i);

	let i = match divide(4, 0) {
		Ok(i) => i,
		Err(e) => {
			eprintln!("could not divide: {}", e);
			0
		},
	};
	println!("{}", i);

	let i = divide(4, 0).unwrap_or(0);
	println!("{}", i);

	// error + panic
	let i = divide(4, 0).unwrap();
	println!("{}", i);

	// error(string) + panic
	let i = divide(4, 0).expect("zero is not divide");
	println!("{}", i);

	// return error
	let i = divide(4, 0)?;
	println!("{}", i);

	Ok(())
}
