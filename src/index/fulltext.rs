use crate::error::{Result, VectorError};
use crate::index::tokenizer::JiebaTokenizer;
use crate::model::{DocumentChunk, VectorFilter};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;
use tantivy::collector::TopDocs;
use tantivy::query::{AllQuery, BooleanQuery, Occur, Query, QueryParser, RegexQuery, TermQuery};
use tantivy::schema::*;
use tantivy::{Index, IndexReader, IndexWriter, ReloadPolicy, TantivyDocument, Term};
use tracing::debug;

pub const JIEBA_TOKENIZER_NAME: &str = "jieba_chinese";

/// Result item from full-text BM25 search.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FullTextSearchResult {
    pub chunk_id: String,
    pub score: f32,
    pub rank: usize,
    pub chunk: DocumentChunk,
}

/// Full-text BM25 Index based on Tantivy and Jieba Chinese tokenization.
pub struct FullTextIndex {
    index: Index,
    reader: IndexReader,
    _schema: Schema,
    f_chunk_id: Field,
    f_doc_id: Field,
    f_title: Field,
    f_period: Field,
    f_date: Field,
    f_volume: Field,
    f_category: Field,
    f_body: Field,
    f_json_chunk: Field,
    writer: std::sync::Mutex<IndexWriter>,
}

pub type FullTextSchemaFields = (
    Field,
    Field,
    Field,
    Field,
    Field,
    Field,
    Field,
    Field,
    Field,
);

impl FullTextIndex {
    /// Create a new in-memory FullTextIndex.
    pub fn new_in_ram() -> Result<Self> {
        let (schema, fields) = Self::build_schema();
        let index = Index::create_in_ram(schema.clone());
        Self::init_with_index(index, schema, fields)
    }

    /// Create or open a FullTextIndex at a directory path.
    pub fn new_in_dir<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let dir_path = dir.as_ref();
        std::fs::create_dir_all(dir_path)?;

        let (schema, fields) = Self::build_schema();
        let mmap_dir = tantivy::directory::MmapDirectory::open(dir_path)
            .map_err(|e| VectorError::Io(std::io::Error::other(e.to_string())))?;

        let exists = Index::exists(&mmap_dir).unwrap_or(false);
        let index = if exists {
            Index::open(mmap_dir).map_err(|e| VectorError::Other(e.to_string()))?
        } else {
            Index::create_in_dir(dir_path, schema.clone())
                .map_err(|e| VectorError::Other(e.to_string()))?
        };

        Self::init_with_index(index, schema, fields)
    }

    fn build_schema() -> (Schema, FullTextSchemaFields) {
        let mut builder = Schema::builder();

        let text_indexing = TextFieldIndexing::default()
            .set_tokenizer(JIEBA_TOKENIZER_NAME)
            .set_index_option(IndexRecordOption::WithFreqsAndPositions);
        let text_options = TextOptions::default()
            .set_indexing_options(text_indexing)
            .set_stored();

        let f_chunk_id = builder.add_text_field("chunk_id", STRING | STORED);
        let f_doc_id = builder.add_text_field("doc_id", STRING | STORED);
        let f_title = builder.add_text_field("title", text_options.clone());
        let f_period = builder.add_text_field("period", STRING | STORED);
        let f_date = builder.add_text_field("date", STRING | STORED);
        let f_volume = builder.add_text_field("volume", STRING | STORED);
        let f_category = builder.add_text_field("category", STRING | STORED);
        let f_body = builder.add_text_field("body", text_options);
        let f_json_chunk = builder.add_text_field("json_chunk", STORED);

        let schema = builder.build();
        (
            schema,
            (
                f_chunk_id,
                f_doc_id,
                f_title,
                f_period,
                f_date,
                f_volume,
                f_category,
                f_body,
                f_json_chunk,
            ),
        )
    }

    fn init_with_index(index: Index, schema: Schema, fields: FullTextSchemaFields) -> Result<Self> {
        // Register Jieba tokenizer
        index
            .tokenizers()
            .register(JIEBA_TOKENIZER_NAME, JiebaTokenizer::new());

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .map_err(|e| VectorError::Other(e.to_string()))?;

        // 50 MB buffer for indexing
        let writer = index
            .writer(50_000_000)
            .map_err(|e| VectorError::Other(e.to_string()))?;

        Ok(Self {
            index,
            reader,
            _schema: schema,
            f_chunk_id: fields.0,
            f_doc_id: fields.1,
            f_title: fields.2,
            f_period: fields.3,
            f_date: fields.4,
            f_volume: fields.5,
            f_category: fields.6,
            f_body: fields.7,
            f_json_chunk: fields.8,
            writer: std::sync::Mutex::new(writer),
        })
    }

    /// Insert a DocumentChunk into the Tantivy full-text index.
    pub fn insert(&self, chunk: &DocumentChunk) -> Result<()> {
        let json_chunk =
            serde_json::to_string(chunk).map_err(|e| VectorError::Serialization(e.to_string()))?;

        let mut doc = TantivyDocument::default();
        doc.add_text(self.f_chunk_id, &chunk.chunk_id);
        doc.add_text(self.f_doc_id, &chunk.doc_id);
        doc.add_text(self.f_title, &chunk.doc_title);
        doc.add_text(self.f_period, chunk.period.as_str());
        doc.add_text(self.f_date, &chunk.date);
        doc.add_text(self.f_volume, &chunk.volume);
        doc.add_text(self.f_category, &chunk.category);
        doc.add_text(self.f_body, &chunk.contextualized_text);
        doc.add_text(self.f_json_chunk, &json_chunk);

        let writer = self
            .writer
            .lock()
            .map_err(|_| VectorError::Other("Writer lock failed".into()))?;
        writer
            .add_document(doc)
            .map_err(|e| VectorError::Other(e.to_string()))?;
        Ok(())
    }

    /// Batch insert multiple chunks and commit.
    pub fn insert_batch(&self, chunks: &[DocumentChunk]) -> Result<()> {
        for chunk in chunks {
            self.insert(chunk)?;
        }
        self.commit()?;
        Ok(())
    }

    /// Commit pending changes to make them searchable.
    pub fn commit(&self) -> Result<()> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|_| VectorError::Other("Writer lock failed".into()))?;
        writer
            .commit()
            .map_err(|e| VectorError::Other(e.to_string()))?;
        self.reader
            .reload()
            .map_err(|e| VectorError::Other(e.to_string()))?;
        Ok(())
    }

    /// Search using BM25 query with optional multi-attribute filters.
    pub fn search(
        &self,
        query_str: &str,
        top_k: usize,
        filter: Option<&VectorFilter>,
    ) -> Result<Vec<FullTextSearchResult>> {
        let searcher = self.reader.searcher();

        let query_parser = QueryParser::for_index(&self.index, vec![self.f_title, self.f_body]);
        let user_query = if query_str.trim().is_empty() {
            Box::new(AllQuery) as Box<dyn Query>
        } else {
            query_parser
                .parse_query(query_str)
                .map_err(|e| VectorError::Other(format!("BM25 query parse error: {e}")))?
        };

        // Combine with filter queries if specified
        let mut clauses: Vec<(Occur, Box<dyn Query>)> = vec![(Occur::Must, user_query)];

        if let Some(f) = filter {
            if let Some(p) = f.period {
                let term = Term::from_field_text(self.f_period, p.as_str());
                clauses.push((
                    Occur::Must,
                    Box::new(TermQuery::new(term, IndexRecordOption::Basic)),
                ));
            }
            if let Some(ref vol) = f.volume {
                let candidates = generate_volume_candidates(vol);
                let mut matched_terms = Vec::new();
                for cand in &candidates {
                    let term = Term::from_field_text(self.f_volume, cand);
                    if searcher.doc_freq(&term).unwrap_or(0) > 0 {
                        matched_terms.push(term);
                    }
                }

                if !matched_terms.is_empty() {
                    if matched_terms.len() == 1 {
                        clauses.push((
                            Occur::Must,
                            Box::new(TermQuery::new(
                                matched_terms.remove(0),
                                IndexRecordOption::Basic,
                            )),
                        ));
                    } else {
                        let term_queries: Vec<(Occur, Box<dyn Query>)> = matched_terms
                            .into_iter()
                            .map(|t| {
                                (
                                    Occur::Should,
                                    Box::new(TermQuery::new(t, IndexRecordOption::Basic))
                                        as Box<dyn Query>,
                                )
                            })
                            .collect();
                        clauses.push((Occur::Must, Box::new(BooleanQuery::new(term_queries))));
                    }
                } else {
                    let escaped = regex::escape(vol.trim());
                    if let Ok(rq) =
                        RegexQuery::from_pattern(&format!(".*{escaped}.*"), self.f_volume)
                    {
                        clauses.push((Occur::Must, Box::new(rq)));
                    } else {
                        let term = Term::from_field_text(self.f_volume, vol);
                        clauses.push((
                            Occur::Must,
                            Box::new(TermQuery::new(term, IndexRecordOption::Basic)),
                        ));
                    }
                }
            }
            if let Some(ref cat) = f.category {
                let cat_term = Term::from_field_text(self.f_category, cat.trim());
                if searcher.doc_freq(&cat_term).unwrap_or(0) > 0 {
                    clauses.push((
                        Occur::Must,
                        Box::new(TermQuery::new(cat_term, IndexRecordOption::Basic)),
                    ));
                } else {
                    let escaped = regex::escape(cat.trim());
                    if let Ok(rq) =
                        RegexQuery::from_pattern(&format!(".*{escaped}.*"), self.f_category)
                    {
                        clauses.push((Occur::Must, Box::new(rq)));
                    } else {
                        clauses.push((
                            Occur::Must,
                            Box::new(TermQuery::new(cat_term, IndexRecordOption::Basic)),
                        ));
                    }
                }
            }
        }

        let final_query = BooleanQuery::new(clauses);
        let top_docs = searcher
            .search(&final_query, &TopDocs::with_limit(top_k))
            .map_err(|e| VectorError::Other(e.to_string()))?;

        let mut results = Vec::with_capacity(top_docs.len());
        for (rank, (score, doc_address)) in top_docs.into_iter().enumerate() {
            let retrieved_doc: TantivyDocument = searcher
                .doc(doc_address)
                .map_err(|e| VectorError::Other(e.to_string()))?;

            let chunk_id = retrieved_doc
                .get_first(self.f_chunk_id)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();

            let json_str = retrieved_doc
                .get_first(self.f_json_chunk)
                .and_then(|v| v.as_str())
                .unwrap_or_default();

            if let Ok(chunk) = serde_json::from_str::<DocumentChunk>(json_str) {
                // Check remaining predicate filters (tags, date range, etc.)
                if let Some(f) = filter
                    && !f.matches(&chunk)
                {
                    continue;
                }

                results.push(FullTextSearchResult {
                    chunk_id,
                    score,
                    rank: rank + 1,
                    chunk,
                });
            }
        }

        debug!("BM25 FullText search retrieved {} results", results.len());
        Ok(results)
    }
}

fn generate_volume_candidates(volume: &str) -> Vec<String> {
    let trimmed = volume.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }
    let mut candidates = Vec::new();
    let mut seen = HashSet::new();

    let mut add = |s: String| {
        if !s.is_empty() && seen.insert(s.clone()) {
            candidates.push(s);
        }
    };

    add(trimmed.to_string());
    if !trimmed.starts_with("毛泽东") {
        add(format!("毛泽东{trimmed}"));
        add(format!("毛泽东选集{trimmed}"));
        add(format!("毛泽东文集{trimmed}"));
        add(format!("选集{trimmed}"));
        add(format!("文集{trimmed}"));
    }
    if let Some(stripped) = trimmed.strip_prefix("毛泽东") {
        add(stripped.to_string());
        if let Some(s2) = stripped
            .strip_prefix("选集")
            .or_else(|| stripped.strip_prefix("文集"))
        {
            add(s2.to_string());
        }
    } else if let Some(s2) = trimmed
        .strip_prefix("选集")
        .or_else(|| trimmed.strip_prefix("文集"))
    {
        add(s2.to_string());
    }

    candidates
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::HistoricalPeriod;

    fn dummy_chunk(id: &str, title: &str, period: HistoricalPeriod, text: &str) -> DocumentChunk {
        DocumentChunk {
            chunk_id: id.to_string(),
            doc_id: format!("doc_{}", id),
            doc_title: title.to_string(),
            author: "毛泽东".to_string(),
            period,
            date: "1938-05".to_string(),
            volume: "第二卷".to_string(),
            category: "军事".to_string(),
            tags: vec!["战略".to_string()],
            chunk_index: 0,
            total_chunks: 1,
            char_count: text.chars().count(),
            raw_text: text.to_string(),
            contextualized_text: format!("【文献】《{}》\n【正文】{}", title, text),
            section_path: vec!["战略总论".to_string()],
        }
    }

    #[test]
    fn test_tantivy_bm25_search() {
        let index = FullTextIndex::new_in_ram().unwrap();

        let c1 = dummy_chunk(
            "c1",
            "论持久战",
            HistoricalPeriod::WarOfResistance,
            "中日战争是持久战，战略防御、战略相持、战略反攻三个阶段。",
        );
        let c2 = dummy_chunk(
            "c2",
            "矛盾论",
            HistoricalPeriod::AgrarianRevolutionaryWar,
            "事物的矛盾法则，即对立统一的法则，主要矛盾与矛盾的主要方面。",
        );

        index.insert_batch(&[c1, c2]).unwrap();

        let results = index.search("持久战 三个阶段", 5, None).unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].chunk_id, "c1");

        let phil_results = index.search("主要矛盾 对立统一", 5, None).unwrap();
        assert!(!phil_results.is_empty());
        assert_eq!(phil_results[0].chunk_id, "c2");
    }

    #[test]
    fn test_bm25_subword_query_hits_compound_domain_term() {
        let index = FullTextIndex::new_in_ram().unwrap();
        let chunk = dummy_chunk(
            "c_compound",
            "星星之火，可以燎原",
            HistoricalPeriod::AgrarianRevolutionaryWar,
            "中国走农村包围城市道路",
        );
        index.insert_batch(&[chunk]).unwrap();

        let results = index.search("农村", 5, None).unwrap();
        assert!(
            !results.is_empty(),
            "query 农村 must recall a chunk whose only match is the compound 农村包围城市"
        );
        assert_eq!(results[0].chunk_id, "c_compound");
    }
}
