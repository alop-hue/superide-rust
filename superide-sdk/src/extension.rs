#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivationEvent {
    OnStartupFinished,
    OnLanguage(String),
    OnCommand(String),
    OnView(String),
    OnAiRequested,
    WorkspaceContains(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Contribution {
    Command { id: String, title: String },
    Theme { id: String, path: String },
    IconTheme { id: String, path: String },
    SidebarView { id: String, title: String },
    SettingsSchema { path: String },
    Provider { id: String },
    Agent { id: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionManifest {
    pub id: String,
    pub display_name: String,
    pub version: String,
    pub activation_events: Vec<ActivationEvent>,
    pub contributions: Vec<Contribution>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionError {
    pub extension_id: String,
    pub message: String,
}

pub type ExtensionResult<T> = Result<T, ExtensionError>;

pub trait ExtensionHost: Send + Sync {
    fn load_manifest(&self, path: &str) -> ExtensionResult<ExtensionManifest>;

    fn activate(
        &self,
        extension_id: &str,
        reason: ActivationEvent,
    ) -> ExtensionResult<()>;

    fn deactivate(&self, extension_id: &str) -> ExtensionResult<()>;
}
