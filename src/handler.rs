use crate::config::FILE_PATH;
use crate::store::Store;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    net::TcpStream,
    sync::{Arc, Mutex},
};

use std::io::Write;

pub fn handle_connection(mut stream: TcpStream, store: Arc<Mutex<Store>>) {
    let cloned_stream = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to clone stream: {}", e);
            return; 
        }
    };

    let buf_reader = BufReader::new(cloned_stream);

    for line in buf_reader.lines() {
        let line = match line {
            Ok(value) => value,
            Err(_) => {
                break;
            }
        };
        let li = line.trim();
        let parts: Vec<&str> = li.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        if parts[0].to_lowercase() == "set" {
            if parts.len() != 3 {
                match writeln!(stream, "error in number of arguments") {
                    Ok(_) => {}
                    Err(_) => {
                        break;
                    }
                }
                continue;
            }
            // let mut shared = store.lock().unwrap()
            let mut shared = match store.lock() {
                Ok(value) => value,
                Err(_) => {
                    // println!("error");
                    break;
                }
            };
            let key = parts[1];
            let value = parts[2];
            shared.set(key.to_string(), value.to_string());
            let mut file = match OpenOptions::new().append(true).create(true).open(FILE_PATH) {
                Ok(value) => value,
                Err(_) => {
                    break;
                }
            };
            // .unwrap();
            match writeln!(file, "{}", li) {
                Ok(_) => {}
                Err(_) => {
                    break;
                }
            }
            drop(shared);
        } else if parts[0].to_lowercase() == "get" {
            if parts.len() != 2 {
                match writeln!(stream, "error in number of arguments") {
                    Ok(_) => {}
                    Err(_) => {
                        break;
                    }
                }
            }
            let shared = match store.lock() {
                Ok(value) => value,
                Err(_) => {
                    break;
                }
            };
            let key = parts[1];

            if let Some(val) = shared.get(key) {
                match writeln!(stream, "{}", val) {
                    Ok(_) => {}
                    Err(_) => {
                        break;
                    }
                }
            } else {
                match writeln!(stream, "(nil)") {
                    Ok(_) => {}
                    Err(_) => {
                        break;
                    }
                }
            }
            drop(shared);
        } else {
            // let event= writeln!(stream, "invalid command");
            match writeln!(stream, "invalid command") {
                Ok(_) => {}
                Err(_) => {
                    break;
                }
            }
        }
    }
}
