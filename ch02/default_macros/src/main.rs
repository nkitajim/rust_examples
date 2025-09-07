/*
print!, println!, eprint!, eprintln!
format!, format_args!
dbg!
assert!, assert_eq!, assert_ne!
panic!, unreachable!, unimplemented!, todo!
vec!
*/

fn main() {
	println!("println!");
	eprintln!("eprintln!");

	println!("{}", format_args!("Hello {}!", "World"));

	dbg!( 1 + 1);

	assert!( 1 == 1 );
	assert_eq!(1, 1);

	let mut v = vec![1];
	v.push(2);
	println!("{:?}", v)
}

/*
fn todo() {
	todo!();
}

fn unimplemented() {
	unimplemented!();
}
*/
