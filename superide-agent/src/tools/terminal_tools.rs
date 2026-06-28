use std::collections::HashMap;
use std::process::Command;
use std::time::Instant;

use crate::tool_registry::{Tool, ToolParam, ToolResult, ToolSpec};

pub struct RunTerminalTool {
    spec: ToolSpec,
}

impl Default for RunTerminalTool {
    fn default() -> Self {
        Self {
            spec: ToolSpec {
                name: "run_terminal".to_string(),
                description: "Execute a shell command and return its output. Supports piping, chaining, and most bash built-ins.".to_string(),
                parameters: vec![
                    ToolParam {
                        name: "command".to_string(),
                        param_type: "string".to_string(),
                        description: "The shell command to execute".to_string(),
                        required: true,
                    },
                    ToolParam {
                        name: "workdir".to_string(),
                        param_type: "string".to_string(),
                        description: "Working directory for the command (default: current dir)".to_string(),
                        required: false,
                    },
                    ToolParam {
                        name: "timeout".to_string(),
                        param_type: "string".to_string(),
                        description: "Timeout in seconds (default: 30)".to_string(),
                        required: false,
                    },
                ],
                requires_approval: true,
            },
        }
    }
}

impl Tool for RunTerminalTool {
    fn spec(&self) -> &ToolSpec {
        &self.spec
    }

    fn execute(&self, args: &HashMap<String, String>) -> ToolResult {
        let command = match args.get("command") {
            Some(c) => c,
            None => {
                return ToolResult::Error {
                    message: "Missing 'command' argument".to_string(),
                }
            }
        };

        let workdir = args.get("workdir").map(|s| s.as_str());
        let timeout_secs = args
            .get("timeout")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(30);

        let output = run_command(command, workdir, timeout_secs);
        match output {
            Ok(out) => ToolResult::Success { output: out },
            Err(e) => ToolResult::Error { message: e },
        }
    }
}

fn run_command(cmd: &str, workdir: Option<&str>, timeout_secs: u64) -> Result<String, String> {
    let shell = if cfg!(target_os = "windows") {
        "cmd.exe"
    } else {
        "sh"
    };
    let flag = if cfg!(target_os = "windows") { "/C" } else { "-c" };

    let mut child = Command::new(shell)
        .arg(flag)
        .arg(cmd)
        .current_dir(workdir.unwrap_or("."))
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn command: {}", e))?;

    let _start = Instant::now();
    const POLL_MS: u64 = 50;
    let max_polls = (timeout_secs * 1000) / POLL_MS;

    for _ in 0..max_polls {
        match child.try_wait() {
            Ok(Some(status)) => {
                let stdout = child.stdout.take().map(|o| {
                    use std::io::Read;
                    let mut buf = String::new();
                    std::io::BufReader::new(o).read_to_string(&mut buf).ok();
                    buf
                }).unwrap_or_default();

                let stderr = child.stderr.take().map(|e| {
                    use std::io::Read;
                    let mut buf = String::new();
                    std::io::BufReader::new(e).read_to_string(&mut buf).ok();
                    buf
                }).unwrap_or_default();

                let mut output = String::new();
                if !stdout.is_empty() {
                    output.push_str(&stdout);
                }
                if !stderr.is_empty() {
                    if !output.is_empty() {
                        output.push('\n');
                    }
                    output.push_str(&format!("[stderr]\n{}", stderr));
                }

                return if status.success() {
                    if output.is_empty() {
                        output = "Command completed successfully (no output)".to_string();
                    }
                    Ok(output)
                } else {
                    let code = status.code().unwrap_or(-1);
                    Err(format!(
                        "Command exited with code {}:\n{}",
                        code,
                        if output.is_empty() { "(no output)" } else { &output }
                    ))
                };
            }
            Ok(None) => {
                std::thread::sleep(std::time::Duration::from_millis(POLL_MS));
            }
            Err(e) => {
                return Err(format!("Failed to wait for command: {}", e));
            }
        }
    }

    // Timeout reached — kill the process
    let _ = child.kill();
    let _ = child.wait();
    Err(format!(
        "Command timed out after {} seconds:\n{}",
        timeout_secs, cmd
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_tool_spec() {
        let tool = RunTerminalTool::default();
        assert_eq!(tool.spec().name, "run_terminal");
        assert!(tool.spec().requires_approval);
        assert!(!tool.spec().parameters.is_empty());
    }

    #[test]
    fn test_terminal_missing_command() {
        let tool = RunTerminalTool::default();
        let args = HashMap::new();
        match tool.execute(&args) {
            ToolResult::Error { message } => assert!(message.contains("command")),
            _ => panic!("Expected error"),
        }
    }

    #[test]
    fn test_terminal_echo() {
        let tool = RunTerminalTool::default();
        let mut args = HashMap::new();
        args.insert("command".to_string(), "echo hello_world".to_string());
        match tool.execute(&args) {
            ToolResult::Success { output } => assert!(output.contains("hello_world")),
            ToolResult::Error { message } => panic!("Unexpected error: {}", message),
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_terminal_with_workdir() {
        let tool = RunTerminalTool::default();
        let mut args = HashMap::new();
        args.insert("command".to_string(), "pwd".to_string());
        args.insert("workdir".to_string(), "/tmp".to_string());
        match tool.execute(&args) {
            ToolResult::Success { output } => assert!(output.contains("/tmp")),
            ToolResult::Error { message } => panic!("Unexpected error: {}", message),
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_terminal_timeout() {
        let tool = RunTerminalTool::default();
        let mut args = HashMap::new();
        args.insert("command".to_string(), "sleep 10".to_string());
        args.insert("timeout".to_string(), "2".to_string());
        match tool.execute(&args) {
            ToolResult::Error { message } => assert!(message.contains("timed out")),
            _ => panic!("Expected timeout error"),
        }
    }

    #[test]
    fn test_terminal_pipe() {
        let tool = RunTerminalTool::default();
        let mut args = HashMap::new();
        args.insert("command".to_string(), "echo foo bar baz | wc -w".to_string());
        match tool.execute(&args) {
            ToolResult::Success { output } => assert!(output.trim().contains("3") || output.trim().starts_with("3")),
            ToolResult::Error { message } => panic!("Unexpected error: {}", message),
            _ => panic!("Unexpected result"),
        }
    }
}
