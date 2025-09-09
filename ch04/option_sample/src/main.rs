fn div(a: i32, b: i32) -> Option<i32> {
	if b == 0 {
		return None
	}
	Some(a / b)
}

fn main() {
	let x = match div(4, 2) {
		Some(x) => x,
		None => 0,
	};
	println!("div(4, 2) match: {}", x);
	let x = match div(4, 0) {
		Some(x) => x,
		None => 0,
	};
	println!("div(4, 0) match: {}", x);

	println!("div(4, 2) unwrap(): {}", div(4, 2).unwrap());
	// println!("{}", div(4, 0).unwrap()); // panic
	println!("div(4, 2).unwrap_or(0): {}", div(4, 2).unwrap_or(0));
	println!("div(4, 0).unwrap_or(0): {}", div(4, 0).unwrap_or(0));
	println!("div(4, 2).unwrap_or_else(|| 0): {}", div(4, 2).unwrap_or_else(|| 0));
	println!("div(4, 0).unwrap_or_else(|| 0): {}", div(4, 0).unwrap_or_else(|| 0));
	println!("div(4, 2).map_or(0, |v| v): {}", div(4, 2).map_or(0, |v| v));
	println!("div(4, 0).map_or(0, |v| v): {}", div(4, 0).map_or(0, |v| v));
	println!("div(4, 2).map_or_else(|| 0, |v| v): {}", div(4, 2).map_or_else(|| 0, |v| v));
	println!("div(4, 0).map_or_else(|| 0, |v| v): {}", div(4, 0).map_or_else(|| 0, |v| v));
	println!("div(4, 2).map(|v| v * 2).unwrap_or(0): {}", div(4, 2).map(|v| v * 2).unwrap_or(0));
	println!("div(4, 0).map(|v| v * 2).unwrap_or(0): {}", div(4, 0).map(|v| v * 2).unwrap_or(0));
	println!("div(4, 2).filter(|v| *v > 2).unwrap_or(0): {}", div(4, 2).filter(|v| *v > 2).unwrap_or(0));
	println!("div(4, 0).filter(|v| *v > 2).unwrap_or(0): {}", div(4, 0).filter(|v| *v > 2).unwrap_or(0));
	// and_thenは、Noneなどを返したいときに使う
	println!("div(4, 2).and_then(|v| Some(v * 2)).unwrap_or(0): {}", div(4, 2).and_then(|v| Some(v * 2)).unwrap_or(0));
	println!("div(4, 0).and_then(|v| Some(v * 2)).unwrap_or(0): {}", div(4, 0).and_then(|v| Some(v * 2)).unwrap_or(0));
}
