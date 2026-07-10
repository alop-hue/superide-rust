use superide_sdk::provider::{
    AiProvider, ChatMessage, ChatRequest, ChatResponse, ProviderCapabilities,
    ProviderError, ProviderId, ProviderKind, ProviderResult, ProviderSettings,
};

pub struct OllamaProvider {
    id: ProviderId,
    endpoint: String,
}

impl OllamaProvider {
    pub fn new() -> Self {
        Self {
            id: ProviderId("ollama".to_string()),
            endpoint: "http://localhost:11434".to_string(),
        }
    }
}

impl AiProvider for OllamaProvider {
    fn id(&self) -> ProviderId {
        self.id.clone()
    }

    fn kind(&self) -> ProviderKind {
        ProviderKind::Ollama
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            chat: true,
            tools: false,
            streaming: true,
            local: true,
        }
    }

    fn initialize(&self, _settings: ProviderSettings) -> ProviderResult<()> {
        Ok(())
    }

    fn chat(&self, request: ChatRequest) -> ProviderResult<ChatResponse> {
        let url = format!("{}/api/chat", self.endpoint);

        let messages: Vec<serde_json::Value> = request
            .messages
            .iter()
            .map(|m| {
                serde_json::json!({
                    "role": m.role,
                    "content": m.content,
                })
            })
            .collect();

        let body = serde_json::json!({
            "model": request.model,
            "messages": messages,
            "stream": false,
        });

        let client = reqwest::blocking::Client::new();
        let resp = client
            .post(&url)
            .json(&body)
            .send()
            .map_err(|e| ProviderError {
                provider_id: self.id.clone(),
                message: format!("Ollama request failed: {}", e),
            })?;

        let status = resp.status();
        let text = resp.text().unwrap_or_default();

        if !status.is_success() {
            return Err(ProviderError {
                provider_id: self.id.clone(),
                message: format!("Ollama error ({}): {}", status, text),
            });
        }

        let json: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| ProviderError {
                provider_id: self.id.clone(),
                message: format!("Failed to parse response: {}", e),
            })?;

        let content = json["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(ChatResponse {
            message: ChatMessage {
                role: "assistant".to_string(),
                content,
            },
            usage: None,
        })
    }
}
