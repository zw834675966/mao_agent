use crate::model::{Document, DocumentChunk};
use unicode_segmentation::UnicodeSegmentation;

/// Configuration options for contextual semantic chunking.
#[derive(Debug, Clone)]
pub struct ChunkerConfig {
    /// Target chunk size in characters (excluding breadcrumb header).
    pub max_chars: usize,
    /// Minimum chunk size in characters before merging with adjacent paragraphs.
    pub min_chars: usize,
    /// Overlap characters between chunks for context continuity.
    pub overlap_chars: usize,
    /// Whether to inject the breadcrumb header (`【出处】... 【正文】...`).
    pub inject_context_header: bool,
}

impl Default for ChunkerConfig {
    fn default() -> Self {
        Self {
            max_chars: 600,
            min_chars: 120,
            overlap_chars: 60,
            inject_context_header: true,
        }
    }
}

/// Chunker specialized for Chinese historical and political texts.
pub struct ChineseSemanticChunker {
    config: ChunkerConfig,
}

impl ChineseSemanticChunker {
    pub fn new(config: ChunkerConfig) -> Self {
        Self { config }
    }

    /// Split a document into semantic, context-enriched chunks.
    pub fn chunk_document(&self, doc: &Document) -> Vec<DocumentChunk> {
        let raw_paragraphs = self.extract_sections(&doc.content);
        if raw_paragraphs.is_empty() {
            return Vec::new();
        }

        // Hard-split oversized paragraphs first: a single paragraph longer than
        // `max_chars` would otherwise pass through unsplit (giant chunks blow
        // past remote embedding input limits, e.g. HTTP 400 from the API).
        let mut paragraphs: Vec<(Vec<String>, String)> = Vec::with_capacity(raw_paragraphs.len());
        for (section_path, para) in raw_paragraphs {
            if para.chars().count() <= self.config.max_chars || self.config.max_chars == 0 {
                paragraphs.push((section_path, para));
                continue;
            }
            let chars: Vec<char> = para.chars().collect();
            for piece in chars.chunks(self.config.max_chars) {
                paragraphs.push((section_path.clone(), piece.iter().collect()));
            }
        }

        // Build raw chunks
        let mut raw_chunks = Vec::new();
        let mut current_buf = String::new();
        let mut current_section = Vec::new();

        for (section_path, para) in paragraphs {
            let para_len = para.chars().count();
            let section_changed = !current_section.is_empty()
                && section_path != current_section
                && current_buf.chars().count() >= self.config.min_chars;

            if current_buf.is_empty() {
                current_section = section_path;
                current_buf.push_str(&para);
            } else if !section_changed
                && current_buf.chars().count() + para_len + 2 <= self.config.max_chars
            {
                current_buf.push_str("\n\n");
                current_buf.push_str(&para);
            } else {
                // Emit current buffer
                if !current_buf.is_empty() {
                    raw_chunks.push((current_section.clone(), current_buf.clone()));
                }

                // Setup overlap from previous text if configured (only if within same section).
                // The overlap prefix must not push the buffer past max_chars.
                if !section_changed
                    && self.config.overlap_chars > 0
                    && current_buf.chars().count() > self.config.overlap_chars
                {
                    let graphemes: Vec<&str> = current_buf.graphemes(true).collect();
                    let overlap_start = graphemes.len().saturating_sub(self.config.overlap_chars);
                    let overlap_str = graphemes[overlap_start..].join("");
                    let candidate = format!("{overlap_str}\n\n{para}");
                    if candidate.chars().count() <= self.config.max_chars {
                        current_buf = candidate;
                    } else {
                        current_buf = para;
                    }
                } else {
                    current_buf = para;
                }
                current_section = section_path;
            }
        }

        if !current_buf.is_empty() {
            raw_chunks.push((current_section, current_buf));
        }

        let total_chunks = raw_chunks.len();
        let mut result_chunks = Vec::with_capacity(total_chunks);

        for (idx, (section_path, raw_text)) in raw_chunks.into_iter().enumerate() {
            let chunk_id = format!("{}_chunk_{:04}", doc.id, idx);
            let char_count = raw_text.chars().count();

            // Construct contextualized representation for embedding and retrieval
            let contextualized_text = if self.config.inject_context_header {
                self.build_contextualized_text(doc, &section_path, &raw_text)
            } else {
                raw_text.clone()
            };

            result_chunks.push(DocumentChunk {
                chunk_id,
                doc_id: doc.id.clone(),
                doc_title: doc.metadata.title.clone(),
                author: doc.metadata.author.clone(),
                period: doc.period_enum,
                date: doc.metadata.date.clone(),
                volume: doc.metadata.volume.clone(),
                category: doc.metadata.category.clone(),
                tags: doc.metadata.tags.clone(),
                chunk_index: idx,
                total_chunks,
                char_count,
                raw_text,
                contextualized_text,
                section_path,
            });
        }

        result_chunks
    }

    /// Build the contextual breadcrumb header.
    fn build_contextualized_text(
        &self,
        doc: &Document,
        section_path: &[String],
        raw_text: &str,
    ) -> String {
        let mut header = String::new();
        header.push_str(&format!("【文献】《{}》", doc.metadata.title));

        let mut meta_parts = Vec::new();
        if !doc.metadata.date.is_empty() && doc.metadata.date != "未知" {
            meta_parts.push(doc.metadata.date.clone());
        }
        if !doc.metadata.period.is_empty() {
            meta_parts.push(doc.metadata.period.clone());
        } else if doc.period_enum != crate::model::HistoricalPeriod::Unknown {
            meta_parts.push(doc.period_enum.as_str().to_string());
        }
        if !doc.metadata.volume.is_empty() {
            meta_parts.push(doc.metadata.volume.clone());
        }

        if !meta_parts.is_empty() {
            header.push_str(&format!("（{}）", meta_parts.join(" · ")));
        }

        if !section_path.is_empty() {
            header.push_str(&format!("\n【章节】{}", section_path.join(" > ")));
        }

        format!("{}\n【正文】{}", header, raw_text)
    }

    /// Parse markdown headings and paragraph sections.
    fn extract_sections(&self, text: &str) -> Vec<(Vec<String>, String)> {
        let mut sections = Vec::new();
        let mut current_path: Vec<String> = Vec::new();
        let mut current_para = Vec::new();

        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                if !current_para.is_empty() {
                    let para_text = current_para.join("\n");
                    sections.push((current_path.clone(), para_text));
                    current_para.clear();
                }
                continue;
            }

            // Check markdown heading levels
            if trimmed.starts_with('#') {
                if !current_para.is_empty() {
                    let para_text = current_para.join("\n");
                    sections.push((current_path.clone(), para_text));
                    current_para.clear();
                }

                let level = trimmed.chars().take_while(|&c| c == '#').count();
                let heading_title = trimmed[level..].trim().to_string();

                // Truncate path to level - 1
                if current_path.len() >= level {
                    current_path.truncate(level - 1);
                }
                current_path.push(heading_title);
            } else {
                current_para.push(trimmed);
            }
        }

        if !current_para.is_empty() {
            let para_text = current_para.join("\n");
            sections.push((current_path, para_text));
        }

        sections
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DocumentMetadata, HistoricalPeriod};

    #[test]
    fn test_chunk_document_with_context_header() {
        let doc = Document {
            id: "doc_test".to_string(),
            metadata: DocumentMetadata {
                title: "中国革命战争的战略问题".to_string(),
                author: "毛泽东".to_string(),
                date: "1936-12".to_string(),
                period: "土地革命战争时期".to_string(),
                volume: "毛泽东选集第一卷".to_string(),
                category: "军事".to_string(),
                tags: vec!["军事战略".to_string()],
                ..Default::default()
            },
            period_enum: HistoricalPeriod::AgrarianRevolutionaryWar,
            headnote: None,
            content: r#"# 第一章 如何研究战争
## 第一节 主要规律
中国革命战争的主要规律是：第一，中国是一个政治经济发展不平衡的半殖民地大国；第二，敌人的强大；第三，红军的弱小；第四，共产党的领导和土地革命。

这些特点规定了中国革命战争的指导路线。
"#
            .to_string(),
            footnotes: vec![],
            file_path: None,
        };

        let chunker = ChineseSemanticChunker::new(ChunkerConfig::default());
        let chunks = chunker.chunk_document(&doc);

        assert!(!chunks.is_empty());
        let first = &chunks[0];
        assert_eq!(first.doc_title, "中国革命战争的战略问题");
        assert_eq!(first.period, HistoricalPeriod::AgrarianRevolutionaryWar);
        assert!(
            first
                .contextualized_text
                .contains("【文献】《中国革命战争的战略问题》")
        );
        assert!(
            first
                .contextualized_text
                .contains("第一章 如何研究战争 > 第一节 主要规律")
        );
        assert!(
            first
                .contextualized_text
                .contains("中国革命战争的主要规律是")
        );
    }

    #[test]
    fn test_oversized_paragraph_is_hard_split_to_max_chars() {
        // Regression: a single paragraph longer than max_chars must not pass
        // through as one giant chunk (it blows past remote embedding input
        // limits, e.g. SiliconFlow HTTP 400 code 20015).
        let giant = "分".repeat(2000);
        let doc = Document {
            id: "doc_giant".to_string(),
            metadata: DocumentMetadata {
                title: "超长段落".to_string(),
                ..Default::default()
            },
            period_enum: HistoricalPeriod::Unknown,
            headnote: None,
            content: giant,
            footnotes: vec![],
            file_path: None,
        };

        let chunker = ChineseSemanticChunker::new(ChunkerConfig {
            max_chars: 600,
            min_chars: 100,
            overlap_chars: 50,
            inject_context_header: true,
        });
        let chunks = chunker.chunk_document(&doc);

        assert!(chunks.len() >= 3, "expected splits, got {}", chunks.len());
        for c in &chunks {
            assert!(
                c.raw_text.chars().count() <= 600,
                "chunk too long: {}",
                c.raw_text.chars().count()
            );
            assert!(c.contextualized_text.contains("【文献】《超长段落》"));
        }
    }
}
