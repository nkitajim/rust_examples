fn main() {
	println!("{}", add(1, 2));
	println!("{}", add_ex(1, 2));
	println!("{}", sub(2, 1));
	println!("{}", sub_ex(2, 1));
	println!("{}", div(5, 2));
	println!("{:?}", div_ex(5, 2));
	let a: i32 = 10;
	let b: i32 = 20;
	println!("{}", bigger_ex(&a, &b));
	println!("{}", bigger(a, b));
}

fn add(i: i32, j: i32) -> i32 {
	i + j
}

fn add_ex<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
	i + j
}

fn sub(i: i32, j: i32) -> i32 {
	i - j
}

fn sub_ex<T: std::ops::Sub<Output = T>>(i: T, j: T) -> T {
	i - j
}

fn div(i: i32, j: i32) -> i32 {
	i / j
}

fn div_ex<T>(i: T, j: T) -> (T, T)
where 
	T: std::ops::Div<Output = T> + std::ops::Rem<Output = T> + Copy
{
	(i / j, i % j)
}

fn bigger (i: i32, j: i32) -> i32 {
	if i > j {
		i
	} else {
		j
	}
}

// 参照で、片一方しか使わない場合はライフタイム注釈が必要
fn bigger_ex <'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
	if *i > *j {
		i
	} else {
		j
	}
}
