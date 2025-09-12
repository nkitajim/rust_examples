use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn main() {
    let b1 = MyBox::new(1);
    let b2 = MyBox::new(String::from("Hello"));
    println!("{:?}", b1);
    println!("{:?}", b2);
    println!("{}", *b1);
    println!("{}", *b2);
}
