#!/usr/bin/env python3
"""
Knowledge-graph extractor for mao_agent (Cycle 11, Task 4).

Emits the JSON contract in tasks/plan.md (entities[] + relationships[]), consumed
by `GraphStore::from_json` after `ingest-graph` bincode conversion.

Modes:
  --mock  Deterministic rule table grounded in corpus/ titles and section
          headings. No network, no chunk_id hashes: source_refs join on stable
          doc_title (+ optional section_path) because ingest-hash chunk_ids go
          stale on re-chunk.
  live    OpenAI-compatible chat/completions via urllib.request
          (--api-key --base-url --model). Response JSON is schema-validated;
          invalid output exits non-zero rather than writing a bad graph.

Stdlib only (argparse, json, urllib, pathlib) — matches scripts/build_corpus.py.
No pip, no LightRAG, no networkx, no openai SDK.
"""

from __future__ import annotations

import argparse
import json
import logging
import sys
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any, Dict, List, Optional

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(name)s - %(message)s",
    handlers=[logging.StreamHandler(sys.stdout)],
)
logger = logging.getLogger("build_knowledge_graph")

BASE_DIR = Path(__file__).resolve().parent.parent
DEFAULT_CORPUS_DIR = BASE_DIR / "corpus"

# Stable join keys: these are frontmatter titles / markdown headings, never
# ingest-hash chunk_ids.
MAO_TITLE = "矛盾论"
MAO_SECTION_PRINCIPAL = "主要矛盾和主要的矛盾方面"
AMD_TITLE = "阿姆达尔定律 (Amdahl's Law)"
MOORE_TITLE = "摩尔定律 (Moore's Law)"
BROOKS_TITLE = "布鲁克斯法则 (Brooks' Law)"
CONWAY_TITLE = "康威定律 (Conway's Law)"
LAW_SECTION_CORE = "一"

REL_TYPES = {
    "aligned_with",
    "primarily_enables",
    "constrains",
    "contradicts",
    "supports",
    "part_of",
}

MOCK_RULES = """\
Mock rule table (deterministic, corpus-grounded):
  矛盾论 #主要矛盾和主要的矛盾方面 states the principal contradiction governs and
  shapes the other contradictions. The rule table aligns that dialectical method
  with engineering laws whose docs state the same structure:
    - 阿姆达尔定律 (Amdahl's Law): the serial fraction governs achievable speedup
      -> aligned_with (weight 1.0)
    - 摩尔定律 (Moore's Law): the 矛盾论 doc on hacker-laws shows Moore slowing
      pushes parallelism; amdahls_law.md cites it -> aligned_with (weight 1.0)
    - 布鲁克斯法则 (Brooks' Law): adding people to a late project grows
      communication overhead, the new governing constraint -> primarily_enables (0.9)
    - 康威定律 (Conway's Law): the organization's communication structure becomes
      the governing constraint on system design -> primarily_enables (0.9)
    - 主要矛盾 -> 次要矛盾: same 矛盾论 section, the governing relationship itself
      -> primarily_enables (0.9)
"""


def _source_ref(doc_title: str, section_path: Optional[List[str]] = None) -> Dict[str, Any]:
    ref: Dict[str, Any] = {"doc_title": doc_title}
    if section_path:
        ref["section_path"] = section_path
    return ref


def _entity(entity_id: str, name: str, domain: str, aliases: List[str], ref: Dict[str, Any]) -> Dict[str, Any]:
    return {
        "id": entity_id,
        "name": name,
        "aliases": aliases,
        "domain": domain,
        "source_refs": [ref],
    }


def build_mock_document() -> Dict[str, Any]:
    """Deterministic seed graph over stable corpus titles (see MOCK_RULES)."""
    mao_ref = _source_ref(MAO_TITLE, [MAO_SECTION_PRINCIPAL])
    law_ref = lambda title: _source_ref(title, [LAW_SECTION_CORE])  # noqa: E731

    entities = [
        _entity(
            "ent:principal_contradiction",
            "主要矛盾",
            "mao",
            ["principal contradiction", "矛盾的主要方面"],
            mao_ref,
        ),
        _entity(
            "ent:secondary_contradictions",
            "次要矛盾",
            "mao",
            ["secondary contradictions"],
            mao_ref,
        ),
        _entity(
            "ent:amdahls_serial_fraction",
            AMD_TITLE,
            "engineering",
            ["Amdahl's Law", "阿姆达尔定律"],
            law_ref(AMD_TITLE),
        ),
        _entity(
            "ent:moores_law",
            MOORE_TITLE,
            "engineering",
            ["Moore's Law", "摩尔定律"],
            law_ref(MOORE_TITLE),
        ),
        _entity(
            "ent:brooks_law",
            BROOKS_TITLE,
            "engineering",
            ["Brooks' Law", "布鲁克斯法则"],
            law_ref(BROOKS_TITLE),
        ),
        _entity(
            "ent:conways_law",
            CONWAY_TITLE,
            "engineering",
            ["Conway's Law", "康威定律"],
            law_ref(CONWAY_TITLE),
        ),
    ]

    relationships = [
        {
            "id": "rel:contradiction-amdahl",
            "source": "ent:principal_contradiction",
            "target": "ent:amdahls_serial_fraction",
            "rel_type": "aligned_with",
            "weight": 1.0,
            "source_refs": [_source_ref(MAO_TITLE), _source_ref(AMD_TITLE)],
        },
        {
            "id": "rel:contradiction-moores",
            "source": "ent:principal_contradiction",
            "target": "ent:moores_law",
            "rel_type": "aligned_with",
            "weight": 1.0,
            "source_refs": [_source_ref(MAO_TITLE), _source_ref(MOORE_TITLE)],
        },
        {
            "id": "rel:contradiction-brooks",
            "source": "ent:principal_contradiction",
            "target": "ent:brooks_law",
            "rel_type": "primarily_enables",
            "weight": 0.9,
            "source_refs": [_source_ref(MAO_TITLE), _source_ref(BROOKS_TITLE)],
        },
        {
            "id": "rel:contradiction-conways",
            "source": "ent:principal_contradiction",
            "target": "ent:conways_law",
            "rel_type": "primarily_enables",
            "weight": 0.9,
            "source_refs": [_source_ref(MAO_TITLE), _source_ref(CONWAY_TITLE)],
        },
        {
            "id": "rel:principal-secondary",
            "source": "ent:principal_contradiction",
            "target": "ent:secondary_contradictions",
            "rel_type": "primarily_enables",
            "weight": 0.9,
            "source_refs": [_source_ref(MAO_TITLE)],
        },
    ]
    return {"entities": entities, "relationships": relationships}


def validate_document(doc: Any) -> None:
    """Schema-validate the tasks/plan.md JSON contract. Raises ValueError."""
    if not isinstance(doc, dict):
        raise ValueError("document root must be a JSON object")
    for key in ("entities", "relationships"):
        if key not in doc or not isinstance(doc[key], list):
            raise ValueError(f"document must contain a '{key}' array")

    entity_ids = set()
    for i, ent in enumerate(doc["entities"]):
        prefix = f"entities[{i}]"
        for field in ("id", "name", "aliases", "domain", "source_refs"):
            if field not in ent:
                raise ValueError(f"{prefix} missing field '{field}'")
        if not ent["id"].startswith("ent:"):
            raise ValueError(f"{prefix}.id must start with 'ent:'")
        if entity_ids and ent["id"] in entity_ids:
            raise ValueError(f"{prefix}.id duplicate: {ent['id']}")
        if not isinstance(ent["aliases"], list) or not isinstance(ent["source_refs"], list):
            raise ValueError(f"{prefix}.aliases/.source_refs must be arrays")
        entity_ids.add(ent["id"])

    for i, rel in enumerate(doc["relationships"]):
        prefix = f"relationships[{i}]"
        for field in ("id", "source", "target", "rel_type", "weight", "source_refs"):
            if field not in rel:
                raise ValueError(f"{prefix} missing field '{field}'")
        if not rel["id"].startswith("rel:"):
            raise ValueError(f"{prefix}.id must start with 'rel:'")
        if rel["rel_type"] not in REL_TYPES:
            raise ValueError(f"{prefix}.rel_type unknown: {rel['rel_type']}")
        if not isinstance(rel["weight"], (int, float)) or isinstance(rel["weight"], bool):
            raise ValueError(f"{prefix}.weight must be a number")
        for endpoint in (rel["source"], rel["target"]):
            if endpoint not in entity_ids:
                raise ValueError(f"{prefix} references unknown entity '{endpoint}'")

    for container, prefix in ((doc["entities"], "entities"), (doc["relationships"], "relationships")):
        for i, item in enumerate(container):
            for j, ref in enumerate(item["source_refs"]):
                rprefix = f"{prefix}[{i}].source_refs[{j}]"
                if "doc_title" not in ref or not isinstance(ref["doc_title"], str):
                    raise ValueError(f"{rprefix} requires a string 'doc_title'")
                if "chunk_id" in ref:
                    raise ValueError(f"{rprefix} must not carry chunk_id (stale hash join)")


SYSTEM_PROMPT = (
    "You are a knowledge-graph extractor. From the given corpus document "
    "titles and excerpts, emit ONLY a JSON object with keys 'entities' and "
    "'relationships'. Entity: {id: 'ent:...', name, aliases: [], domain: "
    "'mao'|'engineering', source_refs: [{doc_title, section_path?}]}. "
    "Relationship: {id: 'rel:...', source, target, rel_type: one of "
    "aligned_with|primarily_enables|constrains|contradicts|supports|part_of, "
    "weight: 0.0-1.0, source_refs: [{doc_title}]}. Join on stable doc titles "
    "from the corpus frontmatter, never on chunk hashes. No markdown fences."
)


def extract_live(api_key: str, base_url: str, model: str) -> Dict[str, Any]:
    """One OpenAI-compatible chat/completions call; validates before returning."""
    corpus_files = sorted(p.name for p in DEFAULT_CORPUS_DIR.rglob("*.md"))[:200]
    user_prompt = (
        "Corpus document titles (frontmatter `title`):\n"
        + "\n".join(corpus_files)
        + "\n\nExtract the cross-domain knowledge graph. Must include a "
        "directed aligned_with edge from the entity named 主要矛盾 to the "
        "entity named 阿姆达尔定律 (Amdahl's Law)."
    )
    payload = json.dumps(
        {
            "model": model,
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": user_prompt},
            ],
            "temperature": 0,
        }
    ).encode("utf-8")
    req = urllib.request.Request(
        f"{base_url}/chat/completions",
        data=payload,
        headers={
            "Content-Type": "application/json",
            "Authorization": f"Bearer {api_key}",
        },
        method="POST",
    )
    logger.info("Live extraction: model=%s base_url=%s", model, base_url)
    try:
        with urllib.request.urlopen(req, timeout=120) as resp:
            body = json.loads(resp.read().decode("utf-8"))
    except (urllib.error.URLError, TimeoutError, json.JSONDecodeError) as exc:
        raise ValueError(f"chat/completions call failed: {exc}") from exc

    content = body["choices"][0]["message"]["content"]
    if content.startswith("```"):
        content = content.strip("`")
        if content.startswith("json"):
            content = content[4:]
    doc = json.loads(content)
    validate_document(doc)
    return doc


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Knowledge-graph extractor for mao_agent (stdlib only).",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument(
        "--mock",
        action="store_true",
        help="Deterministic rule table; no network.",
    )
    parser.add_argument("--output", type=Path, help="Write JSON here (else stdout).")
    parser.add_argument("--corpus-dir", type=Path, default=DEFAULT_CORPUS_DIR, help="Corpus root.")
    parser.add_argument("--api-key", help="Live mode: API key (prefer COHERE/LLM_API_KEY env in callers).")
    parser.add_argument("--base-url", help="Live mode: OpenAI-compatible base URL.")
    parser.add_argument("--model", help="Live mode: chat model name.")
    args = parser.parse_args()

    live_flags = [bool(args.api_key), bool(args.base_url), bool(args.model)]
    if any(live_flags) and not all(live_flags):
        parser.error("--api-key, --base-url and --model must be provided together")
    if args.mock and any(live_flags):
        parser.error("--mock and live flags are mutually exclusive")
    if not args.mock:
        if not all(live_flags):
            parser.error("pass --mock or all of --api-key/--base-url/--model")
        try:
            document = extract_live(args.api_key, args.base_url, args.model)
        except ValueError as exc:
            logger.error(str(exc))
            sys.exit(1)
    else:
        logger.info("Mock mode: %s", MOCK_RULES.strip().splitlines()[0])
        document = build_mock_document()
        try:
            validate_document(document)
        except ValueError as exc:
            logger.error("mock rule table violates schema: %s", exc)
            sys.exit(1)

    text = json.dumps(document, ensure_ascii=False, indent=2) + "\n"
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(text, encoding="utf-8")
        logger.info(
            "Wrote %d entities / %d relationships to %s",
            len(document["entities"]),
            len(document["relationships"]),
            args.output,
        )
    else:
        sys.stdout.write(text)


if __name__ == "__main__":
    main()
