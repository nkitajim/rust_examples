use std::borrow::Cow;
use serde::{Deserialize, Serialize};

fn print_uppercase<'a>(input: Cow<'a, str>) {
	println!("{}", input.to_uppercase());
}

fn normalize<'a>(s: &'a str) -> Cow<'a, str> {
	if s.contains('_') {
		Cow::Owned(s.replace("_", "-")) // 加工が必要 → 所有化（String に変換）
	} else {
		Cow::Borrowed(s) // 加工不要 → 借用のまま返す
	}
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<'a> {
	#[serde(borrow)]
	user_name: Cow<'a, str>,
	#[serde(borrow)]
	values: Cow<'a, [i32]>,
}

fn main() {
	let s1: &str = "hello";
	let s2: String = "world".to_string();
	print_uppercase(Cow::Borrowed(s1));	  // 借用で渡す
	print_uppercase(Cow::Owned(s2)); // 所有で渡す

	println!("{}", normalize("hello_world"));

	let json = r#"
	{
		"user_name": "Alice",
		"values": [1, 2, 3, 4, 5]
	}
	"#;
	let mut data: ApiResponse = serde_json::from_str(json).unwrap();
	// 借用のまま使える（コピーなし）
	println!("user_name: {}", data.user_name);
	println!("values: {:?}", data.values);

	// 値を変更したい場合だけ所有化
	data.values.to_mut().push(6);

	println!("user_name: {}", data.user_name);
	println!("values: {:?}", data.values);

}
