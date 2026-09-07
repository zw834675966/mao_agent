#!/usr/bin/env python3
"""
scripts/add_category.py

Scan markdown files in corpus and insert 'category: history' or 'category: engineering'
into YAML frontmatter if missing. Pure Python stdlib only.
Idempotent: Running multiple times produces the exact same result.
Preserves existing category fields and does NOT touch any files under 'raw/'.
"""

import argparse
import os
import re
import sys

FRONTMATTER_PATTERN = re.compile(r"^---\r?\n(.*?)\r?\n---\r?\n?(.*)$", re.DOTALL)
CATEGORY_KEY_PATTERN = re.compile(r"^\s*category\s*:", re.MULTILINE)

ENGINEERING_DOMAINS = {
    "hacker_laws",
    "hello_algo",
    "papers_we_love",
    "awesome_scalability",
    "engineering",
}


def is_raw_path(path: str) -> bool:
    """Check if any directory component of the path is 'raw'."""
    normalized = os.path.normpath(path)
    parts = normalized.split(os.sep)
    return "raw" in parts


def determine_category(file_path: str) -> str:
    """Determine whether a file belongs to history or engineering domain."""
    normalized = os.path.normpath(file_path)
    parts = normalized.split(os.sep)
    for part in parts:
        if part.lower() == "history":
            return "history"
        if part.lower() in ENGINEERING_DOMAINS:
            return "engineering"
    # Fallback: if filename matches known history filenames
    basename = os.path.basename(file_path)
    if basename.endswith(".md"):
        return "history"
    return "engineering"


def process_file(file_path: str, check_only: bool = False):
    """
    Process a single markdown file:
    Returns (status, reason)
    status: True if modified (or would be modified in check_only), False otherwise.
    """
    if is_raw_path(file_path):
        return False, "skipped_raw"

    if not (file_path.endswith(".md") or file_path.endswith(".markdown")):
        return False, "skipped_non_markdown"

    with open(file_path, "r", encoding="utf-8", errors="ignore") as f:
        content = f.read()

    # Detect line endings
    newline = "\r\n" if "\r\n" in content else "\n"

    m = FRONTMATTER_PATTERN.match(content)
    if m:
        fm_text = m.group(1)
        body = m.group(2)
        if CATEGORY_KEY_PATTERN.search(fm_text):
            return False, "already_has_category"

        cat = determine_category(file_path)
        fm_stripped = fm_text.rstrip("\r\n")
        new_fm = f"{fm_stripped}{newline}category: {cat}{newline}"
        new_content = f"---{newline}{new_fm}---{newline}{body}"
    else:
        # Document without frontmatter
        cat = determine_category(file_path)
        new_content = f"---{newline}category: {cat}{newline}---{newline}{newline}{content}"

    if not check_only:
        with open(file_path, "w", encoding="utf-8", newline="") as f:
            f.write(new_content)

    return True, "added_category"


def scan_corpus(corpus_dir: str, check_only: bool = False):
    stats = {
        "total_scanned": 0,
        "skipped_raw": 0,
        "already_has_category": 0,
        "added_category": 0,
        "modified_files": [],
    }

    for root, dirs, files in os.walk(corpus_dir):
        # Skip walking into raw directories to avoid unnecessary I/O
        if "raw" in dirs:
            dirs.remove("raw")
            stats["skipped_raw"] += 1

        for f in files:
            if not (f.endswith(".md") or f.endswith(".markdown")):
                continue
            stats["total_scanned"] += 1
            full_path = os.path.join(root, f)
            modified, reason = process_file(full_path, check_only=check_only)
            if reason == "already_has_category":
                stats["already_has_category"] += 1
            elif reason == "added_category":
                stats["added_category"] += 1
                stats["modified_files"].append(full_path)

    return stats


def main():
    parser = argparse.ArgumentParser(
        description="Idempotently insert missing category frontmatter into corpus markdown files."
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Dry-run mode: report files needing modification without changing them.",
    )
    parser.add_argument(
        "--corpus-dir",
        default="corpus",
        help="Target corpus directory (default: 'corpus').",
    )
    args = parser.parse_args()

    if not os.path.isdir(args.corpus_dir):
        print(f"Error: Directory '{args.corpus_dir}' does not exist.", file=sys.stderr)
        sys.exit(1)

    mode_str = "DRY-RUN (--check)" if args.check else "EXECUTE"
    print(f"=== Running add_category.py in {mode_str} mode on '{args.corpus_dir}' ===")

    stats = scan_corpus(args.corpus_dir, check_only=args.check)

    print(f"Total clean markdown files scanned: {stats['total_scanned']}")
    print(f"Files already containing category:   {stats['already_has_category']}")
    print(f"Files requiring category addition:  {stats['added_category']}")

    if stats["modified_files"]:
        print(f"\nFiles needing update ({len(stats['modified_files'])}):")
        for p in stats["modified_files"]:
            print(f"  - {p}")
    else:
        print("\nAll scanned markdown files already have a category field.")

    print("\nComplete.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
