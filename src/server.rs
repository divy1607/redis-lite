use std::{net::TcpListener, sync::{Arc, Mutex}, thread};

use crate::{handler::handle_connection, store::Store};


pub fn start_server(store: Arc<Mutex<Store>>) {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let value = store.clone();
        thread::spawn(move || {
            handle_connection(stream, value);
        });
    }
}
