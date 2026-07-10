use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, Command, Stdio};

use crate::protocol::JsonRpcMessage;

pub enum McpTransport {
    Stdio(StdioTransport),
}

pub struct StdioTransport {
    child: Child,
}

impl StdioTransport {
    /// Spawn an MCP server process and connect via stdio.
    pub fn spawn(program: &str, args: &[&str]) -> Result<Self, String> {
        let child = Command::new(program)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| format!("Failed to spawn MCP server '{}': {}", program, e))?;
        Ok(Self { child })
    }

    /// Send a JSON-RPC message to the server's stdin.
    pub fn send(&mut self, msg: &JsonRpcMessage) -> Result<(), String> {
        let stdin = self
            .child
            .stdin
            .as_mut()
            .ok_or_else(|| "No stdin on MCP server process".to_string())?;

        let json = serde_json::to_string(msg)
            .map_err(|e| format!("Failed to serialize message: {}", e))?;

        let header = format!("Content-Length: {}\r\n\r\n", json.len());
        let raw = format!("{}{}", header, json);

        stdin
            .write_all(raw.as_bytes())
            .map_err(|e| format!("Failed to write to MCP server stdin: {}", e))?;
        stdin
            .flush()
            .map_err(|e| format!("Failed to flush MCP server stdin: {}", e))?;
        Ok(())
    }

    /// Receive a JSON-RPC message from the server's stdout.
    pub fn recv(&mut self) -> Result<Option<JsonRpcMessage>, String> {
        let stdout = self
            .child
            .stdout
            .as_mut()
            .ok_or_else(|| "No stdout on MCP server process".to_string())?;

        let mut reader = BufReader::new(stdout);
        let mut header = String::new();

        // Read headers until blank line
        loop {
            header.clear();
            match reader.read_line(&mut header) {
                Ok(0) => return Ok(None),
                Ok(_) => {
                    let trimmed = header.trim();
                    if trimmed.is_empty() {
                        break;
                    }
                }
                Err(e) => return Err(format!("Failed to read header: {}", e)),
            }
        }

        // Read content
        let mut content = String::new();
        reader
            .read_to_string(&mut content)
            .map_err(|e| format!("Failed to read content: {}", e))?;

        if content.is_empty() {
            return Ok(None);
        }

        let msg: JsonRpcMessage =
            serde_json::from_str(&content).map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(Some(msg))
    }

    /// Close the transport.
    pub fn close(&mut self) -> Result<(), String> {
        self.child
            .kill()
            .map_err(|e| format!("Failed to kill MCP server: {}", e))?;
        self.child
            .wait()
            .map_err(|e| format!("Failed to wait for MCP server: {}", e))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::*;

    /// Helper: create valid Content-Length framed bytes for a JSON value.
    fn frame(json: &serde_json::Value) -> Vec<u8> {
        let body = serde_json::to_string(json).unwrap();
        let header = format!("Content-Length: {}\r\n\r\n", body.len());
        format!("{}{}", header, body).into_bytes()
    }

    #[test]
    fn test_frame_construction() {
        let json = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": null
        });
        let bytes = frame(&json);
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.starts_with("Content-Length:"));
        assert!(text.contains("\r\n\r\n"));
        assert!(text.contains(r#""jsonrpc":"2.0""#));
    }

    #[test]
    fn test_content_length_matches_body() {
        let body = r#"{"jsonrpc":"2.0","id":1,"result":null}"#;
        let raw = format!("Content-Length: {}\r\n\r\n{}", body.len(), body);

        // Parse headers
        let mut lines = raw.lines();
        let header = lines.next().unwrap();
        let len: usize = header
            .trim_start_matches("Content-Length: ")
            .parse()
            .unwrap();
        assert_eq!(len, body.len());

        // Skip blank line
        let blank = lines.next().unwrap();
        assert!(blank.is_empty());

        // Read body
        let parsed_body = lines.next().unwrap();
        assert_eq!(parsed_body.len(), len);
    }

    #[test]
    fn test_parse_framed_response() {
        let resp = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": { "protocolVersion": "2025-03-26", "capabilities": {}, "serverInfo": { "name": "test", "version": "0.1.0" } }
        });
        let bytes = frame(&resp);

        // Parse: read header, skip blank, read body
        let text = String::from_utf8(bytes).unwrap();
        let parts: Vec<&str> = text.split("\r\n\r\n").collect();
        assert_eq!(parts.len(), 2);
        assert!(parts[0].starts_with("Content-Length:"));

        let parsed: JsonRpcMessage = serde_json::from_str(parts[1]).unwrap();
        match parsed {
            JsonRpcMessage::Response(r) => {
                assert_eq!(r.id, 1);
                assert!(r.result.is_some());
            }
            _ => panic!("Expected Response"),
        }
    }

    #[test]
    fn test_parse_initialized_notification() {
        let notif = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        });
        let bytes = frame(&notif);
        let text = String::from_utf8(bytes).unwrap();
        let parts: Vec<&str> = text.split("\r\n\r\n").collect();

        let parsed: JsonRpcMessage = serde_json::from_str(parts[1]).unwrap();
        match parsed {
            JsonRpcMessage::Notification(n) => {
                assert_eq!(n.method, "notifications/initialized");
            }
            _ => panic!("Expected Notification"),
        }
    }
}
