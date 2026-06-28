use superide_sdk::provider::{
    AiProvider, ChatMessage, ChatRequest, ChatResponse, ProviderCapabilities,
    ProviderError, ProviderId, ProviderKind, ProviderResult, ProviderSettings,
    TokenUsage,
};

pub struct QwenProvider {
    id: ProviderId,
    api_key: String,
    endpoint: String,
}

impl QwenProvider {
    pub fn new() -> Self {
        Self {
            id: ProviderId("qwen".to_string()),
            api_key: String::new(),
            endpoint: "https://dashscope.aliyuncs.com/compatible-mode/v1".to_string(),
        }
    }
}

impl AiProvider for QwenProvider {
    fn id(&self) -> ProviderId { self.id.clone() }

    fn kind(&self) -> ProviderKind { ProviderKind::Qwen }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities { chat: true, tools: true, streaming: true, local: false }
    }

    fn initialize(&self, settings: ProviderSettings) -> ProviderResult<()> {
        let api_key = settings.values.iter()
            .find(|s| s.key == "api_key")
            .map(|s| s.value.clone())
            .unwrap_or_default();
        if api_key.is_empty() {
            return Err(ProviderError {
                provider_id: self.id.clone(),
                message: "Qwen (DashScope) API key is required".to_string(),
            });
        }
        Ok(())
    }

    fn chat(&self, request: ChatRequest) -> ProviderResult<ChatResponse> {
        let url = format!("{}/chat/completions", self.endpoint);
        let messages: Vec<serde_json::Value> = request.messages.iter().map(|m| {
            serde_json::json!({"role": m.role, "content": m.content})
        }).collect();

        let body = serde_json::json!({
            "model": request.model,
            "messages": messages,
            "max_tokens": 4096,
        });

        let client = reqwest::blocking::Client::new();
        let resp = client.post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .map_err(|e| ProviderError { provider_id: self.id.clone(), message: format!("Request failed: {}", e) })?;

        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        if !status.is_success() {
            return Err(ProviderError {
                provider_id: self.id.clone(),
                message: format!("API error ({}): {}", status, text),
            });
        }

        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| ProviderError { provider_id: self.id.clone(), message: format!("Parse error: {}", e) })?;

        let content = json["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();
        let usage = json["usage"].as_object().map(|u| TokenUsage {
            input_tokens: u["prompt_tokens"].as_u64().unwrap_or(0),
            output_tokens: u["completion_tokens"].as_u64().unwrap_or(0),
        });

        Ok(ChatResponse {
            message: ChatMessage { role: "assistant".to_string(), content },
            usage,
        })
    }
}
