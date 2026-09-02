use crate::model::DocumentChunk;
use serde::{Deserialize, Serialize};
use strsim::jaro_winkler;

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

            // 2. Sliding window Fuzzy Match (Jaro-Winkler)
            let quote_len = norm_quote.chars().count();
            let chunk_chars: Vec<char> = norm_chunk_text.chars().collect();

            if chunk_chars.len() >= quote_len {
                let win_size = (quote_len + 5).min(chunk_chars.len());
                if win_size >= quote_len {
                    for window in chunk_chars.windows(win_size) {
                        let window_str: String = window.iter().collect();
                        let sim = jaro_winkler(&norm_quote, &window_str) as f32;
                        if sim > best_confidence {
                            best_confidence = sim;
                            best_chunk_id = Some(chunk.chunk_id.clone());
                            best_snippet = Some(chunk.raw_text.clone());
                        }
                    }
                }
            }
        }

        if best_confidence >= self.min_confidence {
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
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::HistoricalPeriod;

    #[test]
    fn test_citation_verification_exact_and_hallucination() {
        let verifier = CitationVerifier::default();

        let chunk = DocumentChunk {
            chunk_id: "c1".to_string(),
            doc_id: "doc_1".to_string(),
            doc_title: "论持久战".to_string(),
            author: "毛泽东".to_string(),
            period: HistoricalPeriod::WarOfResistance,
            date: "1938-05-26".to_string(),
            volume: "第二卷".to_string(),
            category: "军事".to_string(),
            tags: vec![],
            chunk_index: 0,
            total_chunks: 1,
            char_count: 80,
            raw_text: "兵民是胜利之本。战争的伟力之最深厚的根源，存在于民众之中。".to_string(),
            contextualized_text: "兵民是胜利之本。战争的伟力之最深厚的根源，存在于民众之中。"
                .to_string(),
            section_path: vec![],
        };

        // Real quote
        let real_rep = verifier.verify_quote(
            "兵民是胜利之本，战争的伟力之最深厚的根源，存在于民众之中",
            "论持久战",
            std::slice::from_ref(&chunk),
        );
        assert!(real_rep.is_verified);
        assert!(real_rep.match_confidence >= 0.95);

        // Fake hallucinated quote
        let fake_rep =
            verifier.verify_quote("互联网技术是未来战争胜负的决定性力量", "论持久战", &[chunk]);
        assert!(!fake_rep.is_verified);
        assert!(fake_rep.warning.is_some());
    }
}
