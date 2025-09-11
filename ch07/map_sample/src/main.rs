use std::collections::HashMap;
// use std::collections::BTreeMap;

fn main() {
    let mut users = HashMap::new();
    // let mut users = BTreeMap::new();
    users.insert("Noboru", "Kitajima");
    println!("{:?}", users);
    if users.contains_key("Noboru") {
        println!("{}", users["Noboru"]);
    }
    // なければ追加
    users.entry("Kyoko").or_insert("Kitajima");
    users.entry("Kyoko").or_insert("Kitajima2"); // すでにあるので追加されない
    println!("{:?}", users);

    // keysやvaluesもある
    for k in users.keys() {
        println!("{}", k);
    }
    // 削除
    users.remove("Kyoko");
    println!("{:?}", users);
}
