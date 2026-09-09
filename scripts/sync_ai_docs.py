#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""AI 文档同步器（[C-16]）。

维护 src/**/*.rs 与 docs/ai/src/**/*.md 的 1:1 镜像覆盖矩阵，
并幂等地重建索引 docs/ai/README.md。

用法:
  python scripts/sync_ai_docs.py --list    # 打印覆盖差异表（不写文件）
  python scripts/sync_ai_docs.py --check   # 校验 1:1 覆盖，缺失则退出码 1
  python scripts/sync_ai_docs.py           # 重建 docs/ai/README.md 索引并打印摘要

Python 3 标准库实现，无第三方依赖。
"""
from __future__ import annotations

import argparse
import datetime as _dt
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "src"
DOCS = ROOT / "docs" / "ai"
INDEX = DOCS / "README.md"
CONSTRAINTS = DOCS / "CONSTRAINTS.md"

# 根级文件映射: src/x.rs -> docs/ai/src/x.md（其余为同名 .md）
ROOT_LEVEL = {"lib.rs", "main.rs", "config.rs", "error.rs", "model.rs", "retry.rs"}

REQUIRED_SECTIONS = [
    "## 1. 一句话职责",
    "## 2. 对外接口（AI 调用面）",
    "## 3. 内部结构",
    "## 4. 数据流与调用链",
    "## 5. 硬约束与陷阱（必须读）",
    "## 6. 测试锚点",
]


def doc_path_for(rs: Path) -> Path:
    rel = rs.relative_to(SRC)
    if rel.name in ROOT_LEVEL and rel.parent == Path("."):
        return DOCS / "src" / (rel.stem + ".md")
    return DOCS / "src" / rel.with_suffix(".md")


def scan() -> list[dict]:
    """返回每个源文件的覆盖记录。"""
    records = []
    for rs in sorted(SRC.rglob("*.rs")):
        dp = doc_path_for(rs)
        status = "missing"
        stale = False
        sections_ok = False
        if dp.exists():
            status = "ok"
            stale = dp.stat().st_mtime < rs.stat().st_mtime
            text = dp.read_text(encoding="utf-8", errors="replace")
            sections_ok = all(s in text for s in REQUIRED_SECTIONS)
            if stale:
                status = "stale"
            if not sections_ok:
                status = "invalid"
        records.append(
            {
                "src": rs.relative_to(ROOT).as_posix(),
                "doc": dp.relative_to(ROOT).as_posix(),
                "status": status,
            }
        )
    return records


def orphans() -> list[str]:
    """docs/ai/src 下没有对应源文件的孤儿文档。"""
    out = []
    for md in sorted((DOCS / "src").rglob("*.md")):
        rel = md.relative_to(DOCS / "src")
        if rel.stem in {Path(p).stem for p in ROOT_LEVEL} and rel.parent == Path("."):
            src = SRC / (rel.stem + ".rs")
        else:
            src = SRC / rel.with_suffix(".rs")
        if not src.exists():
            out.append(md.relative_to(ROOT).as_posix())
    return out


def render_index(records: list[dict], orphan_list: list[str]) -> str:
    now = _dt.datetime.now().strftime("%Y-%m-%d %H:%M")
    ok = sum(1 for r in records if r["status"] == "ok")
    missing = [r for r in records if r["status"] == "missing"]
    stale = [r for r in records if r["status"] == "stale"]
    invalid = [r for r in records if r["status"] == "invalid"]
    lines = [
        "# docs/ai 索引（AI 阅读资料同步总览）",
        "",
        f"> 由 `scripts/sync_ai_docs.py` 于 {now} 自动生成，禁止手工编辑。",
        "> 跨模块硬约束见 [CONSTRAINTS.md](CONSTRAINTS.md)（`[C-xx]` 编号权威来源）。",
        "> 生成规范技能：`.agents/skills/ai-codeblock-docs/SKILL.md`。",
        "",
        "## 覆盖矩阵",
        "",
        "| 状态 | 数量 |",
        "| --- | --- |",
        f"| ok（模板完整且新于源码） | {ok} |",
        f"| stale（源码新于文档，需重生成） | {len(stale)} |",
        f"| invalid（章节缺失，不符模板） | {len(invalid)} |",
        f"| missing（缺文档） | {len(missing)} |",
        f"| 孤儿文档（无对应源码） | {len(orphan_list)} |",
        "",
        "## 单元文档清单",
        "",
        "| 源文件 | AI 文档 | 状态 |",
        "| --- | --- | --- |",
    ]
    for r in records:
        rel_link = r["doc"][len("docs/ai/"):]
        lines.append(f"| `{r['src']}` | [{Path(rel_link).name}]({rel_link}) | {r['status']} |")
    if orphan_list:
        lines += ["", "## 孤儿文档（应删除或恢复源码）", ""]
        lines += [f"- `{p}`" for p in orphan_list]
    if missing:
        lines += ["", "## 待生成队列", ""]
        lines += [f"- `{r['src']}`" for r in missing]
    lines.append("")
    return "\n".join(lines)


def main() -> int:
    ap = argparse.ArgumentParser(description="AI 文档同步器")
    ap.add_argument("--list", action="store_true", help="打印覆盖差异表")
    ap.add_argument("--check", action="store_true", help="校验 1:1 覆盖（CI 门禁）")
    args = ap.parse_args()

    records = scan()
    orphan_list = orphans()

    if args.list:
        for r in records:
            print(f"{r['status']:<8} {r['src']} -> {r['doc']}")
        for p in orphan_list:
            print(f"orphan   {p}")
        return 0

    if args.check:
        bad = [r for r in records if r["status"] != "ok"]
        for r in bad:
            print(f"FAIL {r['status']:<8} {r['src']} ({r['doc']})")
        for p in orphan_list:
            print(f"FAIL orphan   {p}")
        if bad or orphan_list:
            print(f"check failed: {len(bad)} bad / {len(orphan_list)} orphans", file=sys.stderr)
            return 1
        print(f"check ok: {len(records)} units fully documented")
        return 0

    INDEX.parent.mkdir(parents=True, exist_ok=True)
    INDEX.write_text(render_index(records, orphan_list), encoding="utf-8")
    ok = sum(1 for r in records if r["status"] == "ok")
    bad = len(records) - ok
    print(f"index rebuilt: {INDEX.relative_to(ROOT)} ({ok} ok, {bad} not-ok, {len(orphan_list)} orphans)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
