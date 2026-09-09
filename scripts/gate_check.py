#!/usr/bin/env python3
"""loop-gate compatible path policy checker (Python 3 stdlib only).

CLI: check --action <commit|tool|merge|auto-merge> --paths p1,p2
     [--gate-file gate.yaml] [--json]

Exit: 0 allow · 2 escalate · 1 error (bad flags / invalid YAML).
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import tempfile
from dataclasses import dataclass, field
from pathlib import Path
from typing import List, Optional, Sequence, Tuple

EXIT_ALLOW = 0
EXIT_ERROR = 1
EXIT_ESCALATE = 2

ALLOWED_ACTIONS = ("commit", "tool", "merge", "auto-merge")
REPO_ROOT = Path(__file__).resolve().parent.parent

_GLOB_CACHE: dict[str, re.Pattern[str]] = {}


class GateYamlError(ValueError):
    """Invalid gate.yaml syntax or schema."""


@dataclass
class GatePolicy:
    version: int
    denylist: List[str]
    max_files: Optional[int]
    auto_merge_allowlist: List[str]
    source: Path = field(default_factory=lambda: Path("gate.yaml"))


@dataclass
class GateResult:
    exit_code: int
    decision: str
    reason: str
    matched_pattern: Optional[str] = None
    action: str = ""
    paths: List[str] = field(default_factory=list)

    def to_json(self) -> dict:
        return {
            "decision": self.decision,
            "exit": self.exit_code,
            "reason": self.reason,
            "matched_pattern": self.matched_pattern,
            "action": self.action,
            "paths": self.paths,
        }


def _parse_scalar(raw: str):
    s = raw.strip()
    if len(s) >= 2 and s[0] == s[-1] and s[0] in "\"'":
        return s[1:-1]
    if re.fullmatch(r"-?\d+", s):
        return int(s)
    if s.lower() in ("true", "false"):
        return s.lower() == "true"
    if s.lower() in ("null", "~"):
        return None
    return s


def parse_gate_yaml(text: str) -> dict:
    """Minimal YAML subset: comments, scalars, and lists of scalars."""
    if not text.strip():
        raise GateYamlError("empty YAML")
    data: dict = {}
    current_list: Optional[str] = None
    for lineno, raw in enumerate(text.splitlines(), 1):
        stripped = raw.split("#", 1)[0].rstrip()
        if not stripped.strip():
            continue
        if stripped.lstrip().startswith("- "):
            indent = len(stripped) - len(stripped.lstrip())
            if indent == 0 or current_list is None:
                raise GateYamlError(f"line {lineno}: list item without a key")
            item = _parse_scalar(stripped.lstrip()[2:])
            data[current_list].append(item)
            continue
        if stripped[0] in " \t":
            raise GateYamlError(f"line {lineno}: unexpected indent")
        if ":" not in stripped:
            raise GateYamlError(f"line {lineno}: expected 'key:'")
        key, rest = stripped.split(":", 1)
        key = key.strip()
        rest = rest.strip()
        if not key:
            raise GateYamlError(f"line {lineno}: empty key")
        if rest == "":
            current_list = key
            data[key] = []
        else:
            current_list = None
            data[key] = _parse_scalar(rest)
    return data


def policy_from_mapping(data: dict, source: Path) -> GatePolicy:
    if "version" not in data:
        raise GateYamlError("missing 'version'")
    version = data["version"]
    if version != 1:
        raise GateYamlError(f"unsupported version: {version}")
    denylist = data.get("denylist", [])
    if not isinstance(denylist, list) or not all(isinstance(x, str) for x in denylist):
        raise GateYamlError("'denylist' must be a list of strings")
    allow = data.get("autoMergeAllowlist", [])
    if not isinstance(allow, list) or not all(isinstance(x, str) for x in allow):
        raise GateYamlError("'autoMergeAllowlist' must be a list of strings")
    max_files = data.get("maxFiles")
    if max_files is not None and not isinstance(max_files, int):
        raise GateYamlError("'maxFiles' must be an integer")
    return GatePolicy(
        version=int(version),
        denylist=denylist,
        max_files=max_files,
        auto_merge_allowlist=allow,
        source=source,
    )


def load_policy(path: Path) -> GatePolicy:
    try:
        text = path.read_text(encoding="utf-8")
    except OSError as e:
        raise GateYamlError(f"cannot read {path}: {e}") from e
    return policy_from_mapping(parse_gate_yaml(text), path)


def default_gate_file() -> Path:
    cwd = Path.cwd() / "gate.yaml"
    if cwd.is_file():
        return cwd
    rooted = REPO_ROOT / "gate.yaml"
    return rooted


def normalize_path(path: str) -> str:
    raw = path.replace("\\", "/").strip()
    if raw.startswith("./"):
        raw = raw[2:]
    try:
        p = Path(path)
        if p.is_absolute():
            try:
                raw = p.resolve().relative_to(REPO_ROOT.resolve()).as_posix()
            except ValueError:
                raw = p.as_posix().lstrip("/")
    except (OSError, RuntimeError):
        pass
    return raw.lstrip("/")


def _glob_to_regex(pattern: str) -> re.Pattern[str]:
    cached = _GLOB_CACHE.get(pattern)
    if cached is not None:
        return cached
    i = 0
    out: List[str] = ["^"]
    while i < len(pattern):
        if pattern.startswith("**/", i):
            out.append("(?:.*/)?")
            i += 3
            continue
        if pattern.startswith("**", i):
            out.append(".*")
            i += 2
            continue
        ch = pattern[i]
        if ch == "*":
            out.append("[^/]*")
        elif ch == "?":
            out.append("[^/]")
        else:
            out.append(re.escape(ch))
        i += 1
    out.append("$")
    compiled = re.compile("".join(out))
    _GLOB_CACHE[pattern] = compiled
    return compiled


def glob_match(path: str, pattern: str) -> bool:
    """minimatch-ish globstar; slash-less patterns also match the basename."""
    norm = normalize_path(path)
    pat = pattern.replace("\\", "/")
    if pat.endswith("/**") and glob_match(norm, pat[:-3]):
        return True
    if _glob_to_regex(pat).match(norm):
        return True
    if "/" not in pat.rstrip("/"):
        base = norm.rsplit("/", 1)[-1]
        if _glob_to_regex(pat).match(base):
            return True
    return False


def evaluate(policy: GatePolicy, action: str, paths: Sequence[str]) -> GateResult:
    action = action.strip()
    path_list = [p for p in (x.strip() for x in paths) if p]
    base = GateResult(
        exit_code=EXIT_ALLOW,
        decision="allow",
        reason="allow",
        action=action,
        paths=list(path_list),
    )
    if action not in ALLOWED_ACTIONS:
        return GateResult(
            EXIT_ERROR,
            "error",
            f"invalid --action {action!r} (want {', '.join(ALLOWED_ACTIONS)})",
            action=action,
            paths=list(path_list),
        )
    if not path_list:
        return GateResult(
            EXIT_ERROR,
            "error",
            "no paths given",
            action=action,
            paths=[],
        )

    for p in path_list:
        for pat in policy.denylist:
            if glob_match(p, pat):
                return GateResult(
                    EXIT_ESCALATE,
                    "escalate",
                    f"denylist matched {p!r} with {pat!r}",
                    matched_pattern=pat,
                    action=action,
                    paths=list(path_list),
                )

    if policy.max_files is not None and len(path_list) > policy.max_files:
        return GateResult(
            EXIT_ESCALATE,
            "escalate",
            f"maxFiles {policy.max_files} exceeded ({len(path_list)} paths)",
            action=action,
            paths=list(path_list),
        )

    if action == "auto-merge":
        for p in path_list:
            if not any(glob_match(p, pat) for pat in policy.auto_merge_allowlist):
                return GateResult(
                    EXIT_ESCALATE,
                    "escalate",
                    f"auto-merge path {p!r} not on allowlist",
                    action=action,
                    paths=list(path_list),
                )

    return base


def check_gate(
    action: str,
    paths: Sequence[str],
    gate_file: Optional[Path] = None,
) -> GateResult:
    path = Path(gate_file) if gate_file is not None else default_gate_file()
    if not path.is_file():
        return GateResult(
            EXIT_ERROR,
            "error",
            f"gate file not found: {path}",
            action=action,
            paths=list(paths),
        )
    try:
        policy = load_policy(path)
    except GateYamlError as e:
        return GateResult(EXIT_ERROR, "error", str(e), action=action, paths=list(paths))
    return evaluate(policy, action, paths)


def _emit(result: GateResult, as_json: bool) -> int:
    if as_json:
        sys.stdout.write(json.dumps(result.to_json(), ensure_ascii=False) + "\n")
    else:
        stream = sys.stdout if result.exit_code == EXIT_ALLOW else sys.stderr
        stream.write(result.reason + "\n")
    return result.exit_code


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="gate_check.py", description=__doc__)
    sub = parser.add_subparsers(dest="cmd")
    check = sub.add_parser("check", help="evaluate paths against gate.yaml")
    check.add_argument(
        "--action",
        required=True,
        help="commit | tool | merge | auto-merge",
    )
    check.add_argument(
        "--paths",
        required=True,
        help="comma-separated changed file paths",
    )
    check.add_argument("--gate-file", default=None, help="policy file (default: ./gate.yaml)")
    check.add_argument("--json", action="store_true", help="machine-readable decision")
    return parser


def run_cli(argv: Optional[Sequence[str]] = None) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)
    if argv and argv[0] in ("--self-test", "self-test"):
        return run_self_test()
    parser = build_parser()
    try:
        args = parser.parse_args(argv)
    except SystemExit as e:
        code = e.code if isinstance(e.code, int) else EXIT_ERROR
        return EXIT_ERROR if code == 2 else int(code)
    if args.cmd != "check":
        parser.print_usage(sys.stderr)
        sys.stderr.write("error: missing command 'check'\n")
        return EXIT_ERROR
    paths = [p.strip() for p in args.paths.split(",") if p.strip()]
    gate_file = Path(args.gate_file) if args.gate_file else None
    return _emit(check_gate(args.action, paths, gate_file), args.json)


def _cli_exit(extra: List[str], gate_file: Path) -> Tuple[int, str, str]:
    cmd = [
        sys.executable,
        str(Path(__file__).resolve()),
        "check",
        "--gate-file",
        str(gate_file),
        *extra,
    ]
    proc = subprocess.run(cmd, capture_output=True, text=True, cwd=str(REPO_ROOT))
    return proc.returncode, proc.stdout, proc.stderr


def run_self_test() -> int:
    """Drive the shipped CLI (not a reimplementation) against the repo gate.yaml."""
    gate = REPO_ROOT / "gate.yaml"
    failures: List[str] = []

    def expect(label: str, extra: List[str], want: int) -> None:
        code, out, err = _cli_exit(extra, gate)
        if code != want:
            failures.append(
                f"{label}: want exit {want} got {code} stdout={out!r} stderr={err!r}"
            )

    expect("denylist config.toml", ["--action", "tool", "--paths", "config.toml"], EXIT_ESCALATE)
    expect("allow src", ["--action", "tool", "--paths", "src/lib.rs"], EXIT_ALLOW)
    expect(
        "denylist corpus raw",
        ["--action", "tool", "--paths", "corpus/papers_we_love/raw/test.pdf"],
        EXIT_ESCALATE,
    )
    expect(
        "denylist data bin",
        ["--action", "commit", "--paths", "data/vector_store.bin"],
        EXIT_ESCALATE,
    )
    expect(
        "denylist tantivy",
        ["--action", "tool", "--paths", "data/tantivy_index/meta.json"],
        EXIT_ESCALATE,
    )
    expect("denylist env", ["--action", "tool", "--paths", ".env.local"], EXIT_ESCALATE)
    expect(
        "maxFiles",
        [
            "--action",
            "tool",
            "--paths",
            ",".join(f"src/f{i}.rs" for i in range(9)),
        ],
        EXIT_ESCALATE,
    )
    expect(
        "auto-merge docs",
        ["--action", "auto-merge", "--paths", "docs/ops/runbook.md"],
        EXIT_ALLOW,
    )
    expect(
        "auto-merge src escalate",
        ["--action", "auto-merge", "--paths", "src/lib.rs"],
        EXIT_ESCALATE,
    )

    with tempfile.TemporaryDirectory() as td:
        bad = Path(td) / "bad.yaml"
        bad.write_text("version: [\n", encoding="utf-8")
        code, _, err = _cli_exit(["--action", "tool", "--paths", "src/lib.rs"], bad)
        if code != EXIT_ERROR:
            failures.append(f"invalid YAML: want 1 got {code} stderr={err!r}")
        missing = Path(td) / "nope.yaml"
        code, _, _ = _cli_exit(["--action", "tool", "--paths", "src/lib.rs"], missing)
        if code != EXIT_ERROR:
            failures.append(f"missing YAML: want 1 got {code}")

    code, _, err = _cli_exit(["--action", "nope", "--paths", "src/lib.rs"], gate)
    if code != EXIT_ERROR:
        failures.append(f"bad action: want 1 got {code} stderr={err!r}")

    proc = subprocess.run(
        [sys.executable, str(Path(__file__).resolve())],
        capture_output=True,
        text=True,
        cwd=str(REPO_ROOT),
    )
    if proc.returncode != EXIT_ERROR:
        failures.append(f"missing subcommand: want 1 got {proc.returncode}")

    if failures:
        sys.stderr.write("gate_check self-test FAILED\n" + "\n".join(failures) + "\n")
        return EXIT_ERROR
    sys.stdout.write("gate_check self-test passed\n")
    return EXIT_ALLOW


def main() -> None:
    raise SystemExit(run_cli())


if __name__ == "__main__":
    main()
