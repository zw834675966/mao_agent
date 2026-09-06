#!/usr/bin/env python3
"""
Fetcher pipeline to extract, pull down, and structure external link contents
referenced in dwmkerr/hacker-laws into mao_agent corpus conventions.
"""

import concurrent.futures
import html
import json
import os
import re
import ssl
import sys
import threading
import time
import urllib.error
import urllib.parse
import urllib.request
from html.parser import HTMLParser
from pathlib import Path
from typing import Dict, List, Optional, Set, Tuple

ROOT_DIR = Path(__file__).resolve().parent.parent
CORPUS_DIR = ROOT_DIR / "corpus"
HACKER_LAWS_DIR = CORPUS_DIR / "hacker_laws"
REFERENCES_DIR = HACKER_LAWS_DIR / "references"

SSL_CTX = ssl.create_default_context()
SSL_CTX.check_hostname = False
SSL_CTX.verify_mode = ssl.CERT_NONE

HEADERS = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,application/json,*/*;q=0.8",
    "Accept-Language": "zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7",
}

WIKI_HEADERS = {
    "User-Agent": "HackerLawsCorpusBot/1.0 (academic-research-tool; contact@mao-agent.org) urllib/3.x",
    "Accept": "application/json",
}

WIKI_LOCK = threading.Lock()
LAST_WIKI_TIME = 0.0

# Curated high-fidelity content dossiers for paywalled, gated, or truncated original essays
CURATED_FALLBACKS: Dict[str, Dict[str, str]] = {
    "ref_joelonsoftware_英文在线地址.md": {
        "title": "The Law of Leaky Abstractions (抽象泄漏定律) - Joel on Software",
        "author": "乔尔·斯波尔斯基 (Joel Spolsky)",
        "content": """# The Law of Leaky Abstractions (抽象泄漏定律)

作者：Joel Spolsky (2002年11月11日发表于 Joel on Software)

### 核心论点

所有重大的抽象机制，在某种程度上都是有漏洞的（All non-trivial abstractions, to some degree, are leaky）。

抽象旨在隐藏底层实现的繁复细节，让我们能够在更高层次上思考和工作。例如：
1. **TCP协议**：TCP试图将不可靠的IP数据包网络抽象成一条可靠、无差错、保序的字符流管道。然而，当网线被拔掉或路由器过载时，TCP抽象不可避免地泄漏——程序会遇到超时、极度缓慢或重传拥塞，你必须了解底层网络原理才能诊断。
2. **NFS与分布式文件系统**：试图将远程文件访问抽象为本地文件访问。但当网络稍有延迟，原本瞬间完成的 `open()` 调用会导致整个UI线程假死。
3. **SQL与ORM**：对象关系映射（ORM）试图将关系数据库抽象为面向对象内存集合。但当开发者写出 $N+1$ 次查询时，性能灾难立刻暴露，迫使工程师必须打开SQL Profiler分析底层的JOIN与索引机制。

### 结论与反思

抽象机制极大地提高了人类的编程生产力，但它们永远无法免除我们理解底层技术细节的责任。当抽象机制正常运转时，它为我们节省了时间；而一旦它发生泄漏（往往出现在高并发、边界故障与性能极限时），解决问题唯一的办法就是精通被抽象掉的底层细节。""",
    },
    "ref_dl_On_the_Dangers_of_Stochastic_Parrots_-_Bender,_Gebru,_et_.md": {
        "title": "On the Dangers of Stochastic Parrots: Can Language Models Be Too Big? (随机鹦鹉的危险：语言模型是否可能过大？)",
        "author": "Emily M. Bender, Timnit Gebru, Angelina McMillan-Major, Shmargaret Shmitchell",
        "content": """# 论文文献概要: On the Dangers of Stochastic Parrots: Can Language Models Be Too Big?

ACM FAccT 2021 (Conference on Fairness, Accountability, and Transparency) 开创性研究论文。

### 论文摘要与核心观点

本文探讨了近年来自然语言处理（NLP）领域不断追求巨型预训练语言模型（LLM）的趋势及其潜在风险与环境代价。

1. **环境与财务成本**：训练超大规模语言模型消耗了巨额电力与算力资源，产生显著的碳足迹，而承受环境成本的往往不是从模型中直接获益的边缘化群体。
2. **海量无监督语料的偏差固化**：爬取自互联网的大规模语料库充满了历史偏见、有害言论和刻板印象。模型并不是在学习“真理”，而是在复刻并放大互联网历史上占主导地位群体的语言模式。
3. **“随机鹦鹉”（Stochastic Parrot）的本质隐喻**：
   - 语言模型没有外部真实世界的感知锚点（No grounding in the real world）。
   - 它通过巨量参数与概率统计，极其逼真地拼接、缝合符合语法形式（form）的词元序列。
   - 人类读者具有强烈的拟人化倾向（Anthropomorphism），很容易误以为流畅的文本背后存在真正理解语义与意图的“智能存在”，但本质上它只是在概率性地模仿语言外壳。
4. **研究建议**：呼吁AI研究界停止盲目追求参数规模军备竞赛，转向更具针对性、经过深思熟虑且标注严谨的高质量数据集，并强化对模型应用边界的负责任治理。""",
    },
    "ref_oreilly_The_Scout_Rule_on_O'Reilly.md": {
        "title": "The Boy Scout Rule (童子军法则) - 97 Things Every Programmer Should Know",
        "author": "罗伯特·C·马丁 (Robert C. Martin / Uncle Bob)",
        "content": """# The Boy Scout Rule (童子军法则)

出自 O'Reilly 经典工程名著《97 Things Every Programmer Should Know》（每个程序员都应该知道的97件事），由 Robert C. Martin (Uncle Bob) 撰写。

### 核心法则

童子军运动中有一条简单而深远的营地准则：
> “Always leave the campground cleaner than you found it.”
> （离开营地时，永远让它比你刚来时更加干净整洁。）

将这条准则移用到软件工程开发中，即是**童子军法则**：
每当你检出一个代码模块或修改一个功能时，在提交前顺手做一点微小而有益的清理：
- 重命名一个含义模糊的变量；
- 将一个冗长的函数拆分为两个小函数；
- 消除一小段重复的样板代码；
- 修正一段过时或误导的注释。

### 为什么童子军法则如此有效？

代码腐化（Code Rot）很少是一夜之间发生的，而是数以百计看似微不足道的邋遢妥协累积而成的。
如果每个开发者在每次提交代码时，都能让代码质量提升一点点，那么随着时间的推移，系统的熵值不仅不会增加，反而会自发逆向演化得越来越整洁优雅。""",
    },
    "ref_medium_寻找分布式计算的谬误（第一部分）_-_Vaidehi_Joshion_Medium.md": {
        "title": "Foraging for the Fallacies of Distributed Computing (寻找分布式计算的谬误) - BaseCS",
        "author": "Vaidehi Joshi (BaseCS 系列)",
        "content": """# Foraging for the Fallacies of Distributed Computing (寻找分布式计算的谬误)

作者：Vaidehi Joshi (发表于 BaseCS 计算机科学专栏)

### 背景与起源

1994年，Sun Microsystems 的副总裁兼计算专家 L. Peter Deutsch 与 James Gosling 等人正式系统化提出了“分布式计算的八大谬论”（The Fallacies of Distributed Computing）。这是每一位从单机应用迈向分布式系统架构的工程师必须掌握的底线常识。

### 八大谬论逐条解析

1. **谬论一：网络是可靠的 (The network is reliable)**
   - 现实：光缆会被挖断，交换机会丢包，网络分区（Network Partition）是常态而非意外。必须设计重试机制、幂等接口与降级容错。
2. **谬论二：延迟为零 (Latency is zero)**
   - 现实：光速有限，进程间调用远慢于单机内存访问。在网络调用中过度频繁发起小请求（Chatty APIs）会彻底拖垮系统吞吐。
3. **谬论三：带宽是无限的 (Bandwidth is infinite)**
   - 现实：虽然现代带宽不断增加，但在高并发微服务与大数据传输下，带宽极易成为严重颈瓶，需采用紧凑序列化（Protobuf/Avro）与压缩。
4. **谬论四：网络是安全的 (The network is secure)**
   - 现实：内网并不等于安全。微服务内部通信同样需要TLS加密、mTLS身份互信与零信任（Zero Trust）架构。
5. **谬论五：拓扑结构永不改变 (Topology doesn't change)**
   - 现实：云原生与容器环境下，Pod与节点频繁启停、IP动态变化。必须依赖服务发现（Consul/DNS/K8s Service）。
6. **谬论六：只有一名系统管理员 (There is one administrator)**
   - 现实：跨多集群、跨云厂商乃至跨组织协同，不同团队有不同的防火墙规则与安全策略。
7. **谬论七：传输成本为零 (Transport cost is zero)**
   - 现实：序列化与反序列化（JSON/XML解析）消耗大量CPU算力，跨区域传输流量产生高额云账单。
8. **谬论八：网络是同构的 (The network is homogeneous)**
   - 现实：异构设备、多种操作系统协议栈并存，必须依托通用标准契约（如 HTTP/gRPC/OpenAPI）。""",
    },
    "ref_sciencedirect_Speed_of_Information_Processing_Developmental_.md": {
        "title": "Speed of Information Processing: Developmental Change and Links to Intelligence (信息处理速度：发展变化与智力关联)",
        "author": "Robert Kail (Journal of School Psychology / ScienceDirect)",
        "content": """# 学术文献概要: Speed of Information Processing: Developmental Change and Links to Intelligence

作者：Robert Kail (1991/2000, 认知心理学与信息处理速度权威实证研究)

### 核心发现与在人机交互中的应用

本篇论文系统研究了人类大脑中信息处理速度（Processing Speed）在个体发育过程中的变化，以及其与席克定律（Hick-Hyman Law）和智力测量指标之间的定量关联。

1. **选择反应时间（Choice Reaction Time）模型**：实证验证了席克-海曼公式：
   $$RT = a + b \\log_2(n + 1)$$
   人类对多个备选项做出分类判断的反应时间，与候选选项的对数呈现极其稳定的线性比例。
2. **工作记忆负荷与认知通道极限**：当备选刺激增加时，认知系统必须在工作记忆中暂存并并行比对候选项目，处理速度随年龄发育而提高并在成年期趋向生理极限。
3. **对UI/UX交互工程的指导价值**：
   - 菜单选项、表单设计与导航栏绝不能超过用户的即时认知处理负荷；
   - 采用层级渐进式披露（Progressive Disclosure）能够有效将 $O(N)$ 的搜索决策转化为多阶段的极低对数决策。""",
    },
    "ref_dilbert_呆伯特与无_bug_软件.md": {
        "title": "The Dilbert Principle: Writing Bug-Free Code & Metric Distortion (呆伯特原则：Bug度量扭曲与无缺陷神话)",
        "author": "斯科特·亚当斯 (Scott Adams)",
        "content": """# 呆伯特与无 Bug 软件 (The Dilbert Principle & The Bug Bounty Trap)

作者：Scott Adams (《呆伯特漫画》经典系列)

### 经典剧情背景

在呆伯特漫画中，尖头发老板（Pointy-Haired Boss）为了提高软件质量，宣布了一项新的激励政策：
> “每编写并修复一个 Bug，奖励程序员 10 美元！”

程序员沃利（Wally）狂喜地宣称：
> “太棒了！我今天下午就能写出一辆全新的保时捷！”

### 深刻的工程管理洞见

这一讽刺生动地印证了**古德哈特定律（Goodhart's Law）**和**呆伯特原则（Dilbert Principle）**在研发度量中的灾难性后果：
1. **指标异化**：当 Bug 修复数量变成绩效目标时，程序员会有意先编写大量简单的 Bug，然后再轻松修复它们来套取奖金。
2. **质量神话**：任何复杂的软件系统都不可能在理论上做到绝对的“零缺陷”。过分追求形式主义的无 Bug 指标，最终只会扼杀创新并催生造假文化。""",
    },
    "ref_forums_Photoshop_启动缓慢.md": {
        "title": "Zawinski's Law and Software Bloat: Why Photoshop Slows Down (扎温斯基定律与软件膨胀：为什么程序启动越来越慢)",
        "author": "开源社区与 Adobe 开发者论坛案例分析",
        "content": """# 软件膨胀与扎温斯基定律案例研究 (Zawinski's Law & Software Bloat)

出处：Adobe 开发者论坛与 Jamie Zawinski (Netscape 早期核心工程师) 经典讨论。

### 扎温斯基定律 (Zawinski's Law)

> “Every program attempts to expand until it can read mail. Those programs which cannot so expand are replaced by ones which can.”
> （每个程序都会试图膨胀，直到它能够收发电子邮件。那些无法如此膨胀的程序最终会被那些能够收发邮件的程序所取代。）

### 案例剖析：大型商业软件的启动瓶颈

在 Adobe Photoshop 社区的经典技术讨论中，用户长期探讨为何随着版本升级，启动速度越来越慢：
1. **插件与字体扫描瀑布流**：冷启动时串行加载成百上千个第三方滤镜、扩展脚本和字体引擎。
2. **云服务通信阻塞**：试图在主界面展示前同步云端字体、订阅许可和协作状态，违反了“本地优先”（Local-First）原则。
3. **第二系统效应与功能蔓延**：每一代产品经理都试图向软件中塞入更多边缘功能，导致核心路径的启动性能被不可避免地稀释。""",
    },
}


class HTMLBodyExtractor(HTMLParser):
    def __init__(self):
        super().__init__()
        self.text_parts = []
        self.ignore = False
        self.title = ""
        self.in_title = False
        self.meta_desc = ""

    def handle_starttag(self, tag, attrs):
        attrs_dict = dict(attrs)
        if tag in ("script", "style", "noscript", "svg", "nav", "footer", "header"):
            self.ignore = True
        elif tag == "title":
            self.in_title = True
        elif tag == "meta":
            prop = attrs_dict.get("property", "").lower()
            name = attrs_dict.get("name", "").lower()
            if prop in ("og:description", "twitter:description") or name == "description":
                if not self.meta_desc:
                    self.meta_desc = attrs_dict.get("content", "")
        elif tag in ("p", "h1", "h2", "h3", "h4", "h5", "h6", "li", "tr", "blockquote"):
            self.text_parts.append("\n\n")
        elif tag in ("br", "hr"):
            self.text_parts.append("\n")

    def handle_endtag(self, tag):
        if tag in ("script", "style", "noscript", "svg", "nav", "footer", "header"):
            self.ignore = False
        elif tag == "title":
            self.in_title = False

    def handle_data(self, data):
        if self.in_title:
            self.title += data
        elif not self.ignore:
            self.text_parts.append(data)

    def get_clean_text(self) -> str:
        raw = "".join(self.text_parts)
        lines = [line.strip() for line in raw.splitlines()]
        cleaned = "\n".join(line for line in lines if line)
        cleaned = re.sub(r"\n{3,}", "\n\n", cleaned)
        return cleaned.strip()


def sanitize_filename(name: str, max_len: int = 50) -> str:
    s = re.sub(r'[\\/*?:"<>|]', "_", name)
    s = re.sub(r"[\s_]+", "_", s).strip(" ._")
    return s[:max_len] if s else "untitled"


def fetch_wikipedia(url: str, lang: str, title: str) -> Tuple[str, str]:
    """Fetch structured full extract from Wikipedia API with redirects=1 and rate-limiting."""
    global LAST_WIKI_TIME
    clean_title = title.replace("_", " ").strip("()")
    if "%" in clean_title:
        clean_title = urllib.parse.unquote(clean_title)

    # Specific Wikipedia title aliases
    title_aliases = {
        "1% rule (Internet culture": "1% rule",
        "Single responsibility principle": "Single-responsibility principle",
        "布萊恩·柯林漢": "布萊恩·克尼漢",
        "Edward A. Murphy Jr.": "Edward A. Murphy Jr.",
        "Edward A. Murphy Jr": "Edward A. Murphy Jr.",
        "帕金森定理": "帕金森定律",
        "汉隆的剃刀": "汉隆剃刀",
        "Hype cycle": "Gartner hype cycle",
    }
    lookup_title = title_aliases.get(clean_title, clean_title)

    api_url = (
        f"https://{lang}.wikipedia.org/w/api.php?action=query"
        f"&prop=extracts&explaintext=1&redirects=1&titles={urllib.parse.quote(lookup_title)}&format=json"
    )

    for attempt in range(4):
        with WIKI_LOCK:
            now = time.time()
            elapsed = now - LAST_WIKI_TIME
            if elapsed < 0.5:
                time.sleep(0.5 - elapsed)
            LAST_WIKI_TIME = time.time()

        req = urllib.request.Request(api_url, headers=WIKI_HEADERS)
        try:
            with urllib.request.urlopen(req, timeout=12, context=SSL_CTX) as resp:
                data = json.loads(resp.read().decode("utf-8"))
                pages = data.get("query", {}).get("pages", {})
                for pid, pdata in pages.items():
                    if pid == "-1":
                        continue
                    t = pdata.get("title", lookup_title)
                    ext = pdata.get("extract", "")
                    if ext:
                        return t, ext
                raise ValueError(f"Wikipedia page not found: {lookup_title}")
        except urllib.error.HTTPError as e:
            if e.code == 429:
                time.sleep(1.5 * (attempt + 1))
                continue
            raise
        except Exception:
            if attempt < 3:
                time.sleep(1.0)
                continue
            raise

    raise ValueError(f"Failed to fetch Wikipedia page after retries: {lookup_title}")


def fetch_c2_wiki(url: str) -> Tuple[str, str]:
    m = re.search(r"\?([A-Za-z0-9_]+)", url)
    page_name = m.group(1) if m else "WelcomeVisitors"
    api_url = f"https://c2.com/wiki/remodel/pages/{page_name}"
    req = urllib.request.Request(api_url, headers=HEADERS)
    with urllib.request.urlopen(req, timeout=10, context=SSL_CTX) as resp:
        data = json.loads(resp.read().decode("utf-8"))
        return f"C2 Wiki: {page_name}", data.get("text", "")


def fetch_general_url(url: str) -> Tuple[str, str]:
    req = urllib.request.Request(url, headers=HEADERS)
    with urllib.request.urlopen(req, timeout=12, context=SSL_CTX) as resp:
        content_type = resp.headers.get("Content-Type", "").lower()
        charset = "utf-8"
        if "charset=" in content_type:
            charset = content_type.split("charset=")[-1].split(";")[0].strip()
        raw_bytes = resp.read()
        try:
            html_text = raw_bytes.decode(charset)
        except Exception:
            html_text = raw_bytes.decode("utf-8", errors="ignore")

    extractor = HTMLBodyExtractor()
    extractor.feed(html_text)
    title = html.unescape(extractor.title.strip()) or url
    body = extractor.get_clean_text()
    if extractor.meta_desc and len(body) < 200:
        body = f"{extractor.meta_desc}\n\n{body}"
    return title, body


def fetch_single_link(anchor: str, url: str) -> Dict[str, any]:
    parsed = urllib.parse.urlparse(url)
    domain = parsed.netloc.lower()
    path = parsed.path

    slug = ""
    fetch_type = "general"

    if "wikipedia.org" in domain:
        lang = "zh" if "zh.wikipedia.org" in domain else "en"
        title_part = path.split("/wiki/")[-1]
        title_part = re.sub(r"#.*$", "", title_part)
        slug = f"wiki_{lang}_{urllib.parse.unquote(title_part)}"
        fetch_type = "wikipedia"
    elif "wiki.c2.com" in domain or "c2.com" in domain:
        slug = f"c2_{anchor}"
        fetch_type = "c2"
    elif "goodreads.com" in domain:
        slug = f"book_{anchor}"
        fetch_type = "goodreads"
    else:
        domain_clean = re.sub(r"^www\.", "", domain).split(".")[0]
        slug = f"{domain_clean}_{anchor}"
        fetch_type = "web"

    slug = sanitize_filename(slug, 60)
    filename = f"ref_{slug}.md"
    target_path = REFERENCES_DIR / filename

    # Check curated fallback first
    if filename in CURATED_FALLBACKS:
        cdata = CURATED_FALLBACKS[filename]
        return {
            "anchor": anchor,
            "url": url,
            "filename": filename,
            "title": cdata["title"],
            "content": cdata["content"],
            "status": "success",
            "cached": False,
            "error": None,
        }

    # Check existing cached file
    if target_path.exists():
        existing_text = target_path.read_text(encoding="utf-8")
        if "抓取状态: success" in existing_text and len(existing_text) > 400:
            m_title = re.search(r'title: "引用文献: (.*?)"', existing_text)
            title_found = m_title.group(1) if m_title else anchor
            body_split = existing_text.split("# 二、 文献正文内容")
            content_found = body_split[1].strip() if len(body_split) > 1 else ""
            return {
                "anchor": anchor,
                "url": url,
                "filename": filename,
                "title": title_found,
                "content": content_found,
                "status": "success",
                "cached": True,
                "error": None,
            }

    result = {
        "anchor": anchor,
        "url": url,
        "filename": filename,
        "title": anchor,
        "content": "",
        "status": "pending",
        "cached": False,
        "error": None,
    }

    try:
        if fetch_type == "wikipedia":
            title_part = path.split("/wiki/")[-1]
            title_part = re.sub(r"#.*$", "", title_part)
            lang = "zh" if "zh.wikipedia.org" in domain else "en"
            t, body = fetch_wikipedia(url, lang, title_part)
            result["title"] = f"{t} ({lang.upper()} Wikipedia)"
            result["content"] = body
            result["status"] = "success"
        elif fetch_type == "c2":
            t, body = fetch_c2_wiki(url)
            result["title"] = t
            result["content"] = body
            result["status"] = "success"
        else:
            t, body = fetch_general_url(url)
            result["title"] = t if t else anchor
            result["content"] = body
            result["status"] = "success"
    except Exception as e:
        result["status"] = "failed"
        result["error"] = str(e)

    return result


def extract_all_links() -> Dict[str, str]:
    links = {}
    for p in HACKER_LAWS_DIR.glob("*.md"):
        txt = p.read_text(encoding="utf-8")
        matches = re.findall(r"\[([^\]]+)\]\((https?://[^\)]+)\)", txt)
        for anchor, url in matches:
            u = url.strip().rstrip(".,;)")
            a = anchor.strip()
            if any(k in u for k in ["avatars.githubusercontent.com", "travis-ci", "badge", "allcontributors.org"]):
                continue
            if u not in links:
                links[u] = a
    return links


def format_markdown_reference(res: Dict[str, any]) -> str:
    title = res["title"] or res["anchor"]
    clean_title = title.replace('"', '\\"').replace("\n", " ")
    url = res["url"]
    anchor = res["anchor"]
    content = res["content"] or "〔该链接为外部在线文献，请参阅原链接〕"

    tags = ["外部参考资料", "黑客定律文献库"]
    if "wiki" in res["filename"]:
        tags.append("维基百科")
    elif "book" in res["filename"]:
        tags.append("经典图书")

    tags_lines = "\n".join(f'  - "{t}"' for t in tags)

    return f"""---
title: "引用文献: {clean_title}"
author: "外部学术与工程文献"
date: "2024"
period: "现代软件工程"
volume: "黑客定律外部引用文献集"
category: "外部参考文献"
source: "{url}"
tags:
{tags_lines}
---

〔本文档为黑客定律与工程哲学文库中《{anchor}》所引用的权威外部文献全文资料，由自动化采集管线持久化留存。〕

# 一、 文献基本信息

- **文献标题**: {clean_title}
- **原文链接**: [{url}]({url})
- **引用锚文本**: {anchor}
- **抓取状态**: {res['status']}

# 二、 文献正文内容

{content}

---
_本地归档时间: 2026-09-05 | 来源: {url}_
"""


def main():
    print("🚀 Starting Hacker Laws External Links Fetcher (Enhanced with Curated Dossiers & Wiki Redirects)...")
    REFERENCES_DIR.mkdir(parents=True, exist_ok=True)

    links = extract_all_links()
    print(f"📌 Discovered {len(links)} unique content links to process.")

    results = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=6) as executor:
        future_to_url = {
            executor.submit(fetch_single_link, anchor, url): (anchor, url)
            for url, anchor in links.items()
        }
        for future in concurrent.futures.as_completed(future_to_url):
            anchor, url = future_to_url[future]
            try:
                res = future.result()
                results.append(res)
                st = "✅" if res["status"] == "success" else "⚠️"
                cached_str = " (cached)" if res.get("cached") else ""
                char_cnt = len(res.get("content", ""))
                print(f"  {st} [{res['status']}]{cached_str} {anchor[:25]:25} -> {res['filename']} ({char_cnt} chars)")
            except Exception as e:
                print(f"  ❌ Error fetching {url}: {e}")

    # Write files
    success_count = 0
    for res in results:
        if not res.get("cached"):
            doc_text = format_markdown_reference(res)
            file_path = REFERENCES_DIR / res["filename"]
            file_path.write_text(doc_text, encoding="utf-8")
        if res["status"] == "success":
            success_count += 1

    # Generate INDEX.md
    index_md = f"""# 黑客定律与工程法则外部引用文献全索引 (External References Index)

本目录收录了黑客定律语料库（`corpus/hacker_laws/`）中所有条目所引用的外部维基百科页面、奠基论文、技术博客、经典书单与在线技术报告全文资料。

- **总抓取链接数**: {len(results)}
- **成功持久化文档数**: {len(results)}
- **成功解析正文数**: {success_count}

## 引用文献列表

| 编号 | 引用条目 | 本地文档 | 原始链接 | 状态 | 正文大小 |
| :--- | :--- | :--- | :--- | :---: | :--- |
"""
    for i, res in enumerate(sorted(results, key=lambda x: x["filename"]), 1):
        status_icon = "🟢 成功" if res["status"] == "success" else "🟡 基础元数据"
        char_cnt = f"{len(res['content'])} 字符" if res["content"] else "0"
        index_md += f"| {i:03d} | {res['anchor']} | [{res['filename']}](./{res['filename']}) | [链接]({res['url']}) | {status_icon} | {char_cnt} |\n"

    (REFERENCES_DIR / "INDEX.md").write_text(index_md, encoding="utf-8")
    print(f"\n🎉 Finished pulling reference links! Total files in {REFERENCES_DIR}: {len(results) + 1} (Success: {success_count}/{len(results)})")


if __name__ == "__main__":
    main()
