use serde::{Deserialize, Serialize};
use superide_sdk::extension::{ActivationEvent, Contribution};

/// SUPER IDE extension manifest (supersedes Lapce's volt.toml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperExtensionManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub wasm: Option<WasmConfig>,
    #[serde(default)]
    pub activation_events: Vec<String>,
    #[serde(default)]
    pub contributes: ManifestContributions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmConfig {
    pub path: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ManifestContributions {
    #[serde(default)]
    pub commands: Vec<ManifestCommand>,
    #[serde(default)]
    pub themes: Vec<ManifestTheme>,
    #[serde(default)]
    pub icon_themes: Vec<ManifestIconTheme>,
    #[serde(default)]
    pub views: Vec<ManifestView>,
    #[serde(default)]
    pub settings: Vec<ManifestSettings>,
    #[serde(default)]
    pub providers: Vec<String>,
    #[serde(default)]
    pub agents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestCommand {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub shortcut: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestTheme {
    pub id: String,
    pub path: String,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestIconTheme {
    pub id: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestView {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestSettings {
    pub path: String,
}

impl SuperExtensionManifest {
    /// Convert SUPER manifest to SDK ExtensionManifest.
    pub fn to_sdk(&self) -> superide_sdk::extension::ExtensionManifest {
        let mut activation_events = Vec::new();
        for event_str in &self.activation_events {
            if let Some(event) = parse_activation_event(event_str) {
                activation_events.push(event);
            }
        }

        let mut contributions = Vec::new();
        for cmd in &self.contributes.commands {
            contributions.push(Contribution::Command {
                id: cmd.id.clone(),
                title: cmd.title.clone(),
            });
        }
        for theme in &self.contributes.themes {
            contributions.push(Contribution::Theme {
                id: theme.id.clone(),
                path: theme.path.clone(),
            });
        }
        for it in &self.contributes.icon_themes {
            contributions.push(Contribution::IconTheme {
                id: it.id.clone(),
                path: it.path.clone(),
            });
        }
        for view in &self.contributes.views {
            contributions.push(Contribution::SidebarView {
                id: view.id.clone(),
                title: view.title.clone(),
            });
        }
        for s in &self.contributes.settings {
            contributions.push(Contribution::SettingsSchema {
                path: s.path.clone(),
            });
        }
        for p in &self.contributes.providers {
            contributions.push(Contribution::Provider { id: p.clone() });
        }
        for a in &self.contributes.agents {
            contributions.push(Contribution::Agent { id: a.clone() });
        }

        superide_sdk::extension::ExtensionManifest {
            id: self.id.clone(),
            display_name: self.name.clone(),
            version: self.version.clone(),
            activation_events,
            contributions,
        }
    }
}

fn parse_activation_event(s: &str) -> Option<ActivationEvent> {
    match s {
        "onStartupFinished" => Some(ActivationEvent::OnStartupFinished),
        "onAiRequested" => Some(ActivationEvent::OnAiRequested),
        s if s.starts_with("onLanguage:") => {
            Some(ActivationEvent::OnLanguage(s.trim_start_matches("onLanguage:").to_string()))
        }
        s if s.starts_with("onCommand:") => {
            Some(ActivationEvent::OnCommand(s.trim_start_matches("onCommand:").to_string()))
        }
        s if s.starts_with("onView:") => {
            Some(ActivationEvent::OnView(s.trim_start_matches("onView:").to_string()))
        }
        s if s.starts_with("workspaceContains:") => {
            Some(ActivationEvent::WorkspaceContains(s.trim_start_matches("workspaceContains:").to_string()))
        }
        _ => {
            tracing::warn!("Unknown activation event: {}", s);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_activation_events() {
        assert_eq!(parse_activation_event("onStartupFinished"), Some(ActivationEvent::OnStartupFinished));
        assert_eq!(parse_activation_event("onAiRequested"), Some(ActivationEvent::OnAiRequested));
        assert_eq!(parse_activation_event("onLanguage:rust"), Some(ActivationEvent::OnLanguage("rust".to_string())));
        assert_eq!(parse_activation_event("onCommand:editor.format"), Some(ActivationEvent::OnCommand("editor.format".to_string())));
        assert_eq!(parse_activation_event("onView:explorer"), Some(ActivationEvent::OnView("explorer".to_string())));
        assert_eq!(parse_activation_event("workspaceContains:Cargo.toml"), Some(ActivationEvent::WorkspaceContains("Cargo.toml".to_string())));
        assert_eq!(parse_activation_event("unknownEvent"), None);
    }

    #[test]
    fn test_manifest_roundtrip() {
        let json = r#"{
            "id": "test-ext",
            "name": "Test Extension",
            "version": "1.0.0",
            "activation_events": ["onStartupFinished"],
            "contributes": {
                "commands": [
                    { "id": "test.hello", "title": "Say Hello" }
                ]
            }
        }"#;
        let manifest: SuperExtensionManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.id, "test-ext");
        assert_eq!(manifest.contributes.commands.len(), 1);
        assert_eq!(manifest.contributes.commands[0].id, "test.hello");

        let sdk = manifest.to_sdk();
        assert_eq!(sdk.id, "test-ext");
        assert_eq!(sdk.contributions.len(), 1);
    }
}
