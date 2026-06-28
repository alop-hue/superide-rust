use std::{collections::HashMap, sync::Arc};
use parking_lot::RwLock;
use serde_json::Value;

pub struct StateStore {
    store: Arc<RwLock<HashMap<String, Value>>>,
}

impl Default for StateStore {
    fn default() -> Self {
        Self::new()
    }
}

impl StateStore {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        self.store.read().get(key).cloned()
    }

    pub fn set(&self, key: &str, value: Value) {
        self.store.write().insert(key.to_string(), value);
    }

    pub fn remove(&self, key: &str) {
        self.store.write().remove(key);
    }

    pub fn clear(&self) {
        self.store.write().clear();
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(|v| v.as_bool())
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.get(key).and_then(|v| v.as_str().map(|s| s.to_string()))
    }
}