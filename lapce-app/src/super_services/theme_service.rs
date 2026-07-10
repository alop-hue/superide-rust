use std::sync::Arc;
use parking_lot::RwLock;

pub struct ThemeService {
    current_theme: Arc<RwLock<String>>,
}

impl Default for ThemeService {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeService {
    pub fn new() -> Self {
        Self {
            current_theme: Arc::new(RwLock::new("SUPER Orange Dark".to_string())),
        }
    }

    pub fn current(&self) -> String {
        self.current_theme.read().clone()
    }

    pub fn set_theme(&self, name: &str) {
        *self.current_theme.write() = name.to_string();
    }
}