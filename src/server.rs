use crate::config::FILE_PATH;
use crate::rewrite;
use crate::{handler::handle_connection, store::Store};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::{fs, sync::Arc, thread};
use tokio::net::TcpListener;

pub fn start() -> Sender<String> {
    let (tx, rx) = mpsc::channel();

    let mut file = match OpenOptions::new().append(true).create(true).open(FILE_PATH) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Failed to open file {}: {}", FILE_PATH, e);
            std::process::exit(1);
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

    tx
}

pub fn helper(store: Arc<Store>) {
    let contents = match fs::read_to_string(FILE_PATH) {
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
            store.set(key.to_string(), value.to_string());
        }
    }
    rewrite::rewrite(store.clone());
}

pub async fn start_server(store: Arc<Store>, tx: Sender<String>) {
    let listener = match TcpListener::bind("127.0.0.1:8000").await {
        Ok(value) => value,
        Err(_) => {
            return;
        }
    };

    loop {
        let (stream, _) = match listener.accept().await {
            Ok(value) => value,
            Err(e) => {
                eprintln!("some error occured: {:?}", e);
                continue;
            }
        };
        let value = store.clone();
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            match handle_connection(stream, value, tx_clone).await {
                Err(e) => {
                    eprintln!("some error occured: {:?}", e);
                }
            }
        });
    }
}
