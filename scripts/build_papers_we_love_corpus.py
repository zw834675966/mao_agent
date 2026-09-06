#!/usr/bin/env python3
"""
Pipeline script to build, transform, and structure Papers We Love materials
into mao_agent corpus conventions.
"""

import os
import re
import shutil
from pathlib import Path
from typing import Dict, List, Tuple

ROOT_DIR = Path(__file__).resolve().parent.parent
CORPUS_DIR = ROOT_DIR / "corpus"
PWL_DIR = CORPUS_DIR / "papers_we_love"
RAW_DIR = PWL_DIR / "raw"

SCRATCH_DIR = Path(r"C:\Users\Administrator\.gemini\antigravity-cli\brain\a68b3a91-6c07-4ff5-a045-556bda873329\scratch\papers-we-love")

CATEGORY_ZH = {
    "distributed_systems": "分布式系统 (Distributed Systems)",
    "concurrency": "并发与多线程 (Concurrency)",
    "operating_systems": "操作系统与内核 (Operating Systems)",
    "datastores": "存储引擎与数据库 (Datastores)",
    "caching": "缓存系统 (Caching)",
    "data_structures": "核心数据结构 (Data Structures)",
    "algorithms": "经典算法 (Algorithms)",
    "computer_architecture": "计算机体系结构 (Computer Architecture)",
    "networks": "计算机网络与通信协议 (Networks)",
    "security": "系统安全与密码学 (Security)",
    "cryptography": "现代密码学理论 (Cryptography)",
    "machine_learning": "机器学习与统计推断 (Machine Learning)",
    "artificial_intelligence": "人工智能与知识表示 (Artificial Intelligence)",
    "information_retrieval": "信息检索与全文搜索 (Information Retrieval)",
    "comp_sci_fundamentals_and_history": "计算科学基础与理论历史 (Fundamentals & History)",
    "garbage_collection": "垃圾回收与内存管理 (Garbage Collection)",
    "non_blocking_algorithms": "无锁与非阻塞算法 (Non-blocking Algorithms)",
    "faults_and_verification": "形式化验证与容错系统 (Faults & Verification)",
    "languages": "编程语言设计 (Programming Languages)",
    "languages-theory": "程序语言理论与类型系统 (Type Theory)",
    "data_compression": "数据压缩理论 (Data Compression)",
    "virtual_machines": "虚拟机与运行时系统 (Virtual Machines)",
    "software_engineering_orgs": "软件工程组织与协作 (Software Engineering Organizations)",
    "privacy": "隐私保护与匿名通信 (Privacy)",
    "testing": "软件测试与质量保证 (Testing)",
    "quantum_computing": "量子计算原理 (Quantum Computing)",
}


def main():
    print("🚀 Initializing Papers We Love material building pipeline...")
    PWL_DIR.mkdir(parents=True, exist_ok=True)
    RAW_DIR.mkdir(parents=True, exist_ok=True)

    if not SCRATCH_DIR.exists():
        print(f"❌ Error: {SCRATCH_DIR} not found.")
        return

    # 1. Copy raw repository contents
    topic_dirs = [d for d in SCRATCH_DIR.iterdir() if d.is_dir() and not d.name.startswith(".")]
    print(f"📦 Copying {len(topic_dirs)} raw topic categories into {RAW_DIR}...")
    for d in topic_dirs:
        target_topic = RAW_DIR / d.name
        if not target_topic.exists():
            shutil.copytree(d, target_topic)

    # Copy top-level README
    if (SCRATCH_DIR / "README.md").exists():
        shutil.copy2(SCRATCH_DIR / "README.md", RAW_DIR / "README.md")

    # 2. Generate structured markdown documents for each category
    category_summaries = []
    total_docs = 0

    for d in sorted(topic_dirs):
        cat_key = d.name
        cat_title = CATEGORY_ZH.get(cat_key, cat_key.replace("_", " ").title())
        pdfs = list((RAW_DIR / cat_key).glob("*.pdf"))
        readme_file = RAW_DIR / cat_key / "README.md"
        readme_content = readme_file.read_text(encoding="utf-8", errors="ignore") if readme_file.exists() else ""

        # Format clean markdown with YAML frontmatter
        tags = ["Papers We Love", "计算机经典论文", cat_title.split(" (")[0]]
        tags_yaml = "\n".join(f'  - "{t}"' for t in tags)

        pdf_list_md = ""
        if pdfs:
            pdf_list_md = "\n### 本地归档核心学术论文（PDF）\n\n"
            for pdf in sorted(pdfs):
                paper_name = pdf.stem.replace("-", " ").replace("_", " ").title()
                rel_pdf = f"./raw/{cat_key}/{pdf.name}"
                size_mb = pdf.stat().st_size / (1024 * 1024)
                pdf_list_md += f"- **[{paper_name}]({rel_pdf})** `({size_mb:.2f} MB)`\n"

        doc_md = f"""---
title: "经典论文导读: {cat_title}"
author: "Papers We Love 计算机科学学术共同体"
date: "2024"
period: "现代计算机科学"
volume: "计算机科学传世经典论文集 (Papers We Love)"
category: "{cat_title}"
source: "https://github.com/papers-we-love/papers-we-love"
tags:
{tags_yaml}
---

〔本篇为 Papers We Love 经典学术文献库关于“{cat_title}”领域收录的传世奠基论文全景导读与本地文献档案。〕

# 一、 领域学术导读与背景

{readme_content.strip()}

# 二、 核心学术论文与本地文献原件

{pdf_list_md}

---
_本地归档路径: `corpus/papers_we_love/raw/{cat_key}/`_
"""
        out_file = PWL_DIR / f"papers_{cat_key}.md"
        out_file.write_text(doc_md, encoding="utf-8")
        total_docs += 1
        category_summaries.append((cat_key, cat_title, len(pdfs)))

    # 3. Create canonical collection document
    canonical_text = """---
title: "计算机科学传世经典论文总汇编 (Papers We Love Canonical Anthology)"
author: "Papers We Love 计算机科学学术共同体"
date: "2024"
period: "现代计算机科学"
volume: "计算机科学传世经典论文集 (Papers We Love)"
category: "传世经典论文总集"
source: "https://github.com/papers-we-love/papers-we-love"
tags:
  - "Papers We Love"
  - "计算机科学经典"
  - "分布式系统"
  - "操作系统"
  - "体系结构"
  - "机器学习"
---

〔Papers We Love 是全球计算机科学领域最受推崇的学术论文共享与研讨项目，精选了计算机科学半个多世纪以来最重要的开创性论文。〕

# 一、 核心学科领域分类索引

"""
    for i, (cat_key, cat_title, pdf_cnt) in enumerate(category_summaries, 1):
        canonical_text += f"\n### {i:02d}. [{cat_title}](./papers_{cat_key}.md) `({pdf_cnt} 篇经典论文)`\n"

    (PWL_DIR / "papers_canonical_collection.md").write_text(canonical_text, encoding="utf-8")
    print(f"✅ Generated {total_docs} category documents in {PWL_DIR}")
    print("✅ Emitted papers_canonical_collection.md")
    print(f"\n🎉 Successfully structured all Papers We Love materials into {PWL_DIR.resolve()}!")


if __name__ == "__main__":
    main()
