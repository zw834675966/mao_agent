#!/usr/bin/env python3
"""Report-only L1 loop: compile, corpus/raw, indexes, Recall@5, hard-negatives, STATE.md.

Does not rewrite application source. The only allowed write is an atomic replace
of repo-root STATE.md on --full.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
import tempfile
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Dict, List, Optional, Sequence, Tuple

ROOT = Path(__file__).resolve().parent.parent
NAV_NAMES = {"readme.md", "index.md", "changelog.md"}
RECALL_GATE = 0.99
STATE_PATH = ROOT / "STATE.md"
HEADINGS = (
    "Corpus & Index State Snapshot",
    "Retrieval Baseline Gate",
    "High Priority",
    "Watch List",
    "Recent Noise",
)
L1_PREFIX = "- [L1] "


@dataclass
class GateOutcome:
    name: str
    ok: bool
    detail: str
    extra: Dict = field(default_factory=dict)


def utc_now() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def pause_requested() -> bool:
    return os.environ.get("MAO_LOOP_PAUSE", "").strip() in {"1", "true", "TRUE", "yes"}


def run_cmd(
    cmd: Sequence[str],
    timeout: int,
) -> Tuple[int, str, str]:
    proc = subprocess.run(
        list(cmd),
        cwd=str(ROOT),
        capture_output=True,
        text=True,
        timeout=timeout,
    )
    return proc.returncode, proc.stdout, proc.stderr


def is_under_raw(path: Path, corpus: Path) -> bool:
    try:
        parts = path.resolve().relative_to(corpus.resolve()).parts
    except ValueError:
        return False
    return any(p.lower() == "raw" for p in parts)


def iter_corpus_markdown(corpus: Path) -> Tuple[List[Path], List[Path]]:
    active: List[Path] = []
    raw_files: List[Path] = []
    if not corpus.is_dir():
        return active, raw_files
    for p in corpus.rglob("*"):
        if not p.is_file():
            continue
        if is_under_raw(p, corpus):
            raw_files.append(p)
            continue
        if p.suffix.lower() in {".md", ".markdown"}:
            active.append(p)
    return active, raw_files


def has_title_frontmatter(path: Path) -> bool:
    try:
        text = path.read_text(encoding="utf-8")
    except OSError:
        return False
    if not text.startswith("---"):
        return False
    end = text.find("\n---", 3)
    if end < 0:
        return False
    block = text[3:end]
    return bool(re.search(r"(?m)^title\s*:", block))


def gate_compile() -> GateOutcome:
    try:
        code, out, err = run_cmd(
            ["cargo", "check", "--no-default-features"],
            timeout=180,
        )
    except FileNotFoundError:
        return GateOutcome("1 compile", False, "cargo not found")
    except subprocess.TimeoutExpired:
        return GateOutcome("1 compile", False, "cargo check timed out (180s)")
    if code != 0:
        tail = (err or out).strip()[-800:]
        return GateOutcome("1 compile", False, f"cargo check exit {code}\n{tail}")
    return GateOutcome("1 compile", True, "cargo check --no-default-features ok")


def gate_corpus() -> GateOutcome:
    corpus = ROOT / "corpus"
    if not corpus.is_dir():
        return GateOutcome("2 corpus", False, "corpus/ missing")
    active, raw_files = iter_corpus_markdown(corpus)
    leaked = [p for p in active if is_under_raw(p, corpus)]
    if leaked:
        return GateOutcome("2 corpus", False, f"raw/ leaked into active set: {leaked[:3]}")
    missing: List[str] = []
    formal = 0
    nav = 0
    for p in active:
        rel = p.relative_to(ROOT).as_posix()
        if p.name.lower() in NAV_NAMES:
            nav += 1
            continue
        formal += 1
        if not has_title_frontmatter(p):
            missing.append(rel)
    if missing:
        preview = ", ".join(missing[:8])
        return GateOutcome(
            "2 corpus",
            False,
            f"{len(missing)} formal docs missing title frontmatter: {preview}",
            extra={"active": len(active), "raw": len(raw_files), "formal": formal, "nav": nav},
        )
    detail = (
        f"{formal} formal docs with title; {nav} nav files skipped; "
        f"{len(raw_files)} raw assets isolated"
    )
    return GateOutcome(
        "2 corpus",
        True,
        detail,
        extra={
            "active": len(active),
            "raw": len(raw_files),
            "formal": formal,
            "nav": nav,
        },
    )


def _index_ok(bin_path: Path, tantivy: Path) -> bool:
    return (
        bin_path.is_file()
        and bin_path.stat().st_size > 0
        and tantivy.is_dir()
        and (tantivy / "meta.json").is_file()
    )


def gate_indexes() -> GateOutcome:
    default_bin = ROOT / "data" / "vector_store.bin"
    default_tv = ROOT / "data" / "tantivy_index"
    offline_bin = ROOT / "data" / "offline_run2" / "vector_store.bin"
    offline_tv = ROOT / "data" / "offline_run2" / "tantivy_index"
    graph = ROOT / "data" / "graph_store.bin"
    default_ok = _index_ok(default_bin, default_tv)
    offline_ok = _index_ok(offline_bin, offline_tv)
    if not default_ok and not offline_ok:
        return GateOutcome(
            "3 indexes",
            False,
            "no usable vector_store.bin + tantivy_index (checked data/ and data/offline_run2/)",
        )
    bits = []
    if default_ok:
        bits.append(f"default bin {default_bin.stat().st_size} bytes")
    if offline_ok:
        bits.append(f"offline_run2 bin {offline_bin.stat().st_size} bytes")
    if graph.is_file():
        bits.append(f"graph_store.bin {graph.stat().st_size} bytes")
    else:
        bits.append("graph_store.bin absent (no-op)")
    return GateOutcome(
        "3 indexes",
        True,
        "; ".join(bits),
        extra={"default_ok": default_ok, "offline_ok": offline_ok},
    )


def _eval_cmd(index_file: Path, tantivy_dir: Path, queries: Path) -> List[str]:
    return [
        "cargo",
        "run",
        "--no-default-features",
        "--quiet",
        "--",
        "eval-retrieval",
        "--k",
        "5",
        "--mode",
        "hybrid",
        "--no-rerank",
        "--offline",
        "--json",
        "--queries-file",
        str(queries),
        "--index-file",
        str(index_file),
        "--tantivy-dir",
        str(tantivy_dir),
    ]


def _parse_eval_json(stdout: str) -> Tuple[Optional[dict], List[dict]]:
    summary = None
    misses: List[dict] = []
    for line in stdout.splitlines():
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            obj = json.loads(line)
        except json.JSONDecodeError:
            continue
        if obj.get("type") == "summary":
            summary = obj
        elif obj.get("type") == "query":
            rec = obj.get("recall_at_k")
            if isinstance(rec, (int, float)) and rec < 1.0:
                misses.append(obj)
    return summary, misses


def _looks_dim_mismatch(stdout: str, stderr: str) -> bool:
    blob = stdout + "\n" + stderr
    return "向量模型不匹配" in blob or "IdentityMismatch" in blob or "DimensionMismatch" in blob


def gate_recall() -> GateOutcome:
    queries = ROOT / "evals" / "retrieval" / "queries.jsonl"
    if not queries.is_file():
        return GateOutcome("4 recall", False, "evals/retrieval/queries.jsonl missing")
    candidates = [
        (ROOT / "data" / "vector_store.bin", ROOT / "data" / "tantivy_index"),
        (
            ROOT / "data" / "offline_run2" / "vector_store.bin",
            ROOT / "data" / "offline_run2" / "tantivy_index",
        ),
        (
            ROOT / "data" / "offline_run" / "vector_store.bin",
            ROOT / "data" / "offline_run" / "tantivy_index",
        ),
    ]
    last_err = "no snapshot tried"
    for bin_path, tv in candidates:
        if not _index_ok(bin_path, tv):
            continue
        try:
            code, out, err = run_cmd(_eval_cmd(bin_path, tv, queries), timeout=600)
        except FileNotFoundError:
            return GateOutcome("4 recall", False, "cargo not found")
        except subprocess.TimeoutExpired:
            return GateOutcome("4 recall", False, "eval-retrieval timed out (600s)")
        if code != 0 and _looks_dim_mismatch(out, err):
            last_err = f"dim mismatch on {bin_path.as_posix()}"
            continue
        if code != 0:
            last_err = f"eval-retrieval exit {code} on {bin_path.name}: {(err or out).strip()[-600:]}"
            continue
        summary, misses = _parse_eval_json(out)
        if not summary or "recall_at_k" not in summary:
            last_err = f"no summary JSON from eval-retrieval ({bin_path.name})"
            continue
        recall = float(summary["recall_at_k"])
        ok = recall >= RECALL_GATE
        detail = (
            f"Recall@5={recall:.3f} MRR@5={float(summary.get('mrr_at_k', 0)):.3f} "
            f"NDCG@5={float(summary.get('ndcg_at_k', 0)):.3f} "
            f"n={summary.get('n_queries')} index={bin_path.relative_to(ROOT).as_posix()}"
        )
        if not ok:
            detail += f" (gate ≥ {RECALL_GATE})"
        return GateOutcome(
            "4 recall",
            ok,
            detail,
            extra={
                "summary": summary,
                "misses": misses,
                "index": bin_path.relative_to(ROOT).as_posix(),
            },
        )
    return GateOutcome("4 recall", False, last_err)


def gate_hard_negative() -> GateOutcome:
    try:
        code, out, err = run_cmd(
            [
                "cargo",
                "test",
                "--no-default-features",
                "--test",
                "retrieval_hard_eval_test",
                "--",
                "--test-threads",
                "1",
            ],
            timeout=300,
        )
    except FileNotFoundError:
        return GateOutcome("5 hard-neg", False, "cargo not found")
    except subprocess.TimeoutExpired:
        return GateOutcome("5 hard-neg", False, "hard-negative test timed out (300s)")
    if code != 0:
        tail = (err or out).strip()[-800:]
        return GateOutcome("5 hard-neg", False, f"retrieval_hard_eval_test exit {code}\n{tail}")
    return GateOutcome("5 hard-neg", True, "retrieval_hard_eval_test ok (8-gram hard gold)")


def split_state(text: str) -> Dict[str, str]:
    sections: Dict[str, str] = {h: "" for h in HEADINGS}
    current: Optional[str] = None
    buf: List[str] = []
    preamble: List[str] = []
    for line in text.splitlines():
        if line.startswith("## "):
            title = line[3:].strip()
            if current is None:
                preamble.append("\n".join(buf))
            elif current in sections:
                sections[current] = "\n".join(buf).strip()
            buf = []
            current = title if title in sections else None
            continue
        buf.append(line)
    if current in sections:
        sections[current] = "\n".join(buf).strip()
    sections["_preamble"] = "\n".join(preamble).strip()
    return sections


def atomic_write_state(content: str) -> None:
    path = STATE_PATH
    fd, tmp_name = tempfile.mkstemp(prefix=".state-", suffix=".tmp", dir=str(path.parent))
    try:
        with os.fdopen(fd, "w", encoding="utf-8", newline="\n") as fh:
            fh.write(content)
            if not content.endswith("\n"):
                fh.write("\n")
        os.replace(tmp_name, path)
    except Exception:
        try:
            os.unlink(tmp_name)
        except OSError:
            pass
        raise


def render_snapshot(corpus: GateOutcome, indexes: GateOutcome) -> str:
    extra = corpus.extra
    rows = [
        "| Item | Value |",
        "|------|-------|",
        f"| Active corpus markdown (`raw/` skipped) | {extra.get('active', '?')} |",
        f"| Formal docs with `title:` | {extra.get('formal', '?')} |",
        f"| Nav markdown (README/INDEX/CHANGELOG) | {extra.get('nav', '?')} |",
        f"| Raw assets (`corpus/**/raw/**`) | {extra.get('raw', '?')} |",
        f"| Indexes | {indexes.detail} |",
    ]
    return "\n".join(rows)


def render_retrieval(recall: Optional[GateOutcome]) -> str:
    if recall is None or not recall.extra.get("summary"):
        body = recall.detail if recall else "not run"
        return f"Status: **FAIL / skipped** — {body}"
    s = recall.extra["summary"]
    status = "PASS" if recall.ok else "FAIL"
    return "\n".join(
        [
            f"Status: **{status}** (gate Recall@5 ≥ {RECALL_GATE})",
            "",
            "| Metric | Value |",
            "|--------|-------|",
            f"| Recall@5 | {float(s.get('recall_at_k', 0)):.3f} |",
            f"| MRR@5 | {float(s.get('mrr_at_k', 0)):.3f} |",
            f"| NDCG@5 | {float(s.get('ndcg_at_k', 0)):.3f} |",
            f"| n_queries | {s.get('n_queries')} |",
            f"| k | {s.get('k')} |",
            f"| mode | {s.get('mode')} |",
            f"| index | `{recall.extra.get('index', '')}` |",
            f"| flags | `--offline --no-rerank --json` |",
        ]
    )


def merge_high_priority(existing: str, l1_lines: List[str]) -> str:
    kept = [
        ln
        for ln in existing.splitlines()
        if ln.strip() and not ln.strip().startswith(L1_PREFIX.strip())
    ]
    # drop the placeholder empty line
    kept = [ln for ln in kept if ln.strip() != "- (empty — L1 `--full` prepends `- [L1]` rows for eval misses or gate failures)"]
    merged = l1_lines + kept
    return "\n".join(merged) if merged else "- (empty)"


def gate_state(
    mode: str,
    outcomes: List[GateOutcome],
    corpus: GateOutcome,
    indexes: GateOutcome,
    recall: Optional[GateOutcome],
) -> GateOutcome:
    if STATE_PATH.is_file():
        prev = split_state(STATE_PATH.read_text(encoding="utf-8"))
    else:
        prev = {h: "" for h in HEADINGS}

    l1_lines: List[str] = []
    for g in outcomes:
        if not g.ok:
            l1_lines.append(f"{L1_PREFIX}{g.name} failed: {g.detail.splitlines()[0]}")
    if recall is not None:
        for miss in recall.extra.get("misses") or []:
            q = str(miss.get("query", ""))[:80]
            rec = miss.get("recall_at_k")
            l1_lines.append(f"{L1_PREFIX}eval miss recall={rec}: {q}")

    body = [
        "# Loop State — mao_agent",
        "",
        f"Last run: {utc_now()} ({mode})",
        "Mode: report-only L1",
        "",
        f"## {HEADINGS[0]}",
        "",
        render_snapshot(corpus, indexes),
        "",
        f"## {HEADINGS[1]}",
        "",
        render_retrieval(recall),
        "",
        f"## {HEADINGS[2]}",
        "",
        merge_high_priority(prev.get(HEADINGS[2], ""), l1_lines),
        "",
        f"## {HEADINGS[3]}",
        "",
        prev.get(HEADINGS[3], "").strip() or "- (empty)",
        "",
        f"## {HEADINGS[4]}",
        "",
        prev.get(HEADINGS[4], "").strip() or "- (empty)",
        "",
    ]
    try:
        atomic_write_state("\n".join(body))
    except OSError as e:
        return GateOutcome("6 STATE.md", False, f"atomic write failed: {e}")
    text = STATE_PATH.read_text(encoding="utf-8")
    if "Last run:" not in text or HEADINGS[0] not in text:
        return GateOutcome("6 STATE.md", False, "STATE.md truncated or missing headings")
    return GateOutcome("6 STATE.md", True, f"atomic replace {STATE_PATH.name}")


def main(argv: Optional[Sequence[str]] = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check-integrity",
        action="store_true",
        help="gates 1–3 (compile, corpus/raw, indexes)",
    )
    parser.add_argument(
        "--full",
        action="store_true",
        help="gates 1–6 including eval-retrieval, hard-negatives, STATE.md",
    )
    args = parser.parse_args(argv)
    if not args.check_integrity and not args.full:
        parser.print_usage(sys.stderr)
        sys.stderr.write("error: pass --check-integrity or --full\n")
        return 1
    if pause_requested():
        sys.stderr.write("MAO_LOOP_PAUSE is set; L1 inspect escalating\n")
        return 2

    mode = "full" if args.full else "check-integrity"
    outcomes: List[GateOutcome] = []
    def note(g: GateOutcome) -> GateOutcome:
        flag = "PASS" if g.ok else "FAIL"
        print(f"  Gate {g.name}: {flag} - {g.detail.splitlines()[0]}", flush=True)
        return g

    print("L1 inspect (report-only)", flush=True)
    g1 = note(gate_compile())
    outcomes.append(g1)
    g2 = note(gate_corpus())
    outcomes.append(g2)
    g3 = note(gate_indexes())
    outcomes.append(g3)

    g4: Optional[GateOutcome] = None
    if args.full:
        g4 = note(gate_recall())
        outcomes.append(g4)
        outcomes.append(note(gate_hard_negative()))
        outcomes.append(note(gate_state(mode, outcomes, g2, g3, g4)))

    return 0 if all(g.ok for g in outcomes) else 1


if __name__ == "__main__":
    raise SystemExit(main())
