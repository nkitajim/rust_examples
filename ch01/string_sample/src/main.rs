fn print(s: &str) {
	println!("{}", s);
}

fn print_string(s: String) {
	println!("{}", s);
}

fn main() {
	let s1: String = String::from("hello world!");
	let s2: &str = "hello world!";
	print(&s1); // &strは借用のため後続でも利用できる
	print(&s2);
	println!("{}", s1);
	println!("{}", s2);

	let s3 = s1.clone();
	print_string(s1);
	// print_string(s1); すでにmoveされているため利用できない

	// contains
	if s2.contains("ll") {
		println!("contains ll: {}", s2);
	}
	if s3.contains("ll") {
		println!("contains ll: {}", s3);
	}

	// starts_with
	if s2.starts_with("he") {
		println!("starts_with he: {}", s2);
	}
	if s3.starts_with("he") {
		println!("starts_with he: {}", s3);
	}

	let s4: &str = "hello";
	let s5: &str = "world";
	let s6: &str = "!";
	println!("{}", s4.to_owned() + s5 + s6); // &strからString
	println!("{}", s4);

	let s7: String = String::from("hello");
	let s8: String = String::from("world");
	let s9: String = String::from("!");
	println!("{}", s7 + &s8 + &s9);
	println!("{}", s7); // addによってs7はmoveされている。 push_str(+=) の場合はCopy
}
