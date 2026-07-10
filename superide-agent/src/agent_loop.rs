use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use superide_sdk::agent::{
    AgentAction, AgentBackend, AgentBackendKind, AgentError, AgentMode, AgentRequest,
    AgentResult, AgentStep,
};

use crate::context_manager::ContextManager;
use crate::tool_registry::{ToolCall, ToolRegistry};

pub struct AgentLoop {
    pub mode: AgentMode,
    pub context: ContextManager,
    pub tools: ToolRegistry,
    pub max_steps: usize,
}

impl AgentLoop {
    pub fn new(_mode: AgentMode, workspace_root: Option<PathBuf>) -> Self {
        Self {
            mode: _mode,
            context: ContextManager::new(workspace_root),
            tools: ToolRegistry::new(),
            max_steps: 25,
        }
    }

    pub fn run(
        &self,
        prompt: &str,
        active_file: Option<PathBuf>,
        selected_text: Option<String>,
    ) -> Vec<AgentStep> {
        let mut steps = Vec::new();
        let ctx = self.context.build_context(self.mode.clone(), active_file, selected_text);
        let context_prompt = self.context.format_context_prompt(&ctx);

        steps.push(AgentStep {
            action: AgentAction::Think {
                summary: format!(
                    "Received request: {}\n\nContext:\n{}",
                    prompt, context_prompt
                ),
            },
            visible_to_user: true,
        });

        steps
    }
}

// ── Native agent backend ───────────────────────────────────────────────

pub struct NativeAgentBackend {
    tools: Arc<ToolRegistry>,
}

impl NativeAgentBackend {
    pub fn new(tools: Arc<ToolRegistry>) -> Self {
        Self { tools }
    }

    #[allow(unused)]
    fn resolve_tool(&self, action: &AgentAction) -> Option<ToolCall> {
        match action {
            AgentAction::ReadFile { path } => Some(ToolCall {
                tool: "read_file".to_string(),
                args: HashMap::from([("path".to_string(), path.clone())]),
            }),
            AgentAction::WriteFile { path } => Some(ToolCall {
                tool: "write_file".to_string(),
                args: HashMap::from([("path".to_string(), path.clone())]),
            }),
            AgentAction::RunTerminal { command } => Some(ToolCall {
                tool: "run_terminal".to_string(),
                args: HashMap::from([("command".to_string(), command.clone())]),
            }),
            AgentAction::SearchWorkspace { query } => Some(ToolCall {
                tool: "search_workspace".to_string(),
                args: HashMap::from([("pattern".to_string(), query.clone())]),
            }),
            _ => None,
        }
    }
}

impl AgentBackend for NativeAgentBackend {
    fn id(&self) -> &str {
        "super-native"
    }

    fn kind(&self) -> AgentBackendKind {
        AgentBackendKind::Native
    }

    fn start<'a>(&'a self, request: AgentRequest) -> superide_sdk::SdkFuture<'a, AgentResult<()>> {
        Box::pin(async move {
            let tool_specs = self.tools.all_specs();
            tracing::info!(
                "Native agent started (mode={:?}, tools={})",
                request.context.mode,
                tool_specs.len()
            );
            Ok(())
        })
    }

    fn next_step<'a>(&'a self) -> superide_sdk::SdkFuture<'a, AgentResult<AgentStep>> {
        Box::pin(async move {
            Err(AgentError {
                agent_id: "super-native".to_string(),
                message: "Not yet implemented: agent planning loop".to_string(),
            })
        })
    }

    fn stop<'a>(&'a self) -> superide_sdk::SdkFuture<'a, AgentResult<()>> {
        Box::pin(async move { Ok(()) })
    }
}
