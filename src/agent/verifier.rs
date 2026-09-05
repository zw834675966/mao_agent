use crate::model::DocumentChunk;
use serde::{Deserialize, Serialize};
use strsim::{jaro_winkler, levenshtein};

/// Verification report for a citation quote extracted from LLM output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationReport {
    pub quote: String,
    pub claimed_doc_title: String,
    pub is_verified: bool,
    pub match_confidence: f32,
    pub matched_chunk_id: Option<String>,
    pub matched_snippet: Option<String>,
    pub warning: Option<String>,
}

/// Hardcore Citation Verifier preventing hallucinations by checking physical substring matches in corpus.
///
/// Matching policy (default):
/// 1. Exact normalized substring → `is_verified=true`, confidence 1.0
/// 2. Equal-length sliding-window Jaro–Winkler ≥ `min_confidence`, gated by a small
///    Levenshtein edit budget (0 edits for short quotes <40 chars; 1 edit otherwise).
///    This blocks synonym swaps / clause reorders that raw JW≥0.85 would falsely accept.
pub struct CitationVerifier {
    pub min_confidence: f32,
    pub min_quote_len: usize,
}

impl Default for CitationVerifier {
    fn default() -> Self {
        Self {
            min_confidence: 0.85,
            min_quote_len: 6,
        }
    }
}

impl CitationVerifier {
    pub fn new(min_confidence: f32, min_quote_len: usize) -> Self {
        Self {
            min_confidence,
            min_quote_len,
        }
    }

    /// Normalize Chinese text by removing whitespace and punctuation for strict fuzzy/exact matching.
    pub fn normalize_for_matching(text: &str) -> String {
        text.chars()
            .filter(|c| !c.is_whitespace() && !is_chinese_punct(*c) && !c.is_ascii_punctuation())
            .collect()
    }

    /// Max Levenshtein edits allowed for a non-exact fuzzy accept.
    fn allowed_fuzzy_edits(norm_quote_chars: usize) -> usize {
        if norm_quote_chars < 40 {
            // Short quotes: require exact (edit budget 0). Blocks mild synonym/reorder false accepts.
            0
        } else {
            // Longer quotes: allow a single OCR-like substitution/insertion/deletion.
            1
        }
    }

    /// Verify a single quote against a pool of retrieved chunks.
    pub fn verify_quote(
        &self,
        quote: &str,
        claimed_title: &str,
        chunks: &[DocumentChunk],
    ) -> VerificationReport {
        let norm_quote = Self::normalize_for_matching(quote);

        if norm_quote.chars().count() < self.min_quote_len {
            return VerificationReport {
                quote: quote.to_string(),
                claimed_doc_title: claimed_title.to_string(),
                is_verified: false,
                match_confidence: 0.0,
                matched_chunk_id: None,
                matched_snippet: None,
                warning: Some("引文过短（少于6个有效字），无法进行高置信度原著比对。".to_string()),
            };
        }

        let mut best_chunk_id = None;
        let mut best_confidence = 0.0f32;
        let mut best_snippet = None;
        let mut best_window: Option<String> = None;

        for chunk in chunks {
            // Priority check: Title matches
            let title_matches =
                chunk.doc_title.contains(claimed_title) || claimed_title.contains(&chunk.doc_title);
            let norm_chunk_text = Self::normalize_for_matching(&chunk.raw_text);

            // 1. Exact Substring Match (100% confidence)
            if norm_chunk_text.contains(&norm_quote) {
                return VerificationReport {
                    quote: quote.to_string(),
                    claimed_doc_title: claimed_title.to_string(),
                    is_verified: true,
                    match_confidence: 1.0,
                    matched_chunk_id: Some(chunk.chunk_id.clone()),
                    matched_snippet: Some(chunk.raw_text.clone()),
                    warning: if !title_matches {
                        Some(format!(
                            "引文内容在《{}》中匹配成功，但标注出处为《{}》",
                            chunk.doc_title, claimed_title
                        ))
                    } else {
                        None
                    },
                };
            }

            // 2. Sliding window Fuzzy Match (equal-length Jaro-Winkler)
            let quote_len = norm_quote.chars().count();
            let chunk_chars: Vec<char> = norm_chunk_text.chars().collect();

            if chunk_chars.len() >= quote_len {
                for window in chunk_chars.windows(quote_len) {
                    let window_str: String = window.iter().collect();
                    let sim = jaro_winkler(&norm_quote, &window_str) as f32;
                    if sim > best_confidence {
                        best_confidence = sim;
                        best_chunk_id = Some(chunk.chunk_id.clone());
                        best_snippet = Some(chunk.raw_text.clone());
                        best_window = Some(window_str);
                    }
                }
            }
        }

        let quote_len = norm_quote.chars().count();
        let allowed_edits = Self::allowed_fuzzy_edits(quote_len);
        let edits = best_window
            .as_ref()
            .map(|w| levenshtein(&norm_quote, w))
            .unwrap_or(usize::MAX);

        if best_confidence >= self.min_confidence && edits <= allowed_edits {
            VerificationReport {
                quote: quote.to_string(),
                claimed_doc_title: claimed_title.to_string(),
                is_verified: true,
                match_confidence: best_confidence,
                matched_chunk_id: best_chunk_id,
                matched_snippet: best_snippet,
                warning: None,
            }
        } else {
            VerificationReport {
                quote: quote.to_string(),
                claimed_doc_title: claimed_title.to_string(),
                is_verified: false,
                match_confidence: best_confidence,
                matched_chunk_id: None,
                matched_snippet: None,
                warning: Some(format!(
                    "引文未在召回的语料段落中检索到真实出处（最高相似度: {:.2}%），存在引语幻觉风险。",
                    best_confidence * 100.0
                )),
            }
        }
    }
}

fn is_chinese_punct(c: char) -> bool {
    matches!(
        c,
        '，' | '。'
            | '、'
            | '；'
            | '：'
            | '？'
            | '！'
            | '“'
            | '”'
            | '‘'
            | '’'
            | '（'
            | '）'
            | '《'
            | '》'
            | '【'
            | '】'
            | '—'
            | '…'
            | '·'
            | '～'
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::HistoricalPeriod;

    fn sample_chunk(chunk_id: &str, title: &str, raw_text: &str) -> DocumentChunk {
        DocumentChunk {
            chunk_id: chunk_id.to_string(),
            doc_id: format!("doc_{chunk_id}"),
            doc_title: title.to_string(),
            author: "毛泽东".to_string(),
            period: HistoricalPeriod::WarOfResistance,
            date: "1938-05-26".to_string(),
            volume: "第二卷".to_string(),
            category: "军事".to_string(),
            tags: vec![],
            chunk_index: 0,
            total_chunks: 1,
            char_count: raw_text.chars().count(),
            raw_text: raw_text.to_string(),
            contextualized_text: raw_text.to_string(),
            section_path: vec![],
        }
    }

    #[test]
    fn test_citation_verification_exact_and_hallucination() {
        let verifier = CitationVerifier::default();

        let chunk = sample_chunk(
            "c1",
            "论持久战",
            "兵民是胜利之本。战争的伟力之最深厚的根源，存在于民众之中。",
        );

        // Real quote
        let real_rep = verifier.verify_quote(
            "兵民是胜利之本，战争的伟力之最深厚的根源，存在于民众之中",
            "论持久战",
            std::slice::from_ref(&chunk),
        );
        assert!(real_rep.is_verified);
        assert_eq!(real_rep.match_confidence, 1.0);

        // Fake hallucinated quote
        let fake_rep =
            verifier.verify_quote("互联网技术是未来战争胜负的决定性力量", "论持久战", &[chunk]);
        assert!(!fake_rep.is_verified);
        assert!(fake_rep.warning.is_some());
    }

    #[test]
    fn test_adversarial_citation_rejection_suite() {
        let verifier = CitationVerifier::default();

        // Prefer real corpus sample text (论持久战 + 矛盾论).
        let corpus_persist = include_str!("../../corpus/lun_chi_jiu_zhan.md");
        let corpus_contradiction = include_str!("../../corpus/mao_dun_lun.md");

        let persist_quote = "兵民是胜利之本。战争的伟力之最深厚的根源，存在于民众之中。";
        assert!(
            corpus_persist.contains(persist_quote),
            "fixture quote must exist in lun_chi_jiu_zhan.md"
        );

        let contradiction_quote = "捉住了这个主要矛盾，一切问题就迎刃而解了。";
        assert!(
            corpus_contradiction.contains(contradiction_quote),
            "fixture quote must exist in mao_dun_lun.md"
        );

        let persist_chunk = sample_chunk("persist", "论持久战", persist_quote);
        let contradiction_chunk = sample_chunk(
            "contradiction",
            "矛盾论",
            "研究任何过程，如果是存在着两个以上矛盾的复杂过程的话，必须用全力找出它的主要矛盾。捉住了这个主要矛盾，一切问题就迎刃而解了。",
        );
        let chunks = [persist_chunk.clone(), contradiction_chunk.clone()];

        // 1) Exact real quote → verified + conf == 1.0
        let exact = verifier.verify_quote(
            "兵民是胜利之本。战争的伟力之最深厚的根源，存在于民众之中。",
            "论持久战",
            &chunks,
        );
        assert!(exact.is_verified, "exact corpus quote must verify");
        assert_eq!(exact.match_confidence, 1.0);
        assert_eq!(exact.matched_chunk_id.as_deref(), Some("persist"));

        // Adversarial mutations — all must reject under default CitationVerifier.
        let adversarial: Vec<(&str, &str)> = vec![
            // Synonym / 近义替换 (兵民→军民)
            (
                "synonym_swap",
                "军民是胜利之本。战争的伟力之最深厚的根源，存在于民众之中。",
            ),
            // 语序颠倒 (clause reorder)
            (
                "reorder",
                "存在于民众之中，战争的伟力之最深厚的根源，兵民是胜利之本。",
            ),
            (
                "reorder_mild",
                "战争的伟力之最深厚的根源，兵民是胜利之本，存在于民众之中。",
            ),
            // 生造词 / fabricated
            ("fabricated", "互联网技术是未来战争胜负的决定性力量"),
            // 跨篇拼接 (persist + contradiction)
            (
                "cross_doc_splice",
                "兵民是胜利之本。捉住了这个主要矛盾，一切问题就迎刃而解了。",
            ),
            // 随机噪声
            ("random_noise", "xyz随机噪声abcdef战争胜负乱码测试段落"),
        ];

        let mut reject_count = 0usize;
        for (label, quote) in &adversarial {
            let rep = verifier.verify_quote(quote, "论持久战", &chunks);
            assert!(
                !rep.is_verified,
                "adversarial `{label}` must be rejected, got conf={}",
                rep.match_confidence
            );
            reject_count += 1;
        }
        assert_eq!(reject_count, adversarial.len());
        assert_eq!(
            reject_count as f32 / adversarial.len() as f32,
            1.0,
            "100% adversarial reject rate required"
        );

        // Cross-doc splice must also fail when only one source chunk is present.
        let splice_only_persist = verifier.verify_quote(
            "兵民是胜利之本。捉住了这个主要矛盾，一切问题就迎刃而解了。",
            "论持久战",
            std::slice::from_ref(&persist_chunk),
        );
        assert!(!splice_only_persist.is_verified);
    }
}
