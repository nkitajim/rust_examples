fn add_vec(a: &mut Vec<String>, b: &[String]) {
	// short code
	// a.extend(b.iter().cloned());
	for d in b.iter().cloned() {
		a.push(d);
	}
}

fn join_vec(a: &[String], b: &[String]) -> Vec<String> {
	let mut v = vec![];
	// iter() return &String
	for d in a.iter() {
		v.push(d.clone());
	}
	// clonedで&StringからStringに先に変換もできる
	for d in b.iter().cloned() {
		v.push(d);
	}
	v
}

fn main() {
	let mut d1 = vec!["Hello".to_string()];
	let d2 = vec!["World".to_string(), "!".to_string()];

	add_vec(&mut d1, & d2);
	println!("{d1:?}");

	// add_vec(&mut d1, & d1); // 同時に同じ変数の借用ができない
	// println!("{d1:?}");

	let d4 = join_vec(&d1, &d2);
	println!("{d4:?}");

	let d3 = join_vec(&d1, &d1); // 不変参照は同じものでも参照できる
	println!("{d3:?}");
}
