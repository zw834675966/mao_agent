//! One-shot generator: corpus → ChineseSemanticChunker → queries.jsonl
//! Run: cargo run --no-default-features --example gen_retrieval_queries

use mao_agent::corpus::{ChineseSemanticChunker, ChunkerConfig, CorpusScanner};
use mao_agent::model::{DocumentChunk, HistoricalPeriod};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct QueryFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<String>,
}

#[derive(Serialize)]
struct GoldQuery {
    query: String,
    expected_chunk_ids: Vec<String>,
    filter: Option<QueryFilter>,
}

fn period_label(p: HistoricalPeriod) -> Option<&'static str> {
    match p {
        HistoricalPeriod::WarOfResistance => Some("抗日战争时期"),
        HistoricalPeriod::AgrarianRevolutionaryWar => Some("土地革命战争时期"),
        HistoricalPeriod::WarOfLiberation => Some("全国解放战争时期"),
        HistoricalPeriod::SocialistConstruction => Some("社会主义革命和建设时期"),
        HistoricalPeriod::FirstRevolutionaryWar => Some("第一次国内革命战争时期"),
        _ => None,
    }
}

fn is_core_period(p: HistoricalPeriod) -> bool {
    matches!(
        p,
        HistoricalPeriod::WarOfResistance
            | HistoricalPeriod::AgrarianRevolutionaryWar
            | HistoricalPeriod::WarOfLiberation
            | HistoricalPeriod::SocialistConstruction
            | HistoricalPeriod::FirstRevolutionaryWar
    )
}

fn key_snippets(raw: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    for ch in raw.chars() {
        buf.push(ch);
        if matches!(ch, '。' | '！' | '？' | '；' | '\n') {
            let t = buf
                .trim()
                .trim_matches(|c: char| matches!(c, '。' | '！' | '？' | '；' | ' ' | '\t' | '\r'))
                .to_string();
            let n = t.chars().count();
            // Prefer complete sentences of readable length
            if (16..=72).contains(&n) && !t.starts_with('#') {
                out.push(t);
            }
            buf.clear();
        }
    }
    let t = buf.trim().to_string();
    let n = t.chars().count();
    if (16..=72).contains(&n) {
        out.push(t);
    }
    out
}

/// Pick a short distinctive phrase (8–28 chars) from a sentence for the question stem.
fn phrase_from_sentence(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= 28 {
        return s.to_string();
    }
    // Take sentence head — more natural Chinese question stems
    chars.iter().take(24).collect()
}

fn make_questions(chunk: &DocumentChunk, sentence: &str, variant: usize) -> Option<String> {
    let title = &chunk.doc_title;
    let stem = phrase_from_sentence(sentence);
    if stem.chars().count() < 8 {
        return None;
    }
    let q = match variant % 5 {
        0 => format!("《{title}》中「{stem}」相关论述的核心观点是什么？"),
        1 => format!("根据《{title}》，如何理解「{stem}」？"),
        2 => format!("毛泽东在《{title}》里关于「{stem}」提出了哪些论断？"),
        3 => format!("《{title}》中与「{stem}」直接相关的原文论述是什么？"),
        _ => format!("请检索《{title}》中阐述「{stem}」的段落。"),
    };
    Some(q)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let corpus_dir = Path::new("corpus");
    let out_path = Path::new("evals/retrieval/queries.jsonl");
    fs::create_dir_all(out_path.parent().unwrap())?;

    let docs = CorpusScanner::load_documents_from_dir(corpus_dir)?;
    // Match production ingest (main.rs handle_ingest): max_chars=600, min=100, overlap=50
    let chunker = ChineseSemanticChunker::new(ChunkerConfig {
        max_chars: 600,
        min_chars: 100,
        overlap_chars: 50,
        inject_context_header: true,
    });

    let mut all_chunks: Vec<DocumentChunk> = Vec::new();
    for doc in &docs {
        all_chunks.extend(chunker.chunk_document(doc));
    }
    eprintln!("docs={} chunks={}", docs.len(), all_chunks.len());

    let mut queries: Vec<GoldQuery> = Vec::new();
    let mut period_counts: HashMap<&'static str, usize> = HashMap::new();
    let mut used_queries: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut doc_counts: HashMap<String, usize> = HashMap::new();

    // Prefer core-period chunks first for coverage
    let mut ordered: Vec<&DocumentChunk> = all_chunks.iter().filter(|c| is_core_period(c.period)).collect();
    let rest: Vec<&DocumentChunk> = all_chunks.iter().filter(|c| !is_core_period(c.period)).collect();
    ordered.extend(rest);

    let mut variant = 0usize;
    for round in 0..6 {
        for chunk in &ordered {
            if queries.len() >= 105 {
                break;
            }
            let snippets = key_snippets(&chunk.raw_text);
            if snippets.is_empty() {
                continue;
            }
            let snip = &snippets[round % snippets.len()];
            let Some(q) = make_questions(chunk, snip, variant) else {
                variant += 1;
                continue;
            };
            variant += 1;
            if !used_queries.insert(q.clone()) {
                continue;
            }
            // Cap per-doc so all 15 docs get coverage
            let dc = doc_counts.entry(chunk.doc_id.clone()).or_insert(0);
            if *dc >= 10 && queries.len() > 60 {
                continue;
            }
            *dc += 1;

            let filter = if variant % 5 == 0 {
                period_label(chunk.period).map(|p| QueryFilter {
                    period: Some(p.to_string()),
                })
            } else {
                None
            };
            if let Some(label) = period_label(chunk.period) {
                *period_counts.entry(label).or_insert(0) += 1;
            }
            queries.push(GoldQuery {
                query: q,
                expected_chunk_ids: vec![chunk.chunk_id.clone()],
                filter,
            });
        }
    }

    let required = [
        HistoricalPeriod::WarOfResistance,
        HistoricalPeriod::AgrarianRevolutionaryWar,
        HistoricalPeriod::WarOfLiberation,
        HistoricalPeriod::SocialistConstruction,
    ];
    for period in required {
        let label = period_label(period).unwrap();
        let mut n = *period_counts.get(label).unwrap_or(&0);
        if n >= 5 {
            continue;
        }
        for chunk in &all_chunks {
            if chunk.period != period || n >= 5 {
                continue;
            }
            for (i, snip) in key_snippets(&chunk.raw_text).iter().enumerate() {
                if n >= 5 {
                    break;
                }
                let Some(q) = make_questions(chunk, snip, i + 31) else {
                    continue;
                };
                if !used_queries.insert(q.clone()) {
                    continue;
                }
                queries.push(GoldQuery {
                    query: q,
                    expected_chunk_ids: vec![chunk.chunk_id.clone()],
                    filter: Some(QueryFilter {
                        period: Some(label.to_string()),
                    }),
                });
                n += 1;
                *period_counts.entry(label).or_insert(0) += 1;
            }
        }
    }

    if queries.len() > 110 {
        queries.truncate(110);
    }
    if !(90..=110).contains(&queries.len()) {
        eprintln!("WARNING: query count {} outside 90..=110", queries.len());
    }

    let mut out = String::new();
    for q in &queries {
        out.push_str(&serde_json::to_string(q)?);
        out.push('\n');
    }
    fs::write(out_path, out)?;

    eprintln!("wrote {} queries to {}", queries.len(), out_path.display());
    for (k, v) in &period_counts {
        eprintln!("  period {k}: {v}");
    }
    eprintln!("docs covered: {}", doc_counts.len());
    for (i, q) in queries.iter().take(5).enumerate() {
        eprintln!(
            "EXAMPLE{}: {} => {}",
            i + 1,
            q.query,
            q.expected_chunk_ids[0]
        );
    }
    let id_set: std::collections::HashSet<_> =
        all_chunks.iter().map(|c| c.chunk_id.clone()).collect();
    let ok = queries
        .iter()
        .filter(|q| q.expected_chunk_ids.iter().all(|id| id_set.contains(id)))
        .count();
    eprintln!("gold id validation: {ok}/{}", queries.len());
    Ok(())
}
