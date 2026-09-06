#!/usr/bin/env python3
"""
Pipeline script to build, transform, and structure Awesome Scalability materials
into mao_agent corpus conventions.
"""

import os
import re
import shutil
from pathlib import Path
from typing import Dict, List, Tuple

ROOT_DIR = Path(__file__).resolve().parent.parent
CORPUS_DIR = ROOT_DIR / "corpus"
SCALABILITY_DIR = CORPUS_DIR / "awesome_scalability"
RAW_DIR = SCALABILITY_DIR / "raw"

SCRATCH_DIR = Path(r"C:\Users\Administrator\.gemini\antigravity-cli\brain\a68b3a91-6c07-4ff5-a045-556bda873329\scratch\awesome-scalability")

SECTION_META = {
    "Principle": {
        "filename": "scalability_01_principles.md",
        "title": "高并发系统设计基本原则与权衡模型 (Scalability Principles)",
        "author": "Binh Nguyen 与全球架构师社区",
        "category": "架构设计原则与理论模型",
        "tags": ["高并发原则", "CAP定理", "PACELC", "设计权衡", "可扩展性"],
        "headnote": "〔本篇梳理构建可扩展、高可靠与高性能大规模分布式系统的核心理论原则与设计权衡模式。〕",
    },
    "Scalability": {
        "filename": "scalability_02_scalability.md",
        "title": "大规模系统水平与垂直扩展架构实践 (System Scalability)",
        "author": "Binh Nguyen 与全球架构师社区",
        "category": "系统扩展与负载均衡",
        "tags": ["水平扩展", "无状态计算", "分库分表", "分布式缓存", "异步队列"],
        "headnote": "〔本篇深入探讨服务器无状态克隆、数据库分片复制、分布式多级缓存与异步解耦等水平扩展核心手段。〕",
    },
    "Availability": {
        "filename": "scalability_03_availability.md",
        "title": "高可用架构、故障转移与数据复制 (System Availability)",
        "author": "Binh Nguyen 与全球架构师社区",
        "category": "高可用与容灾架构",
        "tags": ["高可用性", "故障转移", "主从复制", "最终一致性", "共识算法"],
        "headnote": "〔本篇阐述高可用系统的关键指标、故障自动转移（Fail-over）、冗余备份与复制一致性权衡。〕",
    },
    "Stability": {
        "filename": "scalability_04_stability.md",
        "title": "分布式系统稳定性治理与容错防护 (System Stability)",
        "author": "Binh Nguyen 与全球架构师社区",
        "category": "系统稳定性与容灾保护",
        "tags": ["稳定性治理", "熔断器", "限流降级", "舱壁隔离", "接口幂等性"],
        "headnote": "〔本篇介绍服务雪崩防护、超时重试、熔断器（Circuit Breaker）、速率限制与故障隔离技术方案。〕",
    },
    "Performance": {
        "filename": "scalability_05_performance.md",
        "title": "低延迟高性能计算与系统调优 (System Performance)",
        "author": "Binh Nguyen 与全球架构师社区",
        "category": "性能工程与延迟优化",
        "tags": ["性能调优", "延迟与吞吐量", "多级缓存", "索引优化", "网络协议栈"],
        "headnote": "〔本篇从吞吐量、P99延迟、网络通信、存储I/O到应用代码层，全方位梳理高性能系统的调优路径。〕",
    },
    "Intelligence": {
        "filename": "scalability_06_intelligence.md",
        "title": "海量数据处理、实时流计算与搜索引擎 (Data Intelligence)",
        "author": "Binh Nguyen 与全球架构师社区",
        "category": "大数据与实时计算架构",
        "tags": ["海量数据", "流计算", "全文检索", "数据仓库", "分布式计算引擎"],
        "headnote": "〔本篇系统性总结现代搜索引擎、MapReduce分布式计算、流式计算与现代湖仓一体化智能架构。〕",
    },
    "Architecture": {
        "filename": "scalability_07_architecture.md",
        "title": "现代分布式系统架构风格与演进 (System Architecture)",
        "author": "Binh Nguyen 与全球架构师社区",
        "category": "软件架构模式与演进",
        "tags": ["微服务架构", "事件驱动架构", "CQRS", "六边形架构", "Serverless"],
        "headnote": "〔本篇对比微服务、事件驱动、CQRS、领域驱动设计（DDD）与面向服务架构的适用边界与演进逻辑。〕",
    },
    "Interview": {
        "filename": "scalability_08_interviews.md",
        "title": "大规模分布式系统设计面试与分析框架 (System Design Interview)",
        "author": "Binh Nguyen 与全球架构师社区",
        "category": "系统设计与架构面试",
        "tags": ["系统设计面试", "估算能力", "架构推演", "设计框架", "经典题解"],
        "headnote": "〔本篇提供大规模系统设计的结构化分析框架，涵盖需求澄清、容量估算、高层架构设计与深层瓶颈剖析。〕",
    },
    "Organization": {
        "filename": "scalability_09_organization.md",
        "title": "工程组织架构、康威定律与敏捷工程文化 (Engineering Organization)",
        "author": "Binh Nguyen 与全球架构师社区",
        "category": "团队管理与工程文化",
        "tags": ["组织架构", "康威定律", "两张披萨团队", "敏捷交付", "工程师文化"],
        "headnote": "〔本篇探讨软件架构与技术团队组织的深层互动关系，阐述两张披萨团队、敏捷文化与工程组织效率。〕",
    },
    "Talk": {
        "filename": "scalability_10_talks.md",
        "title": "全球顶尖科技公司经典分布式架构演讲录 (Architecture Talks)",
        "author": "Binh Nguyen 与全球架构师社区",
        "category": "工业界经典架构案例",
        "tags": ["架构演讲", "工业级案例", "Google架构", "Netflix架构", "Twitter架构"],
        "headnote": "〔本篇收录Google、Netflix、Amazon、Twitter等互联网巨头在大型系统实战演进中的传奇演讲与经验总结。〕",
    },
}


def build_markdown_document(sec_name: str, meta: dict, body: str) -> str:
    tags_yaml = "\n".join(f'  - "{t}"' for t in meta["tags"])
    # Convert github headings
    cleaned_body = re.sub(r"^### (.*)$", r"## \1", body, flags=re.MULTILINE)

    return f"""---
title: "{meta['title']}"
author: "{meta['author']}"
date: "2024"
period: "现代软件工程"
volume: "分布式系统与高并发架构实战文库"
category: "{meta['category']}"
source: "https://github.com/binhnguyennus/awesome-scalability"
tags:
{tags_yaml}
---

{meta['headnote']}

# 一、 架构模块全景阐述

{cleaned_body.strip()}

# 二、 文献引文与出处来源

- **知识库出处**: [binhnguyennus/awesome-scalability (GitHub)](https://github.com/binhnguyennus/awesome-scalability)
- **专题分类**: 分布式系统与高并发架构实战文库 · {meta['category']}
"""


def main():
    print("🚀 Initializing Awesome Scalability material building pipeline...")
    SCALABILITY_DIR.mkdir(parents=True, exist_ok=True)
    RAW_DIR.mkdir(parents=True, exist_ok=True)

    readme_path = SCRATCH_DIR / "README.md"
    if not readme_path.exists():
        print(f"❌ Error: {readme_path} not found.")
        return

    # Copy raw assets
    shutil.copy2(readme_path, RAW_DIR / "README.md")
    logo_path = SCRATCH_DIR / "logo.png"
    if logo_path.exists():
        shutil.copy2(logo_path, RAW_DIR / "logo.png")
        shutil.copy2(logo_path, SCALABILITY_DIR / "logo.png")

    text = readme_path.read_text(encoding="utf-8")
    sections = re.split(r"\n(?=## )", text)

    doc_entries = []
    for s in sections:
        s_strip = s.strip()
        m = re.match(r"^##\s+([A-Za-z0-9_\- ]+)", s_strip)
        if not m:
            continue
        sec_name = m.group(1).strip()
        if sec_name in SECTION_META:
            meta = SECTION_META[sec_name]
            # Strip the '## SecName' header
            body = re.sub(r"^##\s+[^\n]+\n", "", s_strip).strip()
            doc_content = build_markdown_document(sec_name, meta, body)
            target_file = SCALABILITY_DIR / meta["filename"]
            target_file.write_text(doc_content, encoding="utf-8")
            print(f"  ✅ Emitted {meta['filename']} ({len(doc_content)} chars)")
            doc_entries.append((meta["title"], meta["headnote"], body))

    # Build canonical collection document
    canonical_text = """---
title: "大规模分布式系统架构与高并发工程权威全景指南 (Awesome Scalability Playbook)"
author: "Binh Nguyen 与全球架构师社区"
date: "2024"
period: "现代软件工程"
volume: "分布式系统与高并发架构实战文库"
category: "高并发分布式架构全书"
source: "https://github.com/binhnguyennus/awesome-scalability"
tags:
  - "大规模分布式系统"
  - "系统设计大全"
  - "高可用架构"
  - "高并发系统"
  - "性能工程"
---

〔本篇为现代互联网与云原生大规模高可用、高并发与高稳定性分布式系统架构演进模式全景合辑。〕

"""
    for i, (title, note, body) in enumerate(doc_entries, 1):
        canonical_text += f"\n# {i}、 {title}\n\n{note}\n\n{body}\n\n---\n"

    (SCALABILITY_DIR / "awesome_scalability_canonical_playbook.md").write_text(canonical_text, encoding="utf-8")
    print(f"  ✅ Emitted awesome_scalability_canonical_playbook.md")

    print(f"\n🎉 Successfully structured all Awesome Scalability materials into {SCALABILITY_DIR.resolve()}!")


if __name__ == "__main__":
    main()
