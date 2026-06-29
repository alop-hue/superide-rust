use std::path::PathBuf;

use superide_sdk::agent::{AgentContext, AgentMode};

#[derive(Debug, Clone)]
pub struct WorkspaceContext {
    pub workspace_root: Option<PathBuf>,
    pub active_file: Option<PathBuf>,
    pub selected_text: Option<String>,
    pub files_in_workspace: Vec<PathBuf>,
}

pub struct ContextManager {
    pub workspace_root: Option<PathBuf>,
}

impl ContextManager {
    pub fn new(workspace_root: Option<PathBuf>) -> Self {
        Self { workspace_root }
    }

    pub fn build_context(
        &self,
        _mode: AgentMode,
        active_file: Option<PathBuf>,
        selected_text: Option<String>,
    ) -> WorkspaceContext {
        let files = match &self.workspace_root {
            Some(root) if root.exists() => {
                let mut files = Vec::new();
                let walker = ignore::WalkBuilder::new(root)
                    .hidden(false)
                    .parents(false)
                    .require_git(false)
                    .build();
                for entry in walker.flatten() {
                    if entry.path().is_file() {
                        files.push(entry.path().to_path_buf());
                    }
                }
                files
            }
            _ => Vec::new(),
        };

        WorkspaceContext {
            workspace_root: self.workspace_root.clone(),
            active_file,
            selected_text,
            files_in_workspace: files,
        }
    }

    pub fn to_agent_context(&self, ctx: &WorkspaceContext, mode: AgentMode) -> AgentContext {
        AgentContext {
            workspace_root: ctx
                .workspace_root
                .as_ref()
                .map(|p| p.to_string_lossy().to_string()),
            active_file: ctx
                .active_file
                .as_ref()
                .map(|p| p.to_string_lossy().to_string()),
            selected_text: ctx.selected_text.clone(),
            provider_id: None,
            mode,
        }
    }

    pub fn format_context_prompt(&self, ctx: &WorkspaceContext) -> String {
        let mut parts = Vec::new();

        if let Some(root) = &ctx.workspace_root {
            parts.push(format!("Workspace root: {}", root.display()));
        }

        if let Some(file) = &ctx.active_file {
            parts.push(format!("Active file: {}", file.display()));
        }

        if let Some(text) = &ctx.selected_text {
            parts.push(format!(
                "Selected text:\n```\n{}\n```",
                text
            ));
        }

        if !ctx.files_in_workspace.is_empty() {
            let file_list: Vec<String> = ctx
                .files_in_workspace
                .iter()
                .take(30)
                .map(|p| {
                    ctx.workspace_root
                        .as_ref()
                        .and_then(|r| p.strip_prefix(r).ok())
                        .map(|s| s.display().to_string())
                        .unwrap_or_else(|| p.display().to_string())
                })
                .collect();
            let file_count = ctx.files_in_workspace.len();
            parts.push(format!(
                "Files in workspace (showing {} of {}):\n{}",
                file_list.len(),
                file_count,
                file_list.join("\n")
            ));
        }

        parts.join("\n\n")
    }
}
