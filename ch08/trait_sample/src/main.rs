trait Debuggable {
	fn debug(&self);
}

trait Greet: Debuggable {
	// method自体も宣言できる
	fn greet(&self) {
		println!("Hello");
	}
}

struct User {
	name: String,
}

impl Debuggable for User {
	fn debug(&self) {
		println!("User: {}", self.name);
	}
}

impl Greet for User {} // Greet Traitを使えるようにする

struct Empty {}
impl Debuggable for Empty {
	fn debug(&self) {
		println!("Hello World!");
	}
}

impl Greet for Empty {
	fn greet(&self) { // greetの上書き
		println!("Hello World!");
	}
}

fn main() {
	let u = User { name: "Alice".to_string() };
	u.greet(); // Greet 内で Debuggable の機能が使える
	u.debug();

	let e = Empty {};
	e.greet();
	e.debug();
}
