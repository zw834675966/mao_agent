#!/usr/bin/env python3
"""
Pipeline script to build, transform, and structure Hello Algo materials
into mao_agent corpus conventions.
"""

import os
import re
import shutil
from pathlib import Path
from typing import Dict, List, Tuple

ROOT_DIR = Path(__file__).resolve().parent.parent
CORPUS_DIR = ROOT_DIR / "corpus"
HELLO_ALGO_DIR = CORPUS_DIR / "hello_algo"
RAW_DIR = HELLO_ALGO_DIR / "raw"

SCRATCH_DIR = Path(r"C:\Users\Administrator\.gemini\antigravity-cli\brain\a68b3a91-6c07-4ff5-a045-556bda873329\scratch\hello-algo")

CHAPTER_NAMES = {
    "chapter_introduction": "初识算法与编程基础",
    "chapter_computational_complexity": "算法复杂度分析",
    "chapter_data_structure": "数据结构概览与逻辑分类",
    "chapter_array_and_linkedlist": "数组与链表",
    "chapter_stack_and_queue": "栈与队列",
    "chapter_hashing": "哈希表与散列冲突",
    "chapter_tree": "树与二叉树",
    "chapter_heap": "堆与优先队列",
    "chapter_graph": "图与图遍历算法",
    "chapter_searching": "搜索算法",
    "chapter_sorting": "排序算法",
    "chapter_divide_and_conquer": "分治算法",
    "chapter_backtracking": "回溯算法",
    "chapter_dynamic_programming": "动态规划",
    "chapter_greedy": "贪心算法",
    "chapter_appendix": "附录与术语表",
}


def extract_title_from_md(text: str, default: str) -> str:
    m = re.search(r"^#\s+(.+)$", text, re.MULTILINE)
    return m.group(1).strip() if m else default


def build_markdown_document(rel_path: Path, raw_text: str) -> str:
    parent_dir = rel_path.parent.name
    category_name = CHAPTER_NAMES.get(parent_dir, "数据结构与算法")
    stem = rel_path.stem

    # Extract H1 title
    raw_title = extract_title_from_md(raw_text, stem)
    full_title = f"{category_name} · {raw_title}"

    # Extract tags
    tags = ["Hello算法", "数据结构与算法", category_name]
    if raw_title not in tags:
        tags.append(raw_title)

    tags_yaml = "\n".join(f'  - "{t}"' for t in tags)

    # Clean text to ensure proper headnote
    cleaned_body = raw_text.strip()

    return f"""---
title: "{full_title}"
author: "靳宇栋 (krahets) 与 Hello 算法开源社区"
date: "2023"
period: "现代计算机科学"
volume: "Hello 算法：图解数据结构与算法文库"
category: "{category_name}"
source: "https://github.com/krahets/hello-algo"
tags:
{tags_yaml}
---

〔本篇为经典开源图解教程《Hello 算法》中关于“{category_name} · {raw_title}”的完整文献与图文解析。〕

{cleaned_body}

---
_来源文献: [krahets/hello-algo (GitHub)](https://github.com/krahets/hello-algo) · 章节: {rel_path}_
"""


def main():
    print("🚀 Initializing Hello Algo material building pipeline...")
    HELLO_ALGO_DIR.mkdir(parents=True, exist_ok=True)
    RAW_DIR.mkdir(parents=True, exist_ok=True)

    docs_dir = SCRATCH_DIR / "docs"
    if not docs_dir.exists():
        print(f"❌ Error: {docs_dir} not found.")
        return

    # Copy raw upstream docs
    raw_docs_target = RAW_DIR / "docs"
    if not raw_docs_target.exists():
        shutil.copytree(docs_dir, raw_docs_target)
        print(f"  [Raw Docs] Preserved raw documentation tree in {raw_docs_target}")

    # Copy assets for illustrations
    assets_src = docs_dir / "assets"
    assets_dst = HELLO_ALGO_DIR / "assets"
    if assets_src.exists() and not assets_dst.exists():
        shutil.copytree(assets_src, assets_dst)
        print(f"  [Assets] Copied diagrams and animations into {assets_dst}")

    # Process all Markdown chapter files
    count = 0
    all_chapters_summary = []
    for chapter_dir in sorted(docs_dir.iterdir()):
        if chapter_dir.is_dir() and chapter_dir.name.startswith("chapter_"):
            target_chapter_dir = HELLO_ALGO_DIR / chapter_dir.name
            target_chapter_dir.mkdir(parents=True, exist_ok=True)

            # Copy any chapter local asset folders
            for sub in chapter_dir.iterdir():
                if sub.is_dir():
                    dst_sub = target_chapter_dir / sub.name
                    if not dst_sub.exists():
                        shutil.copytree(sub, dst_sub)

            # Process md files
            for md_file in sorted(chapter_dir.glob("*.md")):
                raw_text = md_file.read_text(encoding="utf-8")
                rel_path = md_file.relative_to(docs_dir)
                doc_text = build_markdown_document(rel_path, raw_text)

                out_path = target_chapter_dir / md_file.name
                out_path.write_text(doc_text, encoding="utf-8")
                count += 1

            cat_title = CHAPTER_NAMES.get(chapter_dir.name, chapter_dir.name)
            all_chapters_summary.append((chapter_dir.name, cat_title))

    # Build canonical course syllabus document
    canonical_text = """---
title: "Hello 算法：数据结构与算法权威知识体系总览"
author: "靳宇栋 (krahets) 与 Hello 算法开源社区"
date: "2023"
period: "现代计算机科学"
volume: "Hello 算法：图解数据结构与算法文库"
category: "数据结构与算法全书"
source: "https://github.com/krahets/hello-algo"
tags:
  - "Hello算法"
  - "算法大全"
  - "数据结构全景"
  - "复杂度分析"
  - "程序员算法必备"
---

〔《Hello 算法》是一本面向所有开发者的开源、全彩、动画图解数据结构与算法教程，涵盖计算机科学最核心的结构体系与算法设计范式。〕

# 一、 全书体系架构导航

"""
    for i, (dir_name, cat_title) in enumerate(all_chapters_summary, 1):
        canonical_text += f"\n### {i}. [{cat_title}](./{dir_name}/index.md)\n"
        ch_dir = HELLO_ALGO_DIR / dir_name
        for mf in sorted(ch_dir.glob("*.md")):
            if mf.name != "index.md":
                canonical_text += f"- [{mf.stem}](./{dir_name}/{mf.name})\n"

    (HELLO_ALGO_DIR / "hello_algo_canonical_guide.md").write_text(canonical_text, encoding="utf-8")

    print(f"✅ Generated {count} structured markdown documents in {HELLO_ALGO_DIR.resolve()}")
    print("✅ Emitted hello_algo_canonical_guide.md")
    print(f"\n🎉 Successfully structured all Hello Algo materials into {HELLO_ALGO_DIR.resolve()}!")


if __name__ == "__main__":
    main()
