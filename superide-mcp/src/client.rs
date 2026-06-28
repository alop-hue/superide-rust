use std::collections::HashMap;

use crate::protocol::{
    InitializeResult, JsonRpcMessage, JsonRpcResponse, McpTool, McpToolResult, build_request,
};
use crate::transport::{McpTransport, StdioTransport};

/// An MCP client connected to a server.
pub struct McpClient {
    transport: McpTransport,
    server_info: Option<InitializeResult>,
    #[allow(dead_code)]
    pending: HashMap<u64, std::sync::mpsc::Sender<JsonRpcResponse>>,
    #[allow(dead_code)]
    next_id: u64,
}

impl McpClient {
    /// Connect to an MCP server via stdio.
    pub fn connect_stdio(program: &str, args: &[&str]) -> Result<Self, String> {
        let transport = StdioTransport::spawn(program, args)?;
        Ok(Self {
            transport: McpTransport::Stdio(transport),
            server_info: None,
            pending: HashMap::new(),
            next_id: 1,
        })
    }

    /// Initialize the MCP session (required before any other operation).
    pub fn initialize(&mut self) -> Result<InitializeResult, String> {
        let req = build_request(
            "initialize",
            Some(serde_json::json!({
                "protocolVersion": "2025-03-26",
                "capabilities": {},
                "clientInfo": {
                    "name": "super-ide",
                    "version": "0.1.0"
                }
            })),
        );

        self.send_request(&req)?;

        let resp = self.wait_response(req.id)?;
        let result: InitializeResult = serde_json::from_value(
            resp.result
                .clone()
                .ok_or_else(|| "No result in initialize response".to_string())?,
        )
        .map_err(|e| format!("Failed to parse initialize result: {}", e))?;

        // Send initialized notification
        let initialized = crate::protocol::JsonRpcNotification {
            jsonrpc: "2.0".to_string(),
            method: "notifications/initialized".to_string(),
            params: None,
        };
        self.send_message(&JsonRpcMessage::Notification(initialized))?;

        self.server_info = Some(result.clone());
        Ok(result)
    }

    /// List available tools from the server.
    pub fn list_tools(&mut self) -> Result<Vec<McpTool>, String> {
        let req = build_request("tools/list", None);
        self.send_request(&req)?;
        let resp = self.wait_response(req.id)?;
        let value = resp
            .result
            .clone()
            .ok_or_else(|| "No result in tools/list response".to_string())?;
        let tools: Vec<McpTool> = value["tools"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|v| serde_json::from_value(v.clone()).unwrap_or(McpTool {
                        name: v["name"].as_str().unwrap_or("unknown").to_string(),
                        description: v["description"]
                            .as_str()
                            .unwrap_or("")
                            .to_string(),
                        input_schema: v["inputSchema"].clone(),
                    }))
                    .collect()
            })
            .unwrap_or_default();
        Ok(tools)
    }

    /// Call a tool on the server.
    pub fn call_tool(
        &mut self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<McpToolResult, String> {
        let req = build_request(
            "tools/call",
            Some(serde_json::json!({
                "name": name,
                "arguments": args,
            })),
        );
        self.send_request(&req)?;
        let resp = self.wait_response(req.id)?;
        let value = resp
            .result
            .clone()
            .ok_or_else(|| "No result in tools/call response".to_string())?;
        let result: McpToolResult = serde_json::from_value(value)
            .map_err(|e| format!("Failed to parse tool result: {}", e))?;
        Ok(result)
    }

    // ── Internal helpers ─────────────────────────────────────────────

    fn send_message(&mut self, msg: &JsonRpcMessage) -> Result<(), String> {
        match &mut self.transport {
            McpTransport::Stdio(t) => t.send(msg),
        }
    }

    fn send_request(&mut self, req: &crate::protocol::JsonRpcRequest) -> Result<(), String> {
        self.send_message(&JsonRpcMessage::Request(req.clone()))
    }

    fn wait_response(&mut self, id: u64) -> Result<JsonRpcResponse, String> {
        loop {
            let msg = match &mut self.transport {
                McpTransport::Stdio(t) => t.recv()?,
            };
            match msg {
                Some(JsonRpcMessage::Response(resp)) => {
                    if resp.id == id {
                        if let Some(ref err) = resp.error {
                            return Err(err.to_string());
                        }
                        return Ok(resp);
                    }
                }
                Some(JsonRpcMessage::Request(_)) | Some(JsonRpcMessage::Notification(_)) => {
                    // Ignore unsolicited messages for now
                }
                None => {
                    return Err("MCP server closed connection".to_string());
                }
            }
        }
    }
}

impl Drop for McpClient {
    fn drop(&mut self) {
        let _ = match &mut self.transport {
            McpTransport::Stdio(t) => t.close(),
        };
    }
}
