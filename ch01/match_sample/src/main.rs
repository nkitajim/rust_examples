fn main() {
	let data = [0, 1, 2, 3, 4, 5];
	for i in data {
		let check = match i {
			0 => "match",
			1 ..= 2 => "miss",
			3 | 4 => "match",
			_ => "miss",
		};
		println!("{} {}", i, check);
	}
}
