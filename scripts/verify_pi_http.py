#!/usr/bin/env python3
"""Hermetic (and optional live) probe of Pi Agent MCP HTTP POST shape.

Default: --mock — no network. Validates JSON-RPC 2.0 bodies that a Pi client
would POST to /api/v1/mcp (alias POST /mcp).

Optional: --url http://127.0.0.1:8080/api/v1/mcp — live POST.

Stdlib only. Diagnostics on stderr.
"""
from __future__ import annotations

import argparse
import json
import sys
import urllib.error
import urllib.request

PI_CLIENT = {"name": "pi-coding-agent", "version": "0.1.0"}
PROTOCOL = "2024-11-05"
HTTP_PATH = "/api/v1/mcp"


def initialize_body() -> dict:
    return {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": PROTOCOL,
            "capabilities": {},
            "clientInfo": dict(PI_CLIENT),
        },
    }


def tools_list_body() -> dict:
    return {"jsonrpc": "2.0", "id": 2, "method": "tools/list"}


def assert_pi_initialize(body: dict) -> None:
    assert body["jsonrpc"] == "2.0", body
    assert body["method"] == "initialize", body
    params = body["params"]
    assert params["protocolVersion"] == PROTOCOL, params
    assert params["clientInfo"]["name"] == PI_CLIENT["name"], params
    assert params["clientInfo"]["version"] == PI_CLIENT["version"], params


def assert_tools_list(body: dict) -> None:
    assert body["jsonrpc"] == "2.0", body
    assert body["method"] == "tools/list", body


def run_mock() -> int:
    init = initialize_body()
    listing = tools_list_body()
    assert_pi_initialize(init)
    assert_tools_list(listing)
    print(
        f"OK mock Pi HTTP bodies for POST {HTTP_PATH} "
        f"(alias POST /mcp); clientInfo={PI_CLIENT['name']}",
        file=sys.stderr,
    )
    print("OK")
    return 0


def post_json(url: str, body: dict) -> dict:
    data = json.dumps(body).encode("utf-8")
    req = urllib.request.Request(
        url,
        data=data,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    with urllib.request.urlopen(req, timeout=30) as resp:
        raw = resp.read().decode("utf-8")
    parsed = json.loads(raw)
    if not isinstance(parsed, dict):
        raise SystemExit(f"non-object JSON-RPC response: {raw[:200]}")
    return parsed


def run_live(url: str) -> int:
    init = initialize_body()
    listing = tools_list_body()
    assert_pi_initialize(init)
    assert_tools_list(listing)
    r1 = post_json(url, init)
    if r1.get("error"):
        print(f"initialize error: {r1['error']}", file=sys.stderr)
        return 1
    r2 = post_json(url, listing)
    if r2.get("error"):
        print(f"tools/list error: {r2['error']}", file=sys.stderr)
        return 1
    print(f"OK live POST {url}", file=sys.stderr)
    print("OK")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Probe Pi-shaped MCP JSON-RPC POST /api/v1/mcp (stdlib only)."
    )
    parser.add_argument(
        "--mock",
        action="store_true",
        default=False,
        help="Hermetic body check; no network (use this for CI-adjacent verification).",
    )
    parser.add_argument(
        "--url",
        default="",
        help="Live POST URL, e.g. http://127.0.0.1:8080/api/v1/mcp",
    )
    args = parser.parse_args()
    if args.url:
        return run_live(args.url)
    # Default is mock when --url is omitted (plan: --mock is the hermetic gate).
    return run_mock()


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as exc:
        print(f"FAIL: {exc}", file=sys.stderr)
        raise SystemExit(1)
