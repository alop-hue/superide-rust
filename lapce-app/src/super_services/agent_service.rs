use std::collections::HashMap;
use std::path::PathBuf;

use parking_lot::RwLock;

use superide_agent::context_manager::ContextManager;
use superide_agent::tool_registry::{ToolRegistry, ToolResult};
use superide_agent::tools::file_tools::{EditFileTool, ReadFileTool, SearchWorkspaceTool, WriteFileTool};
use superide_agent::tools::terminal_tools::RunTerminalTool;
use superide_agent::tools::web_search::WebSearchTool;
use superide_sdk::agent::AgentMode;

pub struct AgentService {
    mode: RwLock<Option<AgentMode>>,
    workspace_root: RwLock<Option<PathBuf>>,
    context_manager: RwLock<Option<ContextManager>>,
    tool_registry: RwLock<Option<ToolRegistry>>,
}

impl Default for AgentService {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentService {
    pub fn new() -> Self {
        Self {
            mode: RwLock::new(None),
            workspace_root: RwLock::new(None),
            context_manager: RwLock::new(None),
            tool_registry: RwLock::new(None),
        }
    }

    pub fn initialize(&self, workspace_root: Option<PathBuf>) {
        let mut registry = ToolRegistry::new();
        registry.register(Box::new(ReadFileTool::default()));
        registry.register(Box::new(WriteFileTool::default()));
        registry.register(Box::new(EditFileTool::default()));
        registry.register(Box::new(WebSearchTool::default()));
        registry.register(Box::new(RunTerminalTool::default()));
        if let Some(ref root) = workspace_root {
            registry.register(Box::new(SearchWorkspaceTool::new(root.clone())));
        }

        *self.tool_registry.write() = Some(registry);
        *self.workspace_root.write() = workspace_root.clone();
        *self.context_manager.write() = Some(ContextManager::new(workspace_root));
    }

    pub fn set_mode(&self, mode: AgentMode) {
        *self.mode.write() = Some(mode);
    }

    pub fn tool_specs(&self) -> Vec<superide_agent::tool_registry::ToolSpec> {
        self.tool_registry
            .read()
            .as_ref()
            .map(|r| {
                r.all_specs()
                    .into_iter()
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn execute_tool(
        &self,
        tool: &str,
        args: &HashMap<String, String>,
    ) -> ToolResult {
        match self.tool_registry.read().as_ref() {
            Some(registry) => {
                let call = superide_agent::tool_registry::ToolCall {
                    tool: tool.to_string(),
                    args: args.clone(),
                };
                registry.execute(&call)
            }
            None => ToolResult::Error {
                message: "Agent service not initialized".to_string(),
            },
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.tool_registry.read().is_some()
    }
}
