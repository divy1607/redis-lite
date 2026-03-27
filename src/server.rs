use crate::config::FILE_PATH;
use crate::rewrite;
use crate::{handler::handle_connection, store::Store};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc::Sender;
use std::sync::{RwLock, mpsc};
use std::{fs, net::TcpListener, sync::Arc, thread};

pub fn start() -> Option<Sender<String>> {
    let (tx, rx) = mpsc::channel();

    let mut file = match OpenOptions::new().append(true).create(true).open(FILE_PATH) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Failed to open file {}: {}", FILE_PATH, e);
            return None;
        }
    };

    thread::spawn(move || {
        for li in rx {
            if let Err(e) = writeln!(file, "{}", li) {
                eprintln!("Failed to write to file: {}", e);
                break;
            }
        }
    });

    Some(tx)
}

pub fn start_server(store: Arc<RwLock<Store>>) {
    let tx = start();
    let contents = match fs::read_to_string(FILE_PATH) {
        Ok(value) => value,
        Err(_) => {
            return;
        }
    };
    let mut shared = match store.write() {
        Ok(value) => value,
        Err(_) => {
            return;
        }
    };
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
    rewrite::rewrite(store.clone());
    let listener = match TcpListener::bind("127.0.0.1:8000") {
        Ok(value) => value,
        Err(_) => {
            return;
        }
    };

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(value) => value,
            Err(_) => {
                break;
            }
        };
        let value = store.clone();
        let tx_clone = match tx {
            Some(ref value) => value.clone(),
            None => return,
        };
        thread::spawn(move || {
            handle_connection(stream, value, tx_clone);
        });
    }
}
