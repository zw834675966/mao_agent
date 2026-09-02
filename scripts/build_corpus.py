#!/usr/bin/env python3
"""
Corpus Building and Preprocessing Pipeline for mao_agent.
Hardened against Path Traversal, YAML Injection, and OCR malformations.
"""

from __future__ import annotations

import argparse
import hashlib
import logging
import os
import re
import sys
import tempfile
from pathlib import Path
from typing import Any, Dict, List, Optional
import urllib.request
import urllib.error

# Setup structured logging
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(name)s - %(message)s",
    handlers=[logging.StreamHandler(sys.stdout)],
)
logger = logging.getLogger("build_corpus")

BASE_DIR = Path(__file__).resolve().parent.parent
CORPUS_DIR = BASE_DIR / "corpus"

# Windows reserved device names
WINDOWS_RESERVED_NAMES = {
    "CON", "PRN", "AUX", "NUL",
    *(f"COM{i}" for i in range(1, 10)),
    *(f"LPT{i}" for i in range(1, 10)),
}

CJK_CHAR_CLASS = r'[\u4e00-\u9fff\u3400-\u4dbf\uf900-\ufaff\u3000-\u303f\uff00-\uffef]'
HORIZONTAL_WS = r'[^\S\r\n]+'


def clean_cjk_spaces(text: Optional[str]) -> str:
    """Safely remove OCR spurious horizontal whitespace between CJK characters, preserving newlines."""
    if not text:
        return ""
    text = text.replace('\u3000', ' ').replace('\u00a0', ' ')
    pattern = re.compile(f'(?<={CJK_CHAR_CLASS}){HORIZONTAL_WS}(?={CJK_CHAR_CLASS})')
    text = pattern.sub('', text)
    digit_cjk = re.compile(f'(?<=[0-9]){HORIZONTAL_WS}(?={CJK_CHAR_CLASS})')
    text = digit_cjk.sub('', text)
    cjk_digit = re.compile(f'(?<={CJK_CHAR_CLASS}){HORIZONTAL_WS}(?=[0-9])')
    text = cjk_digit.sub('', text)
    return text.strip()


def parse_chinese_number(cn_str: str) -> Optional[int]:
    """Parse Chinese numeral (1-99) into an integer."""
    digits = {'〇': 0, '零': 0, '一': 1, '二': 2, '三': 3, '四': 4,
              '五': 5, '六': 6, '七': 7, '八': 8, '九': 9, '十': 10,
              '廿': 20, '卅': 30}
    if not cn_str:
        return None
    cn_str = cn_str.strip()
    if cn_str in digits:
        return digits[cn_str]
    if cn_str.startswith('十') and len(cn_str) == 2:
        return 10 + digits.get(cn_str[1], 0)
    if cn_str.endswith('十') and len(cn_str) == 2:
        return digits.get(cn_str[0], 0) * 10
    if len(cn_str) == 3 and cn_str[1] == '十':
        return digits.get(cn_str[0], 0) * 10 + digits.get(cn_str[2], 0)
    if len(cn_str) == 2 and cn_str[0] in ('廿', '卅'):
        return digits.get(cn_str[0], 0) + digits.get(cn_str[1], 0)
    return None


def parse_date(text: Optional[str]) -> str:
    """Extract standard ISO date from Chinese and numeric dates, including ROC era."""
    if not text or not isinstance(text, str):
        return "未知"
    text = clean_cjk_spaces(text)

    # 1. ROC era date (民国X年)
    m_roc = re.search(r'民国\s*([一二三四五六七八九十\d]+)\s*年(?:\s*([一二三四五六七八九十\d]+)\s*月)?(?:\s*([一二三四五六七八九十廿卅\d]+)\s*日)?', text)
    if m_roc:
        roc_year = parse_chinese_number(m_roc.group(1)) or (int(m_roc.group(1)) if m_roc.group(1).isdigit() else None)
        if roc_year:
            ad_year = 1911 + roc_year
            m_val = parse_chinese_number(m_roc.group(2)) or (int(m_roc.group(2)) if m_roc.group(2) and m_roc.group(2).isdigit() else None)
            d_val = parse_chinese_number(m_roc.group(3)) or (int(m_roc.group(3)) if m_roc.group(3) and m_roc.group(3).isdigit() else None)
            if m_val and d_val:
                return f"{ad_year:04d}-{m_val:02d}-{d_val:02d}"
            elif m_val:
                return f"{ad_year:04d}-{m_val:02d}"
            return f"{ad_year:04d}"

    # 2. ISO match
    m_iso = re.search(r'\b(18\d{2}|19\d{2}|20\d{2})[-/.](0?[1-9]|1[0-2])(?:[-/.](0?[1-9]|[12]\d|3[01]))?\b', text)
    if m_iso:
        y = m_iso.group(1)
        m = int(m_iso.group(2))
        d = int(m_iso.group(3)) if m_iso.group(3) else None
        return f"{y}-{m:02d}-{d:02d}" if d else f"{y}-{m:02d}"

    # 3. Arabic digits with Chinese units
    m_ar = re.search(r'\b(18\d{2}|19\d{2}|20\d{2})\s*年\s*([1-9]|1[0-2])\s*月(?:\s*([1-9]|[12]\d|3[01])\s*日)?', text)
    if m_ar:
        y = m_ar.group(1)
        m = int(m_ar.group(2))
        d = int(m_ar.group(3)) if m_ar.group(3) else None
        return f"{y}-{m:02d}-{d:02d}" if d else f"{y}-{m:02d}"

    # 4. Chinese numerals
    digits_map = {'〇': '0', '零': '0', '一': '1', '二': '2', '三': '3', '四': '4',
                  '五': '5', '六': '6', '七': '7', '八': '8', '九': '9'}
    m_cn = re.search(
        r'([一二三四五六七八九〇零]{4})\s*年\s*([一二三四五六七八九十]{1,3})\s*月(?:\s*([一二三四五六七八九十廿卅初]{1,3})\s*日)?',
        text
    )
    if m_cn:
        year_digits = [digits_map.get(c) for c in m_cn.group(1)]
        if all(d is not None for d in year_digits):
            year_str = "".join(year_digits)
            month_val = parse_chinese_number(m_cn.group(2))
            if month_val and 1 <= month_val <= 12:
                day_raw = m_cn.group(3)
                if day_raw:
                    day_clean = day_raw.lstrip('初')
                    day_val = parse_chinese_number(day_clean)
                    if day_val and 1 <= day_val <= 31:
                        return f"{year_str}-{month_val:02d}-{day_val:02d}"
                return f"{year_str}-{month_val:02d}"

    # 5. Year only fallback
    m_y_ar = re.search(r'\b(18\d{2}|19\d{2}|20\d{2})\s*年', text)
    if m_y_ar:
        return m_y_ar.group(1)

    m_y_cn = re.search(r'([一二三四五六七八九〇零]{4})\s*年', text)
    if m_y_cn:
        year_digits = [digits_map.get(c) for c in m_y_cn.group(1)]
        if all(d is not None for d in year_digits):
            return "".join(year_digits)

    return "未知"


def sanitize_filename(name: Optional[str], default: str = "untitled") -> str:
    """Sanitize string into safe cross-platform filename, preventing path traversal."""
    if not name or not isinstance(name, str):
        return default
    name = clean_cjk_spaces(name)
    name = re.sub(r'[\/:*?"<>|\\#\r\n\t]', '_', name)
    name = re.sub(r'〔\d+〕|\[\d+\]', '', name)
    name = re.sub(r'[_.\s]+', '_', name).strip(' ._')
    if not name or name == ".." or name == ".":
        name = default
    base_stem = name.split('.')[0].upper()
    if base_stem in WINDOWS_RESERVED_NAMES:
        name = f"doc_{name}"
    return name[:64].rstrip(' ._')


def escape_yaml_string(val: Any) -> str:
    """Safely escape strings for YAML scalars."""
    if val is None:
        return '""'
    s = str(val).replace('\\', '\\\\').replace('"', '\\"').replace('\n', ' ').replace('\r', '')
    return f'"{s}"'


def make_frontmatter(
    title: str,
    author: str,
    date: str,
    category: str,
    volume: str,
    period: str = "",
    source: str = "",
    tags: Optional[List[str]] = None,
) -> str:
    """Generate strictly formatted and injection-safe YAML frontmatter."""
    tags = tags or []
    tags_lines = "\n".join(f"  - {escape_yaml_string(t)}" for t in tags)
    return f"""---
title: {escape_yaml_string(title)}
author: {escape_yaml_string(author)}
date: {escape_yaml_string(date)}
category: {escape_yaml_string(category)}
volume: {escape_yaml_string(volume)}
period: {escape_yaml_string(period)}
source: {escape_yaml_string(source)}
tags:
{tags_lines}
---

"""


def main() -> None:
    parser = argparse.ArgumentParser(description="Corpus builder for mao_agent")
    parser.add_argument("--output-dir", type=Path, default=CORPUS_DIR, help="Target corpus directory")
    args = parser.parse_args()
    logger.info(f"Initialized corpus builder. Output dir: {args.output_dir.resolve()}")


if __name__ == "__main__":
    main()
