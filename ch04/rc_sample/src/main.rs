use std::rc::Rc;
use std::cell::RefCell;

fn rc_sample() {
	/*
	let a = String::from("Hello, String!");
	let b = a; // aの所有権はbに移動
	let c = a; // aはもう使えない
	println!("rc_sample: {a} {b} {c}");
	*/

	// a=Rc("Hello, String"), b=&a, c=&a
	let a = Rc::new(String::from("Hello, Rc!"));	
	let b = Rc::clone(&a);
	let c = Rc::clone(&a);
	println!("rc_sample: {a} {b} {c}");

	// a="Hello, String", b="Hello, String", c="Hello, String"
	let a = String::from("Hello, String!");
	let b = a.clone();
	let c = a.clone();
	println!("rc_sample: {a} {b} {c}");
}

fn ref_cell_sample() {
	let x = RefCell::new(5);
	*x.borrow_mut() += 1;
	println!("ref_cell_sample: {}", x.borrow());
}

fn rc_ref_cell_sample() {
	let a = Rc::new(RefCell::new(5));

	let b = Rc::clone(&a);
	*b.borrow_mut() += 1;

	println!("a = {}, b = {}", a.borrow(), b.borrow()); 
}

fn main() {
	rc_sample();
	ref_cell_sample();
	rc_ref_cell_sample();
}
