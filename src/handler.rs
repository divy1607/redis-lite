use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::store::Store;

use std::io::Write;

pub fn handle_connection(mut stream: TcpStream, store: Arc<Mutex<Store>>) {
    let buf_reader = BufReader::new(stream.try_clone().unwrap());

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let li = line.trim();
        let parts: Vec<&str> = li.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        if parts[0].to_lowercase() == "set" {
            if parts.len() != 3 {
                writeln!(stream, "error in number of arguments").unwrap();
            }
            let mut shared = store.lock().unwrap();
            let key = parts[1];
            let value = parts[2];
            shared.set(key.to_string(), value.to_string());
            drop(shared);
        } else if parts[0].to_lowercase() == "get" {
            if parts.len() != 2 {
                writeln!(stream, "error in number of arguments").unwrap();
            }
            let shared = store.lock().unwrap();
            let key = parts[1];

            if let Some(val) = shared.get(key) {
                writeln!(stream, "{}", val).unwrap();
            } else {
                writeln!(stream, "(nil)").unwrap();
            }
            drop(shared);

        } else {
            writeln!(stream, "invalid command").unwrap();
        }
    }
}