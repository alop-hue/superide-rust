use superide_ai::ProviderService;
use superide_sdk::provider::{
    ChatMessage, ChatRequest,
};

use crate::config::agent::AgentConfig;

const AI_COMPLETION_SYSTEM_PROMPT: &str = "You are an AI code completion engine. \
Given the code context before and after the cursor position, provide the most likely \
completion text that the developer is about to type. Only output the completion text. \
Do NOT include any explanation, markdown formatting, backticks, or the original context. \
Just the raw completion. Keep it concise — typically a single line or expression.";

pub fn generate_ai_completion(
    text_before: &str,
    text_after: &str,
    provider_service: &ProviderService,
    config: &AgentConfig,
) -> Option<String> {
    let provider_id = provider_service.active_provider()?;

    let user_message = format!(
        "Complete the code at the cursor marked by <CURSOR>:\n\n\
        ```\n{code_before}<CURSOR>{code_after}\n```\n\n\
        Only output the completion text that should replace <CURSOR>.",
        code_before = text_before,
        code_after = text_after,
    );

    let request = ChatRequest {
        model: config.model.clone(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: AI_COMPLETION_SYSTEM_PROMPT.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_message,
            },
        ],
        tools_enabled: false,
    };

    let response = provider_service.chat(&provider_id, request).ok()?;
    let completion = response.message.content.trim().to_string();

    if completion.is_empty() {
        return None;
    }

    Some(completion)
}
