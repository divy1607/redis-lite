use std::sync::{Arc};
mod config;
mod handler;
mod rewrite;
mod server;
mod store;
use crate::store::{Store};
fn main() {
    let store = Arc::new(Store::new(32));
    server::start_server(store);
}
