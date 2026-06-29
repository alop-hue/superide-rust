#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProviderId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderKind {
    OpenAi,
    OpenRouter,
    Anthropic,
    Grok,
    DeepSeek,
    Qwen,
    Glm,
    Kimi,
    MiniMax,
    Ollama,
    OpenAiCompatible,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderCapabilities {
    pub chat: bool,
    pub tools: bool,
    pub streaming: bool,
    pub local: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderSetting {
    pub key: String,
    pub value: String,
    pub secret: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderSettings {
    pub endpoint: Option<String>,
    pub model: Option<String>,
    pub values: Vec<ProviderSetting>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub tools_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatResponse {
    pub message: ChatMessage,
    pub usage: Option<TokenUsage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderError {
    pub provider_id: ProviderId,
    pub message: String,
}

pub type ProviderResult<T> = Result<T, ProviderError>;

pub trait AiProvider: Send + Sync {
    fn id(&self) -> ProviderId;

    fn kind(&self) -> ProviderKind;

    fn capabilities(&self) -> ProviderCapabilities;

    fn initialize(
        &self,
        settings: ProviderSettings,
    ) -> ProviderResult<()>;

    fn chat(
        &self,
        request: ChatRequest,
    ) -> ProviderResult<ChatResponse>;
}
