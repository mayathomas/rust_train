use std::collections::BTreeSet;

fn main() {
    let mut a = BTreeSet::new();
    a.insert(1);
    a.insert(2);
    a.insert(3);
    a.insert(4);
    a.insert(5);
    println!("{:?}", a);
}
