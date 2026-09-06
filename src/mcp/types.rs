//! Model Context Protocol (MCP 2024-11-05) core data types and JSON-RPC 2.0 primitives.
//!
//! Aligned with:
//! - Standard Model Context Protocol (2024-11-05)
//! - Google API Design Guide (Error Model & Input Validation)
//! - OpenCode Go Schema Safety Contract (strict root-level required array, no property required: true)

use serde::{Deserialize, Serialize};

pub const MCP_PROTOCOL_VERSION: &str = "2024-11-05";
pub const SERVER_NAME: &str = "mao_agent";
pub const SERVER_VERSION: &str = "0.1.0";

// ── Standard JSON-RPC 2.0 Structures ────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

impl JsonRpcRequest {
    #[must_use]
    pub fn is_notification(&self) -> bool {
        self.id.is_none()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    #[must_use]
    pub fn success(id: Option<serde_json::Value>, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    #[must_use]
    pub fn error(id: Option<serde_json::Value>, error: JsonRpcError) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(error),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl JsonRpcError {
    pub const PARSE_ERROR: i64 = -32700;
    pub const INVALID_REQUEST: i64 = -32600;
    pub const METHOD_NOT_FOUND: i64 = -32601;
    pub const INVALID_PARAMS: i64 = -32602;
    pub const INTERNAL_ERROR: i64 = -32603;
    /// Mapped from Google API / SRE Overload (`RESOURCE_EXHAUSTED`).
    pub const RESOURCE_EXHAUSTED: i64 = -32053;

    #[must_use]
    pub fn parse_error(detail: impl Into<String>) -> Self {
        Self {
            code: Self::PARSE_ERROR,
            message: detail.into(),
            data: None,
        }
    }

    #[must_use]
    pub fn invalid_request(detail: impl Into<String>) -> Self {
        Self {
            code: Self::INVALID_REQUEST,
            message: detail.into(),
            data: None,
        }
    }

    #[must_use]
    pub fn method_not_found(method: impl Into<String>) -> Self {
        Self {
            code: Self::METHOD_NOT_FOUND,
            message: format!("Method not found: {}", method.into()),
            data: None,
        }
    }

    #[must_use]
    pub fn invalid_params(detail: impl Into<String>) -> Self {
        Self {
            code: Self::INVALID_PARAMS,
            message: detail.into(),
            data: None,
        }
    }

    #[must_use]
    pub fn internal_error(detail: impl Into<String>) -> Self {
        Self {
            code: Self::INTERNAL_ERROR,
            message: detail.into(),
            data: None,
        }
    }

    #[must_use]
    pub fn resource_exhausted(detail: impl Into<String>) -> Self {
        Self {
            code: Self::RESOURCE_EXHAUSTED,
            message: detail.into(),
            data: None,
        }
    }
}

// ── MCP Protocol Payloads (initialize, tools/list, tools/call) ───────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct McpInitializeResult {
    pub protocol_version: String,
    pub capabilities: McpServerCapabilities,
    pub server_info: McpServerInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct McpServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<McpToolsCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct McpToolsCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct McpServerInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct McpToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct McpCallToolResult {
    pub content: Vec<McpContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

impl McpCallToolResult {
    #[must_use]
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            content: vec![McpContent::Text { text: text.into() }],
            is_error: None,
        }
    }

    #[must_use]
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            content: vec![McpContent::Text {
                text: message.into(),
            }],
            is_error: Some(true),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum McpContent {
    #[serde(rename = "text")]
    Text { text: String },
}

// ── Specific Tool Parameter Definitions ─────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct QueryDialecticalArgs {
    pub query: String,
    pub top_k: Option<usize>,
    pub period: Option<String>,
    pub volume: Option<String>,
    pub synthesize: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VerifyCitationArgs {
    pub quote: String,
    pub claimed_title: String,
    pub context_chunks: Option<Vec<String>>,
    pub min_confidence: Option<f64>,
}

// ── Pre-built Tool Definitions (Guaranteed Draft 7 / Go Console Compliant) ──

#[must_use]
pub fn query_dialectical_principles_tool() -> McpToolDefinition {
    McpToolDefinition {
        name: "query_dialectical_principles".to_string(),
        description: "基于毛选与辩证唯物主义文献库检索哲学方法论、矛盾三元组与历史战略案例。支持混合检索、图谱关联推演，并提供可选的大模型哲学综合推演。".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "现实或编码中的困境、矛盾情境或待检索的哲学课题"
                },
                "top_k": {
                    "type": "integer",
                    "description": "召回原典片段数量，默认 3，最大 20",
                    "minimum": 1,
                    "maximum": 20
                },
                "period": {
                    "type": "string",
                    "description": "历史时期过滤（例如：大革命时期、井冈山时期、延安时期、建国后）"
                },
                "volume": {
                    "type": "string",
                    "description": "毛选卷次过滤（例如：第一卷、第二卷、第三卷、第四卷、第五卷）"
                },
                "synthesize": {
                    "type": "boolean",
                    "description": "是否内嵌调用 DialecticalAgent 生成结构化哲学推演报告（主要矛盾剖析与战略切片），默认 false（仅返回原典文献与图谱三元组）"
                }
            },
            "required": ["query"]
        }),
    }
}

#[must_use]
pub fn verify_historical_citation_tool() -> McpToolDefinition {
    McpToolDefinition {
        name: "verify_historical_citation".to_string(),
        description: "对历史文献引述、名言名句进行权威原典比对、真伪核实与精确度评分。若未提供对照正文，服务器自动在本地典籍库中按篇名反查原典正文并校验。".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "quote": {
                    "type": "string",
                    "description": "待核实的引文原文或观点陈述"
                },
                "claimed_title": {
                    "type": "string",
                    "description": "声称的出处篇名（例如：《反对本本主义》、《矛盾论》、《实践论》）"
                },
                "context_chunks": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    },
                    "description": "可选的核对正文片段。若缺省或为空，服务器将自动从本地典籍库中按篇名反查原典正文并校验"
                },
                "min_confidence": {
                    "type": "number",
                    "description": "最低置信度阈值（0.0~1.0），默认 0.85",
                    "minimum": 0.0,
                    "maximum": 1.0
                }
            },
            "required": ["quote", "claimed_title"]
        }),
    }
}

#[must_use]
pub fn list_all_tools() -> Vec<McpToolDefinition> {
    vec![
        query_dialectical_principles_tool(),
        verify_historical_citation_tool(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_rpc_request_response_roundtrip() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: "tools/list".to_string(),
            params: None,
        };
        assert!(!req.is_notification());

        let serialized = serde_json::to_string(&req).expect("serialize request");
        let deserialized: JsonRpcRequest =
            serde_json::from_str(&serialized).expect("deserialize request");
        assert_eq!(req, deserialized);

        let resp = JsonRpcResponse::success(
            Some(serde_json::json!(1)),
            serde_json::json!({ "status": "ok" }),
        );
        let resp_str = serde_json::to_string(&resp).expect("serialize response");
        assert!(resp_str.contains("\"result\":{\"status\":\"ok\"}"));
        assert!(!resp_str.contains("\"error\""));
    }

    #[test]
    fn test_json_rpc_error_codes() {
        let err = JsonRpcError::resource_exhausted("Concurrency limit reached");
        assert_eq!(err.code, -32053);
        assert_eq!(err.message, "Concurrency limit reached");

        let resp = JsonRpcResponse::error(Some(serde_json::json!(42)), err);
        let resp_json = serde_json::to_value(resp).expect("to_value");
        assert_eq!(resp_json["error"]["code"], -32053);
    }

    #[test]
    fn test_strict_json_schema_compliance_no_nested_required_true() {
        let tools = list_all_tools();
        assert_eq!(tools.len(), 2);

        for tool in tools {
            let schema = tool.input_schema;
            assert_eq!(
                schema["type"], "object",
                "Root type must be object for tool {}",
                tool.name
            );

            // Assert properties exists
            assert!(
                schema["properties"].is_object(),
                "Properties must be object for tool {}",
                tool.name
            );

            // Assert root required is an array of strings
            let req_arr = schema["required"]
                .as_array()
                .expect("required must be array");
            assert!(
                !req_arr.is_empty(),
                "required array should not be empty for tool {}",
                tool.name
            );

            // Recursively assert NO property contains `required: true` (which causes OpenCode Go 400 error)
            fn check_no_property_required_true(val: &serde_json::Value, path: &str) {
                if let Some(obj) = val.as_object() {
                    for (k, v) in obj {
                        let current_path = format!("{path}.{k}");
                        if k == "required" {
                            assert!(
                                v.is_array(),
                                "Found non-array 'required' at {current_path}, must be array"
                            );
                        } else {
                            check_no_property_required_true(v, &current_path);
                        }
                    }
                }
            }
            check_no_property_required_true(&schema, &tool.name);
        }
    }
}
