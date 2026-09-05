#!/usr/bin/env python3
"""
Lifecycle Hook Guard for mao_agent.
Enforces safety rules during agent tool execution and stop phases:
1. PreToolUse: Blocks direct tampering with index artifacts (data/vector_store.bin,
   data/tantivy_index) and prevents writing live secrets/API keys to config.toml.
2. Stop: Runs `cargo check --no-default-features --quiet` to prevent stopping with
   compilation errors.
"""

from __future__ import annotations

import json
import os
from pathlib import Path
import re
import subprocess
import sys
from typing import Any, Dict, List, Optional

# Root directory of the mao_agent crate
BASE_DIR = Path(__file__).resolve().parent.parent

# Placeholder / dummy values that are not live secrets
SECRET_PLACEHOLDERS = {
    "",
    "your_api_key",
    "your_key",
    "placeholder",
    "xxx",
    "todo",
    "none",
    "your-cohere-api-key",
    "<api_key>",
    "<your-api-key>",
    "<key>",
}


def contains_live_secret(text: str) -> bool:
    """Detect if text contains a non-placeholder live API key or secret token."""
    if not text:
        return False

    # Check for toml/json/cli key assignment: api_key = "..."
    patterns = [
        r'(?:api[_-]?key|cohere[_-]?key|secret|token)\s*[:=]\s*["\']([^"\']+)["\']',
        r'--(?:api[_-]?key|embed[_-]?api[_-]?key)\s*=?\s*["\']?([^"\s\';]+)["\']?',
    ]
    for pattern in patterns:
        for match in re.finditer(pattern, text, re.IGNORECASE):
            val = match.group(1).strip()
            if val and val.lower() not in SECRET_PLACEHOLDERS and len(val) >= 8:
                return True

    # Check for raw API keys in secret contexts (e.g. 32-64 alphanumeric characters)
    for match in re.finditer(r'\b[A-Za-z0-9_-]{32,64}\b', text):
        val = match.group(0).strip()
        if val.lower() not in SECRET_PLACEHOLDERS:
            if re.search(r'(?:api|key|cohere|token|secret|config)', text, re.IGNORECASE):
                return True

    return False


def is_protected_index_path(path_str: str) -> bool:
    """Check if a path points to or inside data/vector_store.bin or data/tantivy_index."""
    if not path_str:
        return False
    norm = path_str.replace("\\", "/").lower()
    for pattern in [r"data/vector_store\.bin", r"data/tantivy_index"]:
        if re.search(pattern, norm):
            return True
    return False


def handle_pre_tool(payload: Dict[str, Any]) -> Dict[str, Any]:
    """Audit tool call to prevent index tampering and secret leakage."""
    tool_call = payload.get("toolCall") or {}
    tool_name = tool_call.get("name", "")
    args = tool_call.get("args") or {}

    # 1. Check TargetFile argument (standard across write_to_file and replace_file_content)
    target_file = args.get("TargetFile")
    if target_file and isinstance(target_file, str):
        if is_protected_index_path(target_file):
            return {
                "decision": "deny",
                "reason": (
                    f"Direct modification of index artifact '{target_file}' is forbidden. "
                    "Regenerate indexes using `cargo run -- ingest`."
                ),
            }

        norm_target = target_file.replace("\\", "/").lower()
        if norm_target.endswith("config.toml") or "/config.toml" in norm_target:
            # Check content being written
            content_candidates = [
                args.get("CodeContent"),
                args.get("ReplacementContent"),
                args.get("TargetContent"),
            ]
            for content in content_candidates:
                if isinstance(content, str) and contains_live_secret(content):
                    return {
                        "decision": "deny",
                        "reason": (
                            "Writing live secrets to config.toml is prohibited. "
                            "Keep config.toml gitignored with placeholder or use COHERE_API_KEY."
                        ),
                    }

    # 2. Check all string values in args for any index tampering or secret writes
    for key, val in args.items():
        if not isinstance(val, str):
            continue

        # In file-writing tools, only file path arguments (not content being edited) should be checked for index paths
        if tool_name in ("write_to_file", "replace_file_content") and key not in (
            "CodeContent",
            "ReplacementContent",
            "TargetContent",
            "Instruction",
            "Description",
        ):
            if is_protected_index_path(val):
                return {
                    "decision": "deny",
                    "reason": (
                        f"Direct modification of index artifact '{val}' is forbidden. "
                        "Regenerate indexes using `cargo run -- ingest`."
                    ),
                }

        # In command line executions, check for destructive operations or secret injection
        if key == "CommandLine":
            cmd_lower = val.lower().replace("\\", "/")
            if "config.toml" in cmd_lower and contains_live_secret(val):
                return {
                    "decision": "deny",
                    "reason": "Writing live secrets to config.toml via command line is prohibited.",
                }

            # If command attempts direct deletion or overwriting of index files (excluding normal cargo runs)
            is_cargo = re.match(r"^\s*cargo\b", val.strip(), re.IGNORECASE)
            if not is_cargo:
                for pattern in [
                    r">\s*.*data[/\\](?:vector_store\.bin|tantivy_index)",
                    r"\b(?:rm|del|erase|remove-item)\b.*data[/\\](?:vector_store\.bin|tantivy_index)",
                ]:
                    if re.search(pattern, val, re.IGNORECASE):
                        return {
                            "decision": "deny",
                            "reason": (
                                "Direct modification or deletion of index artifacts is forbidden. "
                                "Regenerate indexes using `cargo run -- ingest`."
                            ),
                        }

    return {"decision": "allow"}


def handle_stop() -> Dict[str, Any]:
    """Audit stop phase by running `cargo check --no-default-features --quiet`."""
    cmd = ["cargo", "check", "--no-default-features", "--quiet"]
    try:
        proc = subprocess.run(
            cmd,
            cwd=str(BASE_DIR),
            capture_output=True,
            text=True,
            timeout=120,
        )
        if proc.returncode != 0:
            reason = "cargo check --no-default-features failed, please fix compiler errors."
            if proc.stderr.strip():
                reason += f"\nDetails:\n{proc.stderr.strip()[:600]}"
            return {
                "decision": "continue",
                "reason": reason,
            }
        return {"decision": "allow"}
    except subprocess.TimeoutExpired:
        return {
            "decision": "continue",
            "reason": "cargo check --no-default-features timed out after 120s.",
        }
    except Exception as e:
        return {
            "decision": "continue",
            "reason": f"cargo check execution failed: {e}",
        }


def main() -> None:
    mode = sys.argv[1].lower().replace("_", "-") if len(sys.argv) > 1 else "pre-tool"

    if mode == "stop":
        result = handle_stop()
    elif mode in ("pre-tool", "pre-tool-use", "pretool"):
        payload: Dict[str, Any] = {}
        try:
            raw = sys.stdin.read()
            if raw.strip():
                payload = json.loads(raw)
        except Exception:
            payload = {}
        result = handle_pre_tool(payload)
    else:
        result = {"decision": "allow"}

    sys.stdout.write(json.dumps(result) + "\n")
    sys.stdout.flush()


if __name__ == "__main__":
    main()
