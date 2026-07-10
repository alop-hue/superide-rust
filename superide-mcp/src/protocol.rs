// ── JSON-RPC message types ─────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum JsonRpcMessage {
    Request(JsonRpcRequest),
    Response(JsonRpcResponse),
    Notification(JsonRpcNotification),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl std::fmt::Display for JsonRpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MCP error {}: {}", self.code, self.message)
    }
}

// ── MCP protocol types ────────────────────────────────────────────────

/// Server capabilities advertised during initialization.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct McpCapabilities {
    #[serde(default)]
    pub tools: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<serde_json::Value>,
    #[serde(default)]
    pub prompts: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeResult {
    pub protocol_version: String,
    pub capabilities: McpCapabilities,
    #[serde(default)]
    pub server_info: ServerInfo,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self {
            name: "unknown".to_string(),
            version: "0.0.0".to_string(),
        }
    }
}

/// A tool exposed by the MCP server.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct McpTool {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub input_schema: serde_json::Value,
}

/// Result of calling a tool.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct McpToolResult {
    #[serde(default)]
    pub content: Vec<McpContentItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum McpContentItem {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "resource")]
    Resource { resource: serde_json::Value },
}

// ── Helper: build JSON-RPC requests ───────────────────────────────────

static NEXT_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

pub fn next_id() -> u64 {
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

pub fn build_request(method: &str, params: Option<serde_json::Value>) -> JsonRpcRequest {
    JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: next_id(),
        method: method.to_string(),
        params,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_rpc_request_roundtrip() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "initialize".to_string(),
            params: Some(serde_json::json!({"key": "value"})),
        };
        let json = serde_json::to_string(&req).unwrap();
        let parsed: JsonRpcRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, req);
    }

    #[test]
    fn test_json_rpc_request_no_params() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "ping".to_string(),
            params: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(!json.contains("params"));
    }

    #[test]
    fn test_json_rpc_response_with_result() {
        let resp = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: 1,
            result: Some(serde_json::json!({"ok": true})),
            error: None,
        };
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: JsonRpcResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, resp);
    }

    #[test]
    fn test_json_rpc_response_with_error() {
        let resp = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: 1,
            result: None,
            error: Some(JsonRpcError {
                code: -32601,
                message: "Method not found".to_string(),
                data: None,
            }),
        };
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: JsonRpcResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, resp);
        assert_eq!(parsed.error.unwrap().to_string(), "MCP error -32601: Method not found");
    }

    #[test]
    fn test_json_rpc_notification() {
        let notif = JsonRpcNotification {
            jsonrpc: "2.0".to_string(),
            method: "notifications/initialized".to_string(),
            params: None,
        };
        let json = serde_json::to_string(&notif).unwrap();
        let parsed: JsonRpcNotification = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, notif);
    }

    #[test]
    fn test_json_rpc_message_untagged() {
        // A request should parse as JsonRpcMessage::Request
        let req = r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#;
        let msg: JsonRpcMessage = serde_json::from_str(req).unwrap();
        match msg {
            JsonRpcMessage::Request(r) => assert_eq!(r.method, "ping"),
            _ => panic!("Expected Request variant"),
        }

        // A response should parse as JsonRpcMessage::Response
        let resp = r#"{"jsonrpc":"2.0","id":1,"result":null}"#;
        let msg: JsonRpcMessage = serde_json::from_str(resp).unwrap();
        match msg {
            JsonRpcMessage::Response(r) => assert_eq!(r.id, 1),
            _ => panic!("Expected Response variant"),
        }
    }

    #[test]
    fn test_build_request() {
        let req = build_request("tools/list", None);
        assert_eq!(req.jsonrpc, "2.0");
        assert_eq!(req.method, "tools/list");
        assert!(req.id > 0);
    }

    #[test]
    fn test_next_id_increments() {
        let a = next_id();
        let b = next_id();
        assert!(b > a);
    }

    #[test]
    fn test_initialize_result_roundtrip() {
        let init = InitializeResult {
            protocol_version: "2025-03-26".to_string(),
            capabilities: McpCapabilities {
                tools: Some(serde_json::json!({})),
                resources: None,
                prompts: None,
            },
            server_info: ServerInfo {
                name: "test-server".to_string(),
                version: "1.0.0".to_string(),
            },
        };
        let json = serde_json::to_string(&init).unwrap();
        let parsed: InitializeResult = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, init);
    }

    #[test]
    fn test_mcp_tool_roundtrip() {
        let tool = McpTool {
            name: "read_file".to_string(),
            description: "Read a file from disk".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" }
                }
            }),
        };
        let json = serde_json::to_string(&tool).unwrap();
        let parsed: McpTool = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, tool);
    }

    #[test]
    fn test_mcp_tool_result_text() {
        let result = McpToolResult {
            content: vec![McpContentItem::Text { text: "Hello".to_string() }],
            is_error: None,
        };
        let json = serde_json::to_string(&result).unwrap();
        let parsed: McpToolResult = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, result);
    }

    #[test]
    fn test_mcp_tool_result_error() {
        let result = McpToolResult {
            content: vec![],
            is_error: Some(true),
        };
        let json = serde_json::to_string(&result).unwrap();
        let parsed: McpToolResult = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_error.unwrap());
    }

    #[test]
    fn test_server_info_default() {
        let info = ServerInfo::default();
        assert_eq!(info.name, "unknown");
        assert_eq!(info.version, "0.0.0");
    }
}
