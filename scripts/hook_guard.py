#!/usr/bin/env python3
"""
Lifecycle Hook Guard for mao_agent.
Enforces safety rules during agent tool execution and stop phases:
1. PreToolUse: Blocks writes matching gate.yaml (indexes, config.toml, .env*, corpus/raw)
   and still rejects live secrets in config.toml content.
2. Stop: Runs `cargo check --no-default-features --quiet` to prevent stopping with
   compilation errors.
"""

from __future__ import annotations

import json
from pathlib import Path
import re
import subprocess
import sys
from typing import Any, Dict, List, Optional

# Root directory of the mao_agent crate
BASE_DIR = Path(__file__).resolve().parent.parent
_SCRIPTS_DIR = Path(__file__).resolve().parent
if str(_SCRIPTS_DIR) not in sys.path:
    sys.path.insert(0, str(_SCRIPTS_DIR))

from gate_check import EXIT_ALLOW, check_gate  # noqa: E402

WRITE_TOOLS = {
    "write_to_file",
    "replace_file_content",
    "Write",
    "write",
    "search_replace",
    "StrReplace",
}

PATH_ARG_KEYS = (
    "TargetFile",
    "target_file",
    "file_path",
    "path",
    "old_path",
    "new_path",
)

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


def extract_write_paths(args: Dict[str, Any]) -> List[str]:
    """Collect path-like args from a write/edit tool call."""
    found: List[str] = []
    for key in PATH_ARG_KEYS:
        val = args.get(key)
        if isinstance(val, str) and val.strip():
            found.append(val.strip())
    return found


def deny_if_gated(paths: List[str]) -> Optional[Dict[str, Any]]:
    """Reject denylisted writes using gate.yaml (fail-closed on checker errors)."""
    if not paths:
        return None
    result = check_gate("tool", paths)
    if result.exit_code == EXIT_ALLOW:
        return None
    return {
        "decision": "deny",
        "reason": (
            f"gate.yaml blocked path write ({result.reason}). "
            "Do not edit secrets, indexes, or corpus/raw assets; "
            "regenerate indexes with `cargo run -- ingest`."
        ),
    }


def handle_pre_tool(payload: Dict[str, Any]) -> Dict[str, Any]:
    """Audit tool call to prevent index tampering and secret leakage."""
    tool_call = payload.get("toolCall") or payload.get("tool_call") or {}
    tool_name = str(tool_call.get("name") or payload.get("tool") or "")
    args = tool_call.get("args") or tool_call.get("input") or payload.get("args") or {}
    if not isinstance(args, dict):
        args = {}

    if tool_name in WRITE_TOOLS:
        gated = deny_if_gated(extract_write_paths(args))
        if gated is not None:
            return gated

    target_file = args.get("TargetFile") or args.get("file_path") or args.get("path")
    if target_file and isinstance(target_file, str):
        norm_target = target_file.replace("\\", "/").lower()
        if norm_target.endswith("config.toml") or "/config.toml" in norm_target:
            content_candidates = [
                args.get("CodeContent"),
                args.get("ReplacementContent"),
                args.get("TargetContent"),
                args.get("contents"),
                args.get("new_string"),
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

    for key, val in args.items():
        if not isinstance(val, str):
            continue

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


def _pretool_payload(tool: str, args: Dict[str, Any]) -> Dict[str, Any]:
    return {"toolCall": {"name": tool, "args": args}}


def self_check() -> int:
    """Prove PreToolUse denies denylisted writes via the shipped gate checker."""
    failures: List[str] = []

    denied = handle_pre_tool(
        _pretool_payload("write_to_file", {"TargetFile": "config.toml", "CodeContent": "x = 1\n"})
    )
    if denied.get("decision") != "deny":
        failures.append(f"config.toml write should deny, got {denied}")
    elif "gate.yaml" not in str(denied.get("reason", "")):
        failures.append(f"deny reason should cite gate.yaml, got {denied}")

    allowed = handle_pre_tool(
        _pretool_payload("write_to_file", {"TargetFile": "src/lib.rs", "CodeContent": "// ok\n"})
    )
    if allowed.get("decision") != "allow":
        failures.append(f"src/lib.rs write should allow, got {allowed}")

    raw_pdf = handle_pre_tool(
        _pretool_payload(
            "replace_file_content",
            {"TargetFile": "corpus/papers_we_love/raw/doc.pdf", "ReplacementContent": "x"},
        )
    )
    if raw_pdf.get("decision") != "deny":
        failures.append(f"corpus raw write should deny, got {raw_pdf}")

    proc = subprocess.run(
        [sys.executable, str(Path(__file__).resolve()), "pre-tool"],
        input=json.dumps(
            _pretool_payload("write_to_file", {"TargetFile": "data/vector_store.bin", "CodeContent": "x"})
        ),
        capture_output=True,
        text=True,
        cwd=str(BASE_DIR),
    )
    try:
        parsed = json.loads(proc.stdout.strip().splitlines()[-1])
    except (json.JSONDecodeError, IndexError):
        parsed = {}
        failures.append(f"stdin pre-tool JSON parse failed: {proc.stdout!r} {proc.stderr!r}")
    if parsed.get("decision") != "deny":
        failures.append(f"stdin denylist write should deny, got {parsed}")

    if failures:
        sys.stderr.write("hook_guard self-check FAILED\n" + "\n".join(failures) + "\n")
        return 1
    sys.stdout.write("hook_guard self-check passed\n")
    return 0


def main() -> None:
    mode = sys.argv[1].lower().replace("_", "-") if len(sys.argv) > 1 else "self-check"

    if mode in ("self-check", "self-test", "--self-test"):
        raise SystemExit(self_check())

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
