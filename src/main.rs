use std::sync::{Arc, Mutex};
mod handler;
mod server;
mod store;
use crate::store::Store;
fn main() {
    let store = Store::new();
    let hash: Arc<Mutex<Store>> = Arc::new(Mutex::new(store));

    server::start_server(hash);
}
