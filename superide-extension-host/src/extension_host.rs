use std::collections::HashMap;
use std::path::PathBuf;

use superide_sdk::extension::{
    ActivationEvent, ExtensionError, ExtensionHost, ExtensionManifest, ExtensionResult,
};

use crate::contribution_registry::ContributionRegistry;
use crate::discovery::discover_extensions;
use crate::manifest::SuperExtensionManifest;

/// State of a loaded extension.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionState {
    Discovered,
    Activated,
    Deactivated,
}

pub struct SuperExtensionHost {
    extensions: HashMap<String, ManagedExtension>,
    contribution_registry: ContributionRegistry,
    search_dirs: Vec<PathBuf>,
}

struct ManagedExtension {
    manifest: SuperExtensionManifest,
    state: ExtensionState,
    root: PathBuf,
}

impl SuperExtensionHost {
    pub fn new(search_dirs: Vec<PathBuf>) -> Self {
        Self {
            extensions: HashMap::new(),
            contribution_registry: ContributionRegistry::new(),
            search_dirs,
        }
    }

    /// Refresh the extension catalog by scanning all search directories.
    pub fn refresh(&mut self) -> Result<Vec<String>, ExtensionError> {
        let discovered = discover_extensions(&self.search_dirs)?;
        let mut ids = Vec::new();

        for ext in discovered {
            let id = ext.manifest.id.clone();
            self.extensions.insert(
                id.clone(),
                ManagedExtension {
                    manifest: ext.manifest,
                    state: ExtensionState::Discovered,
                    root: ext.root,
                },
            );
            ids.push(id);
        }

        Ok(ids)
    }

    /// Activate an extension by ID.
    pub fn activate_extension(&mut self, extension_id: &str) -> ExtensionResult<()> {
        let ext = self
            .extensions
            .get_mut(extension_id)
            .ok_or_else(|| ExtensionError {
                extension_id: extension_id.to_string(),
                message: "Extension not found".to_string(),
            })?;

        if ext.state == ExtensionState::Activated {
            return Ok(()); // Already activated
        }

        // Register contributions
        let sdk_manifest = ext.manifest.to_sdk();
        self.contribution_registry
            .register_extension(extension_id, &sdk_manifest.contributions);

        ext.state = ExtensionState::Activated;
        tracing::info!(
            "Activated extension '{}' v{}",
            extension_id,
            ext.manifest.version
        );

        Ok(())
    }

    /// Activate all extensions that match a given activation event.
    pub fn activate_for_event(&mut self, event: &ActivationEvent) -> Vec<String> {
        let mut activated = Vec::new();
        let ids: Vec<String> = self.extensions.keys().cloned().collect();

        for id in ids {
            let should_activate = self.extensions.get(&id).map(|ext| {
                let sdk = ext.manifest.to_sdk();
                sdk.activation_events.contains(event)
            });

            if should_activate == Some(true) {
                if self.activate_extension(&id).is_ok() {
                    activated.push(id);
                }
            }
        }

        activated
    }

    pub fn contribution_registry(&self) -> &ContributionRegistry {
        &self.contribution_registry
    }

    pub fn list_extensions(&self) -> Vec<(&str, &ExtensionState)> {
        self.extensions
            .iter()
            .map(|(id, ext)| (id.as_str(), &ext.state))
            .collect()
    }

    pub fn get_wasm_path(&self, extension_id: &str) -> Option<PathBuf> {
        self.extensions.get(extension_id).and_then(|ext| {
            ext.manifest.wasm.as_ref().map(|wasm| ext.root.join(&wasm.path))
        })
    }
}

impl ExtensionHost for SuperExtensionHost {
    fn load_manifest(&self, path: &str) -> ExtensionResult<ExtensionManifest> {
        let content =
            std::fs::read_to_string(path).map_err(|e| ExtensionError {
                extension_id: path.to_string(),
                message: format!("Failed to read manifest: {}", e),
            })?;
        let manifest: SuperExtensionManifest =
            serde_json::from_str(&content).map_err(|e| ExtensionError {
                extension_id: path.to_string(),
                message: format!("Failed to parse manifest: {}", e),
            })?;
        Ok(manifest.to_sdk())
    }

    fn activate(&self, _extension_id: &str, _reason: ActivationEvent) -> ExtensionResult<()> {
        Err(ExtensionError {
            extension_id: _extension_id.to_string(),
            message: "Use SuperExtensionHost::activate for mutable access".to_string(),
        })
    }

    fn deactivate(&self, _extension_id: &str) -> ExtensionResult<()> {
        Err(ExtensionError {
            extension_id: _extension_id.to_string(),
            message: "Not implemented".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_extension(dir: &std::path::Path, id: &str, events: &[&str]) {
        let ext_dir = dir.join(id);
        std::fs::create_dir_all(&ext_dir).unwrap();

        let activation: Vec<String> = events.iter().map(|s| s.to_string()).collect();
        let manifest = serde_json::json!({
            "id": id,
            "name": id,
            "version": "1.0.0",
            "activation_events": activation,
            "contributes": {
                "commands": [
                    { "id": format!("{}.hello", id), "title": format!("{} Hello", id) }
                ]
            }
        });

        std::fs::write(
            ext_dir.join("super-extension.json"),
            serde_json::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn test_discover_and_activate() {
        let dir = tempfile::tempdir().unwrap();
        create_test_extension(dir.path(), "ext-a", &["onStartupFinished"]);
        create_test_extension(dir.path(), "ext-b", &["onLanguage:rust"]);

        let mut host = SuperExtensionHost::new(vec![dir.path().to_path_buf()]);
        let ids = host.refresh().unwrap();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&"ext-a".to_string()));
        assert!(ids.contains(&"ext-b".to_string()));

        // Activate ext-a
        host.activate_extension("ext-a").unwrap();
        assert_eq!(
            host.extensions.get("ext-a").unwrap().state,
            ExtensionState::Activated
        );

        // Check command registered
        let cmd = host.contribution_registry.find_command("ext-a.hello");
        assert!(cmd.is_some());
        assert_eq!(cmd.unwrap().extension_id, "ext-a");
    }

    #[test]
    fn test_activate_by_event() {
        let dir = tempfile::tempdir().unwrap();
        create_test_extension(dir.path(), "ext-a", &["onStartupFinished"]);
        create_test_extension(dir.path(), "ext-b", &["onLanguage:rust"]);

        let mut host = SuperExtensionHost::new(vec![dir.path().to_path_buf()]);
        host.refresh().unwrap();

        let activated = host.activate_for_event(&ActivationEvent::OnStartupFinished);
        assert_eq!(activated, vec!["ext-a"]);

        let activated = host.activate_for_event(&ActivationEvent::OnLanguage("rust".to_string()));
        assert_eq!(activated, vec!["ext-b"]);
    }
}
