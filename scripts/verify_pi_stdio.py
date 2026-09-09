#!/usr/bin/env python3
"""Ops probe: Pi Agent-shaped MCP stdio JSON-RPC against mao_agent.

Spawns `cargo run --no-default-features -- mcp --offline` (debug, not --release)
or an existing binary via --bin. Drives initialize → initialized → tools/list →
query_dialectical_principles (synthesize false). Child traces stay on stderr.

Stdlib only. Not a CI gate. Diagnostics on stderr.
"""
from __future__ import annotations

import argparse
import json
import subprocess
import sys
import threading
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
PI_CLIENT = {"name": "pi-coding-agent", "version": "0.1.0"}
PROTOCOL = "2024-11-05"


def _drain_stderr(pipe) -> None:
    try:
        for _line in iter(pipe.readline, ""):
            sys.stderr.write(_line)
        pipe.close()
    except OSError:
        pass


def _write_rpc(proc: subprocess.Popen, obj: dict) -> None:
    assert proc.stdin is not None
    proc.stdin.write(json.dumps(obj, ensure_ascii=False) + "\n")
    proc.stdin.flush()


def _read_rpc(proc: subprocess.Popen, timeout_s: float) -> dict:
    assert proc.stdout is not None
    line = []

    def _read() -> None:
        raw = proc.stdout.readline()
        line.append(raw)

    t = threading.Thread(target=_read, daemon=True)
    t.start()
    t.join(timeout_s)
    if t.is_alive():
        raise TimeoutError(f"no stdout JSON-RPC line within {timeout_s}s")
    raw = line[0] if line else ""
    if not raw:
        raise RuntimeError("EOF on MCP stdout before JSON-RPC response")
    try:
        parsed = json.loads(raw)
    except json.JSONDecodeError as exc:
        raise RuntimeError(f"non-JSON stdout line: {raw[:200]!r}") from exc
    if not isinstance(parsed, dict):
        raise RuntimeError(f"non-object JSON-RPC: {parsed!r}")
    return parsed


def _expect_ok(msg: dict, label: str) -> dict:
    if msg.get("jsonrpc") != "2.0":
        raise RuntimeError(f"{label}: missing jsonrpc 2.0: {msg}")
    if msg.get("error"):
        raise RuntimeError(f"{label}: JSON-RPC error: {msg['error']}")
    if "result" not in msg:
        raise RuntimeError(f"{label}: no result: {msg}")
    return msg["result"]


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Probe Pi-shaped MCP stdio (line-delimited JSON-RPC)."
    )
    parser.add_argument(
        "--bin",
        default="",
        help="Path to mao-agent binary. Default: cargo run --no-default-features -- mcp --offline",
    )
    parser.add_argument(
        "--timeout",
        type=float,
        default=180.0,
        help="Seconds to wait for each stdout frame (first cargo run may compile).",
    )
    args = parser.parse_args()

    if args.bin:
        cmd = [args.bin, "mcp", "--offline"]
    else:
        cmd = [
            "cargo",
            "run",
            "--no-default-features",
            "--",
            "mcp",
            "--offline",
        ]

    print(f"spawning: {cmd} cwd={REPO}", file=sys.stderr)
    proc = subprocess.Popen(
        cmd,
        cwd=str(REPO),
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        encoding="utf-8",
        errors="replace",
    )
    assert proc.stderr is not None
    threading.Thread(target=_drain_stderr, args=(proc.stderr,), daemon=True).start()

    try:
        _write_rpc(
            proc,
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {
                    "protocolVersion": PROTOCOL,
                    "capabilities": {},
                    "clientInfo": dict(PI_CLIENT),
                },
            },
        )
        init = _expect_ok(_read_rpc(proc, args.timeout), "initialize")
        if init.get("protocolVersion") != PROTOCOL:
            raise RuntimeError(f"unexpected protocolVersion: {init.get('protocolVersion')}")

        _write_rpc(
            proc,
            {"jsonrpc": "2.0", "method": "notifications/initialized"},
        )

        _write_rpc(proc, {"jsonrpc": "2.0", "id": 2, "method": "tools/list"})
        listed = _expect_ok(_read_rpc(proc, args.timeout), "tools/list")
        tools = listed.get("tools")
        if not isinstance(tools, list) or not tools:
            raise RuntimeError(f"tools/list empty: {listed}")

        _write_rpc(
            proc,
            {
                "jsonrpc": "2.0",
                "id": 3,
                "method": "tools/call",
                "params": {
                    "name": "query_dialectical_principles",
                    "arguments": {
                        "query": "矛盾的法则与转化",
                        "synthesize": False,
                    },
                },
            },
        )
        try:
            call = _read_rpc(proc, args.timeout)
            if call.get("error"):
                print(f"warning: tools/call error {call['error']}", file=sys.stderr)
            else:
                print("tools/call query_dialectical_principles ok", file=sys.stderr)
        except (TimeoutError, RuntimeError) as exc:
            print(f"warning: tools/call skipped: {exc}", file=sys.stderr)

        print("OK Pi stdio initialize + tools/list", file=sys.stderr)
        print("OK")
        return 0
    finally:
        if proc.stdin:
            try:
                proc.stdin.close()
            except OSError:
                pass
        try:
            proc.wait(timeout=10)
        except subprocess.TimeoutExpired:
            proc.kill()


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:  # noqa: BLE001 — ops probe, surface any frame error
        print(f"FAIL: {exc}", file=sys.stderr)
        raise SystemExit(1)
