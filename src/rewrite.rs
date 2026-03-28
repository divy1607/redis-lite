use std::fs;
use std::sync::Arc;

use crate::config::FILE_PATH;
use crate::store::Store;

pub fn rewrite(store: Arc<Store>) {
    let contents = fs::read_to_string(FILE_PATH).unwrap_or_default();
    let (keys_len, snapshot) = {
        let keys_len = store.len();
        let snapshot: Vec<(String, String)> = store
            .shards
            .iter()
            .flat_map(|shard| {
                let map = match shard.lock.read() {
                    Ok(val) => val,
                    Err(poisoned) => poisoned.into_inner(),
                };
                map.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect::<Vec<_>>()
            })
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
