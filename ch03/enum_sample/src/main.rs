#![allow(dead_code)]

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// データを持たせることもできる
enum Cal {
    Add(i32, i32),
    Sub(i32, i32),
}

fn main() {
	let dir = Direction::Up;

	// enumは全て宣言しないとエラーになってくれる
	match dir {
		Direction::Up => println!("Up!"),
		Direction::Down => println!("Down!"),
		Direction::Left => println!("Left!"),
		_ => println!("Right!"),
	}

	let cal = Cal::Add(1, 2);
	match cal {
		Cal::Add(x, y) => println!("{} + {} = {}", x, y, x + y),
		Cal::Sub(x, y) => println!("{} - {} = {}", x, y, x - y),
	}
}
