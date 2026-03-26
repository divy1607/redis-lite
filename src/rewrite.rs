use std::fs;
use std::sync::{Arc, Mutex};

use crate::config::FILE_PATH;
use crate::store::Store;

pub fn rewrite(store: Arc<Mutex<Store>>) {
    let contents = fs::read_to_string(FILE_PATH).unwrap_or_default();
    let (keys_len, snapshot) = {
        let shared = store.lock().unwrap();
        let keys_len = shared.len();
        let snapshot: Vec<(String, String)> = shared
            .hash
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        (keys_len, snapshot)
    };
    if contents.lines().count() > 3 * keys_len {
        let mut lines = String::new();
        for (k, v) in snapshot {
            let line = format!("SET {} {}\n", k, v);
            lines.push_str(&line);
        }
        let temp_path = format!("{}.tmp", FILE_PATH);
        fs::write(&temp_path, lines).expect("unable to rewrite file");
        fs::rename(&temp_path, FILE_PATH).expect("unable to replace AOF");
    }
}
