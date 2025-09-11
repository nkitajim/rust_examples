use bincode::{config, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
struct Person {
	name: String,
	scores: Vec<i32>,
}

fn main() {
	let p = Person {
		name: "Alice".to_string(),
		scores: vec![100, 90, 80],
	};
	let config = config::standard();

	// struct -> バイト列
	let bytes = bincode::encode_to_vec(&p, config).unwrap();
	println!("serialized bytes: {:?}", bytes);

	// バイト列 -> struct
	let (p2, _): (Person, usize) = bincode::decode_from_slice(&bytes[..], config).unwrap();
	println!("deserialized: {:?}", p2);
}
