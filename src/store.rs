use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    sync::RwLock,
};

pub struct Store {
    pub shards: Vec<Shard>,
}

pub struct Shard {
    pub lock: RwLock<HashMap<String, String>>,
}

impl Store {
    pub fn new(nshards: usize) -> Self {
        assert!(nshards > 0);
        let mut shards = Vec::with_capacity(nshards);
        for _ in 0..nshards {
            shards.push(Shard {
                lock: RwLock::new(HashMap::new()),
            });
        }
        Store { shards }
    }

    pub fn get_shard_index(&self, key: &String) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        let hash = s.finish();
        hash as usize % self.shards.len()
    }

    pub fn set(&self, key: String, value: String) {
        let index = self.get_shard_index(&key);
        let shard = &self.shards[index];

        let _map = match shard.lock.write() {
            Ok(mut val) => val.insert(key, value),
            Err(_) => {
                return;
            }
        };
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let index = self.get_shard_index(&key.to_string());
        let shard = &self.shards[index];

        match shard.lock.read() {
            Ok(val) => val.get(key).cloned(),
            Err(poisoned) => poisoned.into_inner().get(key).cloned(),
        }
    }

    pub fn len(&self) -> usize {
        let mut total: usize = 0;

        for shard in &self.shards {
            let map = match shard.lock.read() {
                Ok(val) => val,
                Err(_) => return 0,
            };
            total += map.len();
        }
        total
    }
}
