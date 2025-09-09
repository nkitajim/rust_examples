macro_rules! add {
	($a: expr, $b: expr) => {
		$a + $b
	}
}

fn main() {
	println!("1 + 1 = {}", add!(1, 1));
}
