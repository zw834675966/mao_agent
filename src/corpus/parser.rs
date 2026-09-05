use crate::corpus::cleaner::clean_cjk_spaces;
use crate::error::{Result, VectorError};
use crate::model::{Document, DocumentMetadata, HistoricalPeriod};
use regex::Regex;
use std::path::Path;
use std::sync::LazyLock;

static FRONTMATTER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)^---\r?\n(.*?)\r?\n---\r?\n(.*)$").unwrap());

static HEADNOTE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    // Matches headnotes like 〔这是毛泽东同志在...〕 or ［这是毛泽东...］ or 【题注】
    Regex::new(r"(?s)^\s*(〔.*?〕|［.*?］|【.*?】|\*这是.*?\*)\s*\n(.*)$").unwrap()
});

static FN_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:\[\d+\]|〔\d+〕|\*\s*注\s*释|\*\s*题\s*注)\s*(.*)$").unwrap()
});

/// Parser for historical Markdown documents with YAML frontmatter.
pub struct MarkdownParser;

impl MarkdownParser {
    /// Parse a Markdown string with YAML frontmatter into a Document.
    pub fn parse_str(content: &str, file_path: Option<&str>) -> Result<Document> {
        let (metadata, raw_body) = if let Some(caps) = FRONTMATTER_REGEX.captures(content) {
            let yaml_str = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let body_str = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            let meta: DocumentMetadata = serde_yaml::from_str(yaml_str).map_err(|e| {
                VectorError::FrontmatterError(format!("Failed to parse YAML frontmatter: {e}"))
            })?;
            (meta, body_str)
        } else {
            // Default fallback if no frontmatter is found
            let fallback_title = file_path
                .and_then(|p| Path::new(p).file_stem())
                .and_then(|s| s.to_str())
                .unwrap_or("未命名文献")
                .to_string();
            let meta = DocumentMetadata {
                title: fallback_title,
                author: "毛泽东".to_string(),
                date: "未知".to_string(),
                ..Default::default()
            };
            (meta, content)
        };

        // Determine historical period enum
        let period_enum = if !metadata.period.is_empty() {
            HistoricalPeriod::from_str_or_date(&metadata.period)
        } else if !metadata.date.is_empty() {
            HistoricalPeriod::from_str_or_date(&metadata.date)
        } else {
            HistoricalPeriod::Unknown
        };

        // Clean body text
        let cleaned_body = clean_cjk_spaces(raw_body);

        // Separate headnote if present
        let (headnote, body_without_headnote) =
            if let Some(caps) = HEADNOTE_REGEX.captures(&cleaned_body) {
                let note = caps.get(1).map(|m| m.as_str().trim().to_string());
                let remaining = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                (note, remaining.to_string())
            } else {
                (None, cleaned_body)
            };

        // Extract footnotes (e.g. [1] ..., 〔1〕 ...)
        let (final_content, footnotes) = extract_footnotes(&body_without_headnote);

        // Deterministic document ID using SHA256 of title + date
        let doc_id = generate_doc_id(&metadata.title, &metadata.date, &metadata.volume);

        Ok(Document {
            id: doc_id,
            metadata,
            period_enum,
            headnote,
            content: final_content,
            footnotes,
            file_path: file_path.map(|s| s.to_string()),
        })
    }

    /// Read and parse a file from disk.
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Document> {
        let p = path.as_ref();
        let content = std::fs::read_to_string(p).map_err(VectorError::Io)?;
        Self::parse_str(&content, p.to_str())
    }
}

fn generate_doc_id(title: &str, date: &str, volume: &str) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(title.as_bytes());
    hasher.update(b"\0");
    hasher.update(date.as_bytes());
    hasher.update(b"\0");
    hasher.update(volume.as_bytes());
    let result = hasher.finalize();
    let num = u64::from_be_bytes(result[..8].try_into().expect("SHA-256 digest is 32 bytes"));
    format!("doc_{num:016x}")
}

fn extract_footnotes(text: &str) -> (String, Vec<String>) {
    let mut footnotes = Vec::new();
    let mut content_lines = Vec::new();

    let mut in_footnote_section = false;

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# 注释") || trimmed.starts_with("## 注释") || trimmed == "注释"
        {
            in_footnote_section = true;
            continue;
        }

        if in_footnote_section {
            if !trimmed.is_empty() {
                footnotes.push(trimmed.to_string());
            }
        } else if let Some(caps) = FN_REGEX.captures(trimmed) {
            if let Some(note) = caps.get(1) {
                footnotes.push(note.as_str().to_string());
            }
        } else {
            content_lines.push(line);
        }
    }

    (content_lines.join("\n").trim().to_string(), footnotes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter_document() {
        let raw = r#"---
title: "论持久战"
author: "毛泽东"
date: "1938-05-26"
period: "抗日战争时期"
volume: "毛泽东选集第二卷"
category: "军事战略"
tags:
  - "持久战"
  - "抗日战争"
---

〔这是毛泽东同志于一九三八年五月二十六日至六月三日在延安抗日战争研究会的讲演。〕

# 问题的提起

一、中国向何处去？

二、为什么是持久战？
"#;

        let doc = MarkdownParser::parse_str(raw, Some("test/lun_chi_jiu_zhan.md")).unwrap();
        assert_eq!(doc.metadata.title, "论持久战");
        assert_eq!(doc.metadata.date, "1938-05-26");
        assert_eq!(doc.period_enum, HistoricalPeriod::WarOfResistance);
        assert_eq!(doc.metadata.volume, "毛泽东选集第二卷");
        assert!(doc.headnote.is_some());
        assert!(doc.headnote.unwrap().contains("延安抗日战争研究会"));
        assert!(doc.content.contains("为什么是持久战"));
        assert_eq!(doc.metadata.tags, vec!["持久战", "抗日战争"]);
    }
}
