use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::tool_registry::{Tool, ToolParam, ToolResult, ToolSpec};

// ── Read File ─────────────────────────────────────────────────────────

pub struct ReadFileTool {
    spec: ToolSpec,
}

impl Default for ReadFileTool {
    fn default() -> Self {
        Self {
            spec: ToolSpec {
                name: "read_file".to_string(),
                description: "Read the contents of a file from the filesystem".to_string(),
                parameters: vec![ToolParam {
                    name: "path".to_string(),
                    param_type: "string".to_string(),
                    description: "Path to the file".to_string(),
                    required: true,
                }],
                requires_approval: false,
            },
        }
    }
}

impl Tool for ReadFileTool {
    fn spec(&self) -> &ToolSpec {
        &self.spec
    }

    fn execute(&self, args: &HashMap<String, String>) -> ToolResult {
        let path = match args.get("path") {
            Some(p) => PathBuf::from(p),
            None => {
                return ToolResult::Error {
                    message: "Missing 'path' argument".to_string(),
                }
            }
        };

        match fs::read_to_string(&path) {
            Ok(content) => {
                let lines = content.lines().count();
                let preview: String = content.lines().take(50).collect::<Vec<_>>().join("\n");
                let truncated = if lines > 50 {
                    format!(
                        "{}\n\n... ({} more lines, {} total)",
                        preview,
                        lines - 50,
                        lines
                    )
                } else {
                    content
                };
                ToolResult::Success {
                    output: format!("File: {}\n```\n{}\n```", path.display(), truncated),
                }
            }
            Err(e) => ToolResult::Error {
                message: format!("Failed to read '{}': {}", path.display(), e),
            },
        }
    }
}

// ── Write File ─────────────────────────────────────────────────────────

pub struct WriteFileTool {
    spec: ToolSpec,
}

impl Default for WriteFileTool {
    fn default() -> Self {
        Self {
            spec: ToolSpec {
                name: "write_file".to_string(),
                description: "Write content to a file. Creates parent directories if needed."
                    .to_string(),
                parameters: vec![
                    ToolParam {
                        name: "path".to_string(),
                        param_type: "string".to_string(),
                        description: "Path to the file".to_string(),
                        required: true,
                    },
                    ToolParam {
                        name: "content".to_string(),
                        param_type: "string".to_string(),
                        description: "Content to write".to_string(),
                        required: true,
                    },
                ],
                requires_approval: true,
            },
        }
    }
}

impl Tool for WriteFileTool {
    fn spec(&self) -> &ToolSpec {
        &self.spec
    }

    fn execute(&self, args: &HashMap<String, String>) -> ToolResult {
        let path = match args.get("path") {
            Some(p) => PathBuf::from(p),
            None => {
                return ToolResult::Error {
                    message: "Missing 'path' argument".to_string(),
                }
            }
        };
        let content = match args.get("content") {
            Some(c) => c,
            None => {
                return ToolResult::Error {
                    message: "Missing 'content' argument".to_string(),
                }
            }
        };

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    return ToolResult::Error {
                        message: format!("Failed to create directories: {}", e),
                    };
                }
            }
        }

        match fs::write(&path, content) {
            Ok(_) => ToolResult::Success {
                output: format!("Wrote {} bytes to {}", content.len(), path.display()),
            },
            Err(e) => ToolResult::Error {
                message: format!("Failed to write '{}': {}", path.display(), e),
            },
        }
    }
}

// ── Edit File ──────────────────────────────────────────────────────────

pub struct EditFileTool {
    spec: ToolSpec,
}

impl Default for EditFileTool {
    fn default() -> Self {
        Self {
            spec: ToolSpec {
                name: "edit_file".to_string(),
                description: "Apply a search-and-replace edit to a file".to_string(),
                parameters: vec![
                    ToolParam {
                        name: "path".to_string(),
                        param_type: "string".to_string(),
                        description: "Path to the file".to_string(),
                        required: true,
                    },
                    ToolParam {
                        name: "old_string".to_string(),
                        param_type: "string".to_string(),
                        description: "Text to search for (must match exactly)".to_string(),
                        required: true,
                    },
                    ToolParam {
                        name: "new_string".to_string(),
                        param_type: "string".to_string(),
                        description: "Replacement text".to_string(),
                        required: true,
                    },
                ],
                requires_approval: true,
            },
        }
    }
}

impl Tool for EditFileTool {
    fn spec(&self) -> &ToolSpec {
        &self.spec
    }

    fn execute(&self, args: &HashMap<String, String>) -> ToolResult {
        let path = match args.get("path") {
            Some(p) => PathBuf::from(p),
            None => {
                return ToolResult::Error {
                    message: "Missing 'path' argument".to_string(),
                }
            }
        };
        let old = match args.get("old_string") {
            Some(s) => s,
            None => {
                return ToolResult::Error {
                    message: "Missing 'old_string' argument".to_string(),
                }
            }
        };
        let new = match args.get("new_string") {
            Some(s) => s,
            None => {
                return ToolResult::Error {
                    message: "Missing 'new_string' argument".to_string(),
                }
            }
        };

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                return ToolResult::Error {
                    message: format!("Failed to read '{}': {}", path.display(), e),
                }
            }
        };

        if !content.contains(old) {
            return ToolResult::Error {
                message: format!("Could not find '{}' in {}", old, path.display()),
            };
        }

        let new_content = content.replace(old, new);
        match fs::write(&path, &new_content) {
            Ok(_) => ToolResult::Success {
                output: format!(
                    "Edited {}: replaced {} occurrence(s)",
                    path.display(),
                    content.matches(old).count()
                ),
            },
            Err(e) => ToolResult::Error {
                message: format!("Failed to write '{}': {}", path.display(), e),
            },
        }
    }
}

// ── Search Workspace ───────────────────────────────────────────────────

pub struct SearchWorkspaceTool {
    spec: ToolSpec,
    workspace_root: PathBuf,
}

impl SearchWorkspaceTool {
    pub fn new(workspace_root: PathBuf) -> Self {
        Self {
            workspace_root,
            spec: ToolSpec {
                name: "search_workspace".to_string(),
                description: "Search for text in workspace files using regex".to_string(),
                parameters: vec![
                    ToolParam {
                        name: "pattern".to_string(),
                        param_type: "string".to_string(),
                        description: "Regex pattern to search for".to_string(),
                        required: true,
                    },
                    ToolParam {
                        name: "glob".to_string(),
                        param_type: "string".to_string(),
                        description: "Optional file glob filter (e.g. '*.rs')".to_string(),
                        required: false,
                    },
                ],
                requires_approval: false,
            },
        }
    }
}

impl Tool for SearchWorkspaceTool {
    fn spec(&self) -> &ToolSpec {
        &self.spec
    }

    fn execute(&self, args: &HashMap<String, String>) -> ToolResult {
        let pattern = match args.get("pattern") {
            Some(p) => p,
            None => {
                return ToolResult::Error {
                    message: "Missing 'pattern' argument".to_string(),
                }
            }
        };

        let glob_filter = args.get("glob").map(|s| s.as_str());

        if !self.workspace_root.exists() {
            return ToolResult::Error {
                message: "Workspace root does not exist".to_string(),
            };
        }

        let walker = ignore::WalkBuilder::new(&self.workspace_root)
            .hidden(false)
            .parents(false)
            .require_git(false)
            .build();

        let mut results: Vec<(String, usize, String)> = Vec::new();
        let re = match regex::Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                return ToolResult::Error {
                    message: format!("Invalid regex '{}': {}", pattern, e),
                }
            }
        };

        for entry in walker.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            if let Some(glob) = glob_filter {
                if !glob_match(glob, path) {
                    continue;
                }
            }

            let content = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            for (lineno, line) in content.lines().enumerate() {
                if re.is_match(line) {
                    results.push((
                        path.strip_prefix(&self.workspace_root)
                            .map(|p| p.display().to_string())
                            .unwrap_or_else(|_| path.display().to_string()),
                        lineno + 1,
                        line.trim().to_string(),
                    ));
                    if results.len() >= 50 {
                        break;
                    }
                }
            }
            if results.len() >= 50 {
                break;
            }
        }

        if results.is_empty() {
            ToolResult::Success {
                output: format!("No matches found for '{}'", pattern),
            }
        } else {
            let output = results
                .iter()
                .map(|(f, l, c)| format!("{}:{}  {}", f, l, c))
                .collect::<Vec<_>>()
                .join("\n");
            ToolResult::Success {
                output: format!(
                    "Found {} result(s) for '{}':\n{}",
                    results.len(),
                    pattern,
                    output
                ),
            }
        }
    }
}

fn glob_match(glob: &str, path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|name| {
            let re_str = glob
                .replace('.', "\\.")
                .replace('*', ".*")
                .replace('?', ".");
            regex::Regex::new(&format!("^{}$", re_str))
                .map(|re| re.is_match(name))
                .unwrap_or(true)
        })
        .unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_not_found() {
        let tool = ReadFileTool::default();
        let result = tool.execute(&HashMap::from([("path".to_string(), "/nonexistent/file.txt".to_string())]));
        match result {
            ToolResult::Error { message } => assert!(message.contains("Failed to read")),
            _ => panic!("Expected Error result"),
        }
    }

    #[test]
    fn test_read_file_missing_args() {
        let tool = ReadFileTool::default();
        let result = tool.execute(&HashMap::new());
        match result {
            ToolResult::Error { message } => assert_eq!(message, "Missing 'path' argument"),
            _ => panic!("Expected Error result"),
        }
    }

    #[test]
    fn test_write_file_and_read_back() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test_write.txt");

        let write = WriteFileTool::default();
        let result = write.execute(&HashMap::from([
            ("path".to_string(), path.to_string_lossy().to_string()),
            ("content".to_string(), "hello world".to_string()),
        ]));
        match result {
            ToolResult::Success { output } => assert!(output.contains("Wrote 11 bytes")),
            _ => panic!("Expected Success, got {:?}", result),
        }

        let read = ReadFileTool::default();
        let result = read.execute(&HashMap::from([
            ("path".to_string(), path.to_string_lossy().to_string()),
        ]));
        match result {
            ToolResult::Success { output } => assert!(output.contains("hello world")),
            _ => panic!("Expected Success, got {:?}", result),
        }
    }

    #[test]
    fn test_edit_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test_edit.rs");
        fs::write(&path, "fn old_name() {}").unwrap();

        let edit = EditFileTool::default();
        let result = edit.execute(&HashMap::from([
            ("path".to_string(), path.to_string_lossy().to_string()),
            ("old_string".to_string(), "old_name".to_string()),
            ("new_string".to_string(), "new_name".to_string()),
        ]));
        match result {
            ToolResult::Success { .. } => {}
            _ => panic!("Expected Success, got {:?}", result),
        }

        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "fn new_name() {}");
    }

    #[test]
    fn test_edit_file_not_found() {
        let edit = EditFileTool::default();
        let result = edit.execute(&HashMap::from([
            ("path".to_string(), "/nonexistent.rs".to_string()),
            ("old_string".to_string(), "old".to_string()),
            ("new_string".to_string(), "new".to_string()),
        ]));
        match result {
            ToolResult::Error { message } => assert!(message.contains("Failed to read")),
            _ => panic!("Expected Error, got {:?}", result),
        }
    }

    #[test]
    fn test_edit_file_pattern_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test_no_match.rs");
        fs::write(&path, "content").unwrap();

        let edit = EditFileTool::default();
        let result = edit.execute(&HashMap::from([
            ("path".to_string(), path.to_string_lossy().to_string()),
            ("old_string".to_string(), "does_not_exist".to_string()),
            ("new_string".to_string(), "replacement".to_string()),
        ]));
        match result {
            ToolResult::Error { message } => assert!(message.contains("Could not find")),
            _ => panic!("Expected Error, got {:?}", result),
        }
    }

    #[test]
    fn test_tool_registry() {
        use crate::tool_registry::{ToolCall, ToolRegistry};
        let mut reg = ToolRegistry::new();
        reg.register(Box::new(ReadFileTool::default()));

        let specs = reg.all_specs();
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].name, "read_file");

        let result = reg.execute(&ToolCall {
            tool: "read_file".to_string(),
            args: HashMap::new(),
        });
        match result {
            ToolResult::Error { message } => assert_eq!(message, "Missing 'path' argument"),
            _ => panic!("Expected Error"),
        }

        let result = reg.execute(&ToolCall {
            tool: "unknown_tool".to_string(),
            args: HashMap::new(),
        });
        match result {
            ToolResult::Error { message } => assert!(message.contains("Unknown tool")),
            _ => panic!("Expected Error"),
        }
    }

    #[test]
    fn test_glob_match() {
        assert!(glob_match("*.rs", Path::new("main.rs")));
        assert!(!glob_match("*.rs", Path::new("main.py")));
        assert!(glob_match("test?.txt", Path::new("test1.txt")));
        assert!(!glob_match("test?.txt", Path::new("test12.txt")));
        assert!(glob_match("*.{ts,tsx}", Path::new("component.tsx")));
    }
}

