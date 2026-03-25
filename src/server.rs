use crate::config::FILE_PATH;
use crate::{handler::handle_connection, store::Store};
use std::{
    fs,
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};

pub fn start_server(store: Arc<Mutex<Store>>) {
    let contents = fs::read_to_string(FILE_PATH).unwrap();
    let mut shared = store.lock().unwrap();
    for line in contents.lines() {
        let line = line.trim();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 {
            println!("error in number of arguments");
            continue;
        }
        if parts[0].to_lowercase() == "set" {
            let key = parts[1];
            let value = parts[2];
            shared.set(key.to_string(), value.to_string());
        }
    }
    drop(shared);
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let value = store.clone();
        thread::spawn(move || {
            handle_connection(stream, value);
        });
    }
}
