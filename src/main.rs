use std::{collections::HashMap, io};
mod store;
mod handler;

fn main() {
    let mut hash: HashMap<String, String> = HashMap::new();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let input = input.trim();
        let vec: Vec<&str> = input.split_whitespace().collect();
        if vec.is_empty() {
            continue;
        }
        if vec[0].to_lowercase() == "set" {
            assert_eq!(vec.len(), 3);
            let key = vec[1];
            let value = vec[2];
            hash.insert(key.to_string(), value.to_string());
        } else if vec[0].to_lowercase() == "get" {
            assert_eq!(vec.len(), 2);
            let key = vec[1];
            if let Some(val) = hash.get(key) {
                println!("{:?}", val);
            } else {
                println!("(nil)")
            }
        } else {
            println!("invalid command");
        }
    }
}
