use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;
use superide_sdk::provider::{
    AiProvider, ChatRequest, ChatResponse, ProviderCapabilities, ProviderError,
    ProviderId, ProviderResult, ProviderSettings,
};

use crate::token_usage::{TokenUsageTracker, UsageRecord};

pub struct ProviderService {
    registry: Arc<RwLock<HashMap<String, Box<dyn AiProvider>>>>,
    initialized: Arc<RwLock<HashMap<String, bool>>>,
    active_id: Arc<RwLock<Option<String>>>,
    usage: Arc<TokenUsageTracker>,
}

impl Default for ProviderService {
    fn default() -> Self {
        Self::new()
    }
}

impl ProviderService {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(HashMap::new())),
            initialized: Arc::new(RwLock::new(HashMap::new())),
            active_id: Arc::new(RwLock::new(None)),
            usage: Arc::new(TokenUsageTracker::new()),
        }
    }

    pub fn register(&self, id: &str, provider: Box<dyn AiProvider>) {
        self.registry
            .write()
            .insert(id.to_string(), provider);
        self.initialized.write().insert(id.to_string(), false);
    }

    pub fn usage_tracker(&self) -> &Arc<TokenUsageTracker> {
        &self.usage
    }

    pub fn active_provider(&self) -> Option<String> {
        self.active_id.read().clone()
    }

    pub fn set_active(&self, id: &str) {
        *self.active_id.write() = Some(id.to_string());
    }

    pub fn is_initialized(&self, id: &str) -> bool {
        *self.initialized.read().get(id).unwrap_or(&false)
    }

    pub fn initialize(&self, id: &str, settings: ProviderSettings) -> ProviderResult<()> {
        let exists = {
            let reg = self.registry.read();
            reg.contains_key(id)
        };
        if !exists {
            return Err(ProviderError {
                provider_id: ProviderId(id.to_string()),
                message: format!("Provider '{}' is not registered", id),
            });
        }

        let reg = self.registry.read();
        if let Some(provider) = reg.get(id) {
            provider.initialize(settings)?;
        }

        self.initialized.write().insert(id.to_string(), true);
        Ok(())
    }

    pub fn chat(&self, provider_id: &str, request: ChatRequest) -> ProviderResult<ChatResponse> {
        let reg = self.registry.read();
        let provider = reg.get(provider_id).ok_or_else(|| ProviderError {
            provider_id: ProviderId(provider_id.to_string()),
            message: format!("Provider '{}' is not registered", provider_id),
        })?;

        let response = provider.chat(request)?;

        if let Some(usage) = response.usage {
            self.usage.record(UsageRecord {
                provider_id: provider_id.to_string(),
                model: String::new(),
                input_tokens: usage.input_tokens,
                output_tokens: usage.output_tokens,
            });
        }

        Ok(response)
    }

    pub fn capabilities_of(&self, id: &str) -> Option<ProviderCapabilities> {
        let reg = self.registry.read();
        reg.get(id).map(|p| p.capabilities())
    }

    pub fn list_providers(&self) -> Vec<String> {
        self.registry.read().keys().cloned().collect()
    }
}
