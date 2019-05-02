use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
    phone: u64,
}

fn main () {
    let person1 = Person {
    id: 5,
    name: "Janet".to_string(),
    phone: 555_666_7777,
    };
    let person2 = Person {
        id: 5,
        name: "Bob".to_string(),
        phone: 555_666_7777,
    };
    let str = String::from("aa");

    println!("{:#?} -- {:#?} -- {}", calculate_hash(&person1), calculate_hash(&person2), calculate_string_hash(&str));
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    return s.finish();
}
fn calculate_string_hash(t: &String) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    return s.finish();
}
