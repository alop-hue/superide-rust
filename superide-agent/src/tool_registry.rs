use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ToolResult {
    Success { output: String },
    Error { message: String },
    NeedsApproval { action: String },
    Done { summary: String },
}

#[derive(Debug, Clone)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ToolParam>,
    pub requires_approval: bool,
}

#[derive(Debug, Clone)]
pub struct ToolParam {
    pub name: String,
    pub param_type: String,
    pub description: String,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub struct ToolCall {
    pub tool: String,
    pub args: HashMap<String, String>,
}

pub trait Tool: Send + Sync {
    fn spec(&self) -> &ToolSpec;
    fn execute(&self, args: &HashMap<String, String>) -> ToolResult;
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        let name = tool.spec().name.clone();
        self.tools.insert(name, tool);
    }

    pub fn get(&self, name: &str) -> Option<&dyn Tool> {
        self.tools.get(name).map(|t| t.as_ref())
    }

    pub fn all_specs(&self) -> Vec<&ToolSpec> {
        self.tools.values().map(|t| t.spec()).collect()
    }

    pub fn execute(&self, call: &ToolCall) -> ToolResult {
        match self.get(&call.tool) {
            Some(tool) => tool.execute(&call.args),
            None => ToolResult::Error {
                message: format!("Unknown tool: {}", call.tool),
            },
        }
    }
}
