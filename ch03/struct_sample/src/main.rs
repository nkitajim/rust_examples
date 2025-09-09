#![allow(dead_code)]

#[derive(Debug)]
struct Person {
	name: String,
	age: i32,
}

impl Person {
	fn print_name(&self) {
		println!("Hello {}", self.name);
	}
}

#[derive(Debug)]
struct Persons(Vec<Person>);

impl Persons {
	fn new() -> Self {
		Self(Vec::new())
	}

	fn add(&mut self, person: Person) {
		self.0.push(person);
	}

	fn len(&self) -> usize {
		self.0.len()
	}
}

fn main() {
	let person = Person{name: String::from("Noboru"), age: 42};
	person.print_name();
	println!("age: {}", person.age);

	/*
	let mut persons = Persons(!vec[
		Person{name: String::from("Noboru"), age: 42},
		Person{name: String::from("Kyoko"), age: 42},
	]);
	*/
	let mut persons = Persons::new();
	persons.add(Person{name: String::from("Noboru"), age: 42});
	persons.add(Person{name: String::from("Kyoko"), age: 42});

	// iterは借用. iter_mutは書き込みも可能
	for x in persons.0.iter() {
		x.print_name();
		println!("name: {}", x.name);
	}

	// into_iterは所有権を奪う
	for x in persons.0.into_iter() {
		x.print_name();
		println!("name: {}", x.name);
	}
}
