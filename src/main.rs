use std::sync::Arc;
mod config;
mod handler;
mod rewrite;
mod server;
mod store;
use crate::server::{helper, start};
use crate::store::Store;
#[tokio::main]
async fn main() {
    let store = Arc::new(Store::new(32));
    helper(store.clone());
    let tx = start();
    server::start_server(store, tx).await;
}
