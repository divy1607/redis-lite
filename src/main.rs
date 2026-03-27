use std::sync::{Arc, RwLock};
mod config;
mod handler;
mod rewrite;
mod server;
mod store;
use crate::store::Store;
fn main() {
    let store = Store::new();
    let hash: Arc<RwLock<Store>> = Arc::new(RwLock::new(store));

    server::start_server(hash);
}
