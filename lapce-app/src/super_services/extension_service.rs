use std::path::PathBuf;
use std::sync::Arc;

use parking_lot::RwLock;

use superide_extension_host::contribution_registry::ContributionRegistry;
use superide_extension_host::extension_host::{ExtensionState, SuperExtensionHost};
use superide_sdk::extension::ActivationEvent;

pub struct ExtensionService {
    host: Arc<RwLock<SuperExtensionHost>>,
}

impl ExtensionService {
    pub fn new(search_dirs: Vec<PathBuf>) -> Self {
        Self {
            host: Arc::new(RwLock::new(SuperExtensionHost::new(search_dirs))),
        }
    }

    /// Scan extension directories and load manifests.
    pub fn refresh(&self) -> Vec<String> {
        self.host.write().refresh().unwrap_or_default()
    }

    /// Activate an extension by ID.
    pub fn activate(&self, id: &str) {
        let _ = self.host.write().activate_extension(id);
    }

    /// Activate all extensions matching a given activation event.
    pub fn activate_for_event(&self, event: ActivationEvent) -> Vec<String> {
        self.host.write().activate_for_event(&event)
    }

    /// Get the contribution registry (read-only).
    pub fn contributions(&self) -> Arc<RwLock<ContributionRegistry>> {
        // We need to work around the borrow; return a snapshot.
        Arc::new(RwLock::new(
            self.host.read().contribution_registry().clone(),
        ))
    }

    /// Get the WASM path for an extension, if it has one.
    pub fn wasm_path(&self, id: &str) -> Option<PathBuf> {
        self.host.read().get_wasm_path(id)
    }

    /// List all discovered extensions and their states.
    pub fn list_extensions(&self) -> Vec<(String, ExtensionState)> {
        self.host
            .read()
            .list_extensions()
            .into_iter()
            .map(|(id, state)| (id.to_string(), state.clone()))
            .collect()
    }
}
