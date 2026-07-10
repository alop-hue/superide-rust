use serde::{Deserialize, Serialize};
use structdesc::FieldNames;

#[derive(FieldNames, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AgentConfig {
    #[field_names(desc = "Enable the AI agent panel")]
    pub enable_agent: bool,
    #[field_names(desc = "The AI provider to use (openai, anthropic, google, ollama)")]
    pub provider: String,
    #[field_names(desc = "The model name for the chosen provider")]
    pub model: String,
    #[field_names(desc = "Temperature for model responses (0.0 - 2.0)")]
    pub temperature: f64,
    #[field_names(desc = "Maximum tokens per response")]
    pub max_tokens: usize,
    #[field_names(desc = "Enable DuckDuckGo web search for the agent")]
    pub enable_search: bool,
    #[field_names(desc = "Show agent thinking steps in the UI")]
    pub show_thinking: bool,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            enable_agent: true,
            provider: "openai".to_string(),
            model: "gpt-4o".to_string(),
            temperature: 0.7,
            max_tokens: 4096,
            enable_search: true,
            show_thinking: true,
        }
    }
}
