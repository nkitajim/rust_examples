trait Animal {
    fn name(&self) -> &str;
    fn speak(&self);
}

// Dog 型
struct Dog {
    name: String,
}

// Cat 型
struct Cat {
    name: String,
}

// Dog に Animal を実装
impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) {
        println!("{} says: ワン！", self.name);
    }
}

// Cat に Animal を実装
impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) {
        println!("{} says: ニャー！", self.name);
    }
}

// Trait を引数に取る関数
//fn introduce(animal: &impl Animal) {
fn introduce(animal: &dyn Animal) {
    println!("こんにちは！私は {} です。", animal.name());
    animal.speak();
}

fn main() {
    let dog = Dog { name: "ポチ".to_string() };
    let cat = Cat { name: "タマ".to_string() };

    println!("{}", dog.name);
    println!("{}", dog.name());

    introduce(&dog);
    introduce(&cat);
}
