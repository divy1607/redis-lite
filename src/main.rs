use std::{collections::HashMap, io};

fn main() {
    let mut hash: HashMap<String, String> = HashMap::new();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let vec: Vec<&str> = input.split_whitespace().collect();
        if vec[0] == "SET" {
            assert_eq!(vec.len(), 3);
            hash.insert(vec[1].to_string(), vec[2].to_string());
        } else if vec[0] == "GET" {
            assert_eq!(vec.len(), 2);
            if hash.contains_key(&vec[1].to_string()) {
                println!("{}", hash[&vec[1].to_string()]);
            } else {
                panic!("no such key exists");
            }
        } else {
            panic!()
        }
    }
}
