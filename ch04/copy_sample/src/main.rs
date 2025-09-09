// deriveだけで基本的な構造のものは機能を追加できる
// Displayなどdefaultでは出さないものは、impl Display for Robot などを追加する
#[derive(Debug,Copy,Clone,PartialEq)]
struct Robot {
	id: i32
}

// VecなどHeapに置くものはCopyできない
#[derive(Debug,Clone,PartialEq)]
struct Robots(Vec<Robot>);

#[derive(Debug,Clone,Copy)]
enum Message {
	Ok,
	#[allow(unused_variables,dead_code)]
	Err,
}

// Robot, Messageの所有権を奪う
fn print_robot(r: Robot, m: Message) {
	match m {
		Message::Ok => println!("OK: {}", r.id),
		Message::Err => println!("Error: {}", r.id),
	};
}

fn print_robots(robots: Robots) {
	for r in robots.0.iter() {
		println!("{}", r.id);
	}
}

fn main() {
	let r = Robot{id: 1};

	print_robot(r, Message::Ok);
	print_robot(r, Message::Ok); // Copyしているので問題なく使える

	let r2 = Robot{id: 1};
	let r3 = Robot{id: 2};
	if r == r2 {
		println!("r == r2");
	} else {
		println!("r != r2");
	}

	if r == r3 {
		println!("r == r3");
	} else {
		println!("r != r3");
	}

	let robots = Robots(vec!(Robot{id:1}, Robot{id:2}));
	print_robots(robots);
	// print_robots(robots); // Copyができないため失敗する
}
