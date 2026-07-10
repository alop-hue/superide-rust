use std::path::{Path, PathBuf};

use superide_sdk::extension::ExtensionError;

use crate::manifest::SuperExtensionManifest;

fn home_dir() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
}

/// Standard SUPER IDE extension directories.
pub const EXTENSION_DIRS: &[&str] = &[
    ".super-ide/extensions",
    ".config/super-ide/extensions",
    ".local/share/super-ide/extensions",
];

/// Discover extensions installed in the given search paths.
pub fn discover_extensions(
    search_dirs: &[PathBuf],
) -> Result<Vec<DiscoveredExtension>, ExtensionError> {
    let mut extensions = Vec::new();

    for dir in search_dirs {
        if !dir.exists() {
            continue;
        }
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let manifest_path = path.join("super-extension.json");
            if !manifest_path.exists() {
                continue;
            }
            match load_manifest(&manifest_path) {
                Ok(manifest) => {
                    extensions.push(DiscoveredExtension {
                        manifest,
                        root: path,
                        manifest_path,
                    });
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to load extension manifest at {}: {}",
                        manifest_path.display(),
                        e.message
                    );
                }
            }
        }
    }

    Ok(extensions)
}

/// Discover extensions relative to a workspace root.
pub fn discover_workspace_extensions(
    workspace_root: &Path,
) -> Result<Vec<DiscoveredExtension>, ExtensionError> {
    let mut dirs = Vec::new();

    // Check workspace .super-ide/extensions
    let local = workspace_root.join(".super-ide").join("extensions");
    if local.exists() {
        dirs.push(local);
    }

    // Check XDG config/data paths relative to home
    if let Some(home) = home_dir() {
        for sub in EXTENSION_DIRS {
            let p = home.join(sub);
            if p.exists() {
                dirs.push(p);
            }
        }
    }

    discover_extensions(&dirs)
}

#[derive(Debug, Clone)]
pub struct DiscoveredExtension {
    pub manifest: SuperExtensionManifest,
    pub root: PathBuf,
    pub manifest_path: PathBuf,
}

fn load_manifest(path: &Path) -> Result<SuperExtensionManifest, ExtensionError> {
    let content = std::fs::read_to_string(path).map_err(|e| ExtensionError {
        extension_id: path.to_string_lossy().to_string(),
        message: format!("Failed to read manifest: {}", e),
    })?;
    let manifest: SuperExtensionManifest = serde_json::from_str(&content).map_err(|e| {
        ExtensionError {
            extension_id: path.to_string_lossy().to_string(),
            message: format!("Failed to parse manifest: {}", e),
        }
    })?;
    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let extensions = discover_extensions(&[dir.path().to_path_buf()]).unwrap();
        assert!(extensions.is_empty());
    }

    #[test]
    fn test_load_valid_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let ext_dir = dir.path().join("my-ext");
        std::fs::create_dir_all(&ext_dir).unwrap();

        let manifest_content = r#"{
            "id": "my-ext",
            "name": "My Extension",
            "version": "0.1.0",
            "activation_events": ["onStartupFinished"],
            "contributes": {}
        }"#;
        std::fs::write(ext_dir.join("super-extension.json"), manifest_content).unwrap();

        let extensions = discover_extensions(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(extensions.len(), 1);
        assert_eq!(extensions[0].manifest.id, "my-ext");
    }

    #[test]
    fn test_skip_invalid_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let ext_dir = dir.path().join("bad-ext");
        std::fs::create_dir_all(&ext_dir).unwrap();
        std::fs::write(ext_dir.join("super-extension.json"), "not valid json").unwrap();

        let extensions = discover_extensions(&[dir.path().to_path_buf()]).unwrap();
        assert!(extensions.is_empty());
    }
}
