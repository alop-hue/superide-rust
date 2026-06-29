use std::collections::HashMap;

use crate::tool_registry::{Tool, ToolParam, ToolResult, ToolSpec};

pub struct WebSearchTool {
    spec: ToolSpec,
}

impl Default for WebSearchTool {
    fn default() -> Self {
        Self {
            spec: ToolSpec {
                name: "web_search".to_string(),
                description: "Search the web using DuckDuckGo. Returns relevant snippets and URLs.".to_string(),
                parameters: vec![ToolParam {
                    name: "query".to_string(),
                    param_type: "string".to_string(),
                    description: "The search query".to_string(),
                    required: true,
                }],
                requires_approval: false,
            },
        }
    }
}

impl Tool for WebSearchTool {
    fn spec(&self) -> &ToolSpec {
        &self.spec
    }

    fn execute(&self, args: &HashMap<String, String>) -> ToolResult {
        let query = match args.get("query") {
            Some(q) => q,
            None => {
                return ToolResult::Error {
                    message: "Missing 'query' argument".to_string(),
                }
            }
        };

        match search_duckduckgo(query) {
            Ok(results) => ToolResult::Success { output: results },
            Err(e) => ToolResult::Error { message: e },
        }
    }
}

fn search_duckduckgo(query: &str) -> Result<String, String> {
    let url = format!(
        "https://api.duckduckgo.com/?q={}&format=json&no_html=1&skip_disambig=1",
        urlencoding(query)
    );

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .user_agent("SUPER IDE/0.1 (research agent)")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let resp = client
        .get(&url)
        .send()
        .map_err(|e| format!("DuckDuckGo request failed: {}", e))?;

    let status = resp.status();
    let body = resp
        .text()
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    if !status.is_success() {
        return Err(format!("DuckDuckGo returned status {}: {}", status, body));
    }

    let parsed: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse DuckDuckGo response: {}", e))?;

    let mut output = String::new();

    // Extract abstract text
    if let Some(abstract_text) = parsed["AbstractText"].as_str() {
        if !abstract_text.is_empty() {
            output.push_str("## Summary\n");
            output.push_str(abstract_text);
            output.push('\n');
            if let Some(source) = parsed["AbstractSource"].as_str() {
                output.push_str(&format!("\nSource: {}\n", source));
            }
            if let Some(url_str) = parsed["AbstractURL"].as_str() {
                if !url_str.is_empty() {
                    output.push_str(&format!("URL: {}\n", url_str));
                }
            }
            output.push('\n');
        }
    }

    // Extract answer
    if let Some(answer) = parsed["Answer"].as_str() {
        if !answer.is_empty() {
            output.push_str("## Answer\n");
            output.push_str(answer);
            output.push('\n');
            if let Some(url_str) = parsed["AnswerURL"].as_str() {
                if !url_str.is_empty() {
                    output.push_str(&format!("URL: {}\n", url_str));
                }
            }
            output.push('\n');
        }
    }

    // Extract related topics / results
    if let Some(results) = parsed["RelatedTopics"].as_array() {
        if !results.is_empty() {
            output.push_str("## Related Results\n");
            for (i, result) in results.iter().enumerate() {
                if let Some(text) = result["Text"].as_str() {
                    let title = if let Some(first_url) = result["FirstURL"].as_str() {
                        format!("{}. {} ({})", i + 1, text, first_url)
                    } else {
                        format!("{}. {}", i + 1, text)
                    };
                    output.push_str(&title);
                    output.push('\n');
                }
                // Handle topics with sub-items
                if let Some(topics) = result["Topics"].as_array() {
                    for topic in topics {
                        if let Some(text) = topic["Text"].as_str() {
                            if let Some(url_str) = topic["FirstURL"].as_str() {
                                output.push_str(&format!("  - {} ({})\n", text, url_str));
                            }
                        }
                    }
                }
                if i > 20 {
                    output.push_str("... (results truncated)\n");
                    break;
                }
            }
        }
    }

    // Check for no results
    if parsed["AbstractText"].as_str().map_or(true, |s| s.is_empty())
        && parsed["Answer"].as_str().map_or(true, |s| s.is_empty())
        && parsed["RelatedTopics"]
            .as_array()
            .map_or(true, |a| a.is_empty())
    {
        // Try suggesting a direct web search
        output.push_str("No instant results found. Try a more specific query.");
    }

    Ok(output)
}

fn urlencoding(query: &str) -> String {
    let mut encoded = String::new();
    for byte in query.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            b' ' => encoded.push_str("+"),
            _ => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encoding() {
        assert_eq!(urlencoding("hello world"), "hello+world");
        assert_eq!(urlencoding("rust lang"), "rust+lang");
        assert_eq!(urlencoding("a/b"), "a%2Fb");
    }

    #[test]
    fn test_web_search_spec() {
        let tool = WebSearchTool::default();
        assert_eq!(tool.spec().name, "web_search");
        assert!(!tool.spec().parameters.is_empty());
        assert_eq!(tool.spec().parameters[0].name, "query");
    }

    #[test]
    fn test_web_search_missing_query() {
        let tool = WebSearchTool::default();
        let args = HashMap::new();
        match tool.execute(&args) {
            ToolResult::Error { message } => {
                assert!(message.contains("query"));
            }
            _ => panic!("Expected error for missing query"),
        }
    }

    #[test]
    fn test_web_search_network() {
        let tool = WebSearchTool::default();
        let mut args = HashMap::new();
        args.insert("query".to_string(), "Rust programming language".to_string());
        let result = tool.execute(&args);
        match result {
            ToolResult::Success { output } => {
                assert!(!output.is_empty());
                assert!(output.contains("Rust") || output.contains("rust"));
            }
            ToolResult::Error { message } => {
                // Network might not be available in test environment
                eprintln!("Network error (expected if offline): {}", message);
            }
            _ => panic!("Unexpected result"),
        }
    }
}
