/// Known model definitions for provider auto-complete and display.

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub id: &'static str,
    pub display_name: &'static str,
    pub provider: &'static str,
    pub supports_tools: bool,
    pub supports_streaming: bool,
}

pub static KNOWN_MODELS: &[ModelInfo] = &[
    // OpenAI
    ModelInfo { id: "gpt-4o", display_name: "GPT-4o", provider: "openai", supports_tools: true, supports_streaming: true },
    ModelInfo { id: "gpt-4o-mini", display_name: "GPT-4o Mini", provider: "openai", supports_tools: true, supports_streaming: true },
    ModelInfo { id: "gpt-4-turbo", display_name: "GPT-4 Turbo", provider: "openai", supports_tools: true, supports_streaming: true },
    // Anthropic
    ModelInfo { id: "claude-sonnet-4-20250514", display_name: "Claude Sonnet 4", provider: "anthropic", supports_tools: true, supports_streaming: true },
    ModelInfo { id: "claude-3-5-sonnet-20241022", display_name: "Claude 3.5 Sonnet", provider: "anthropic", supports_tools: true, supports_streaming: true },
    ModelInfo { id: "claude-3-5-haiku-20241022", display_name: "Claude 3.5 Haiku", provider: "anthropic", supports_tools: true, supports_streaming: true },
    // OpenRouter
    ModelInfo { id: "openai/gpt-4o", display_name: "OpenRouter GPT-4o", provider: "openrouter", supports_tools: true, supports_streaming: true },
    ModelInfo { id: "anthropic/claude-sonnet-4", display_name: "OpenRouter Claude Sonnet 4", provider: "openrouter", supports_tools: true, supports_streaming: true },
    ModelInfo { id: "google/gemini-2.0-flash-001", display_name: "Gemini 2.0 Flash", provider: "openrouter", supports_tools: true, supports_streaming: true },
    ModelInfo { id: "meta-llama/llama-3.3-70b-instruct", display_name: "Llama 3.3 70B", provider: "openrouter", supports_tools: true, supports_streaming: true },
    // Ollama
    ModelInfo { id: "llama3.2", display_name: "Llama 3.2 (local)", provider: "ollama", supports_tools: false, supports_streaming: true },
    ModelInfo { id: "qwen2.5", display_name: "Qwen 2.5 (local)", provider: "ollama", supports_tools: false, supports_streaming: true },
    ModelInfo { id: "deepseek-r1", display_name: "DeepSeek R1 (local)", provider: "ollama", supports_tools: false, supports_streaming: true },
    ModelInfo { id: "mistral", display_name: "Mistral (local)", provider: "ollama", supports_tools: false, supports_streaming: true },
    // DeepSeek
    ModelInfo { id: "deepseek-chat", display_name: "DeepSeek Chat", provider: "deepseek", supports_tools: true, supports_streaming: true },
    ModelInfo { id: "deepseek-reasoner", display_name: "DeepSeek Reasoner", provider: "deepseek", supports_tools: false, supports_streaming: true },
    // Qwen
    ModelInfo { id: "qwen-max", display_name: "Qwen Max", provider: "qwen", supports_tools: true, supports_streaming: true },
    ModelInfo { id: "qwen-plus", display_name: "Qwen Plus", provider: "qwen", supports_tools: true, supports_streaming: true },
    // Grok
    ModelInfo { id: "grok-3", display_name: "Grok 3", provider: "grok", supports_tools: true, supports_streaming: true },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_models_not_empty() {
        assert!(!KNOWN_MODELS.is_empty());
    }

    #[test]
    fn test_models_have_unique_ids() {
        let mut ids: Vec<&str> = KNOWN_MODELS.iter().map(|m| m.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), KNOWN_MODELS.len());
    }

    #[test]
    fn test_find_model_by_id() {
        let model = KNOWN_MODELS.iter().find(|m| m.id == "gpt-4o");
        assert!(model.is_some());
        assert_eq!(model.unwrap().provider, "openai");
    }

    #[test]
    fn test_models_by_provider() {
        let openai: Vec<&ModelInfo> = KNOWN_MODELS.iter().filter(|m| m.provider == "openai").collect();
        assert!(openai.len() >= 3);
    }
}
