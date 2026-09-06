//! Standard I/O transport for MCP JSON-RPC 2.0 (stdio mode).
//!
//! Compliant with:
//! - Model Context Protocol 2024-11-05 stdio specification
//! - DeepSeek Harness `@deepseek-ai/dsh-mcp-client` stdio child process protocol
//! - Google SRE Failure Domain Isolation (graceful EOF exit, zero panic, stderr logging isolation)

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::mcp::dispatcher::McpDispatcher;
use crate::mcp::types::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};

/// Run the stdio MCP server loop on standard input and standard output.
pub async fn run_stdio_server(dispatcher: McpDispatcher) -> std::io::Result<()> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let reader = BufReader::new(stdin);
    run_stdio_loop(reader, stdout, &dispatcher).await
}

/// Generic I/O loop allowing hermetic testing over mock async reader and writer.
pub async fn run_stdio_loop<R, W>(
    mut reader: R,
    mut writer: W,
    dispatcher: &McpDispatcher,
) -> std::io::Result<()>
where
    R: AsyncBufReadExt + Unpin,
    W: AsyncWriteExt + Unpin,
{
    let mut line = String::new();

    while reader.read_line(&mut line).await? != 0 {
        let trimmed = line.trim().trim_start_matches('\u{feff}');
        if !trimmed.is_empty() {
            let req: JsonRpcRequest = match serde_json::from_str(trimmed) {
                Ok(r) => r,
                Err(e) => {
                    let err_resp = JsonRpcResponse::error(
                        None,
                        JsonRpcError::parse_error(format!("Failed to parse JSON-RPC: {e}")),
                    );
                    let mut out = serde_json::to_string(&err_resp).unwrap_or_default();
                    out.push('\n');
                    writer.write_all(out.as_bytes()).await?;
                    writer.flush().await?;
                    line.clear();
                    continue;
                }
            };

            if let Some(resp) = dispatcher.handle_request(req).await {
                let mut out = serde_json::to_string(&resp).unwrap_or_default();
                out.push('\n');
                writer.write_all(out.as_bytes()).await?;
                writer.flush().await?;
            }
        }
        line.clear();
    }

    tracing::info!("MCP stdio reader reached EOF; terminating loop cleanly");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::VectorStore;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_stdio_loop_handshake_and_eof() {
        let store = Arc::new(VectorStore::new_deterministic(64));
        let dispatcher = McpDispatcher::new(store, None, None, None);

        let input_lines = concat!(
            "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\"}\n",
            "{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n",
            "{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/list\"}\n"
        );

        let reader = std::io::Cursor::new(input_lines.as_bytes());
        let mut writer = Vec::new();

        run_stdio_loop(reader, &mut writer, &dispatcher)
            .await
            .expect("loop should run clean to EOF");

        let output_str = String::from_utf8(writer).expect("utf8 string");
        let lines: Vec<&str> = output_str.trim().split('\n').collect();
        assert_eq!(
            lines.len(),
            2,
            "Expected 2 responses for 2 calls (notification produces no response)"
        );

        let resp1: serde_json::Value = serde_json::from_str(lines[0]).expect("json parse resp1");
        assert_eq!(resp1["id"], 1);
        assert_eq!(resp1["result"]["protocolVersion"], "2024-11-05");

        let resp2: serde_json::Value = serde_json::from_str(lines[1]).expect("json parse resp2");
        assert_eq!(resp2["id"], 2);
        let tools = resp2["result"]["tools"].as_array().expect("tools array");
        assert_eq!(tools.len(), 2);
    }

    #[tokio::test]
    async fn test_stdio_loop_handles_malformed_json_without_exit() {
        let store = Arc::new(VectorStore::new_deterministic(64));
        let dispatcher = McpDispatcher::new(store, None, None, None);

        let input_lines = concat!(
            "NOT_VALID_JSON\n",
            "{\"jsonrpc\":\"2.0\",\"id\":99,\"method\":\"ping\"}\n"
        );

        let reader = std::io::Cursor::new(input_lines.as_bytes());
        let mut writer = Vec::new();

        run_stdio_loop(reader, &mut writer, &dispatcher)
            .await
            .expect("loop should survive parse errors");

        let output_str = String::from_utf8(writer).expect("utf8 string");
        let lines: Vec<&str> = output_str.trim().split('\n').collect();
        assert_eq!(lines.len(), 2);

        let err_resp: serde_json::Value = serde_json::from_str(lines[0]).expect("json parse err");
        assert_eq!(err_resp["error"]["code"], -32700);

        let ping_resp: serde_json::Value = serde_json::from_str(lines[1]).expect("json parse ping");
        assert_eq!(ping_resp["id"], 99);
    }
}
