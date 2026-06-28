use crate::{SdkFuture, provider::ProviderId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentMode {
    Chat,
    Plan,
    Edit,
    Review,
    Debug,
    Research,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentBackendKind {
    Native,
    ClaudeCode,
    Codex,
    OpenCode,
    Blackbox,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentContext {
    pub workspace_root: Option<String>,
    pub active_file: Option<String>,
    pub selected_text: Option<String>,
    pub provider_id: Option<ProviderId>,
    pub mode: AgentMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentRequest {
    pub prompt: String,
    pub context: AgentContext,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentAction {
    Think { summary: String },
    ReadFile { path: String },
    WriteFile { path: String },
    RunTerminal { command: String },
    SearchWorkspace { query: String },
    AskUser { prompt: String },
    Done { summary: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentStep {
    pub action: AgentAction,
    pub visible_to_user: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentError {
    pub agent_id: String,
    pub message: String,
}

pub type AgentResult<T> = Result<T, AgentError>;

pub trait AgentBackend: Send + Sync {
    fn id(&self) -> &str;

    fn kind(&self) -> AgentBackendKind;

    fn start<'a>(&'a self, request: AgentRequest) -> SdkFuture<'a, AgentResult<()>>;

    fn next_step<'a>(&'a self) -> SdkFuture<'a, AgentResult<AgentStep>>;

    fn stop<'a>(&'a self) -> SdkFuture<'a, AgentResult<()>>;
}
