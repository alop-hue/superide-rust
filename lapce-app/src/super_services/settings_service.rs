use std::{collections::HashMap, sync::Arc};
use parking_lot::RwLock;
use serde_json::Value;

pub struct SettingsService {
    settings: Arc<RwLock<HashMap<String, Value>>>,
    listeners: Arc<RwLock<Vec<Box<dyn Fn(String, Value) + Send>>>>,
}

impl Default for SettingsService {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsService {
    pub fn new() -> Self {
        Self {
            settings: Arc::new(RwLock::new(HashMap::new())),
            listeners: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        self.settings.read().get(key).cloned()
    }

    pub fn set(&self, key: &str, value: Value) {
        self.settings.write().insert(key.to_string(), value.clone());
        for listener in self.listeners.read().iter() {
            listener(key.to_string(), value.clone());
        }
    }

    pub fn on_change(&self, handler: Box<dyn Fn(String, Value) + Send>) {
        self.listeners.write().push(handler);
    }
}