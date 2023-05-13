use std::collections::HashMap;

pub fn run() {
    if !cfg!(feature = "for_loop") {
        return;
    }

    for i in 0..5 {
        println!("{}", i);
    }

    let mut my_hash_map = HashMap::new();
    my_hash_map.insert("one", 1);
    my_hash_map.insert("two", 2);
    my_hash_map.insert("three", 3);

    for (key, value) in my_hash_map.iter() {
        println!("Key: {}, Value: {}", key, value);
    }
}
