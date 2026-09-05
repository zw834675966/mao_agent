use mao_agent::agent::engine::DialecticalAgent;
use mao_agent::index::fulltext::FullTextIndex;
use mao_agent::index::hybrid::HybridSearchCoordinator;
use mao_agent::model::{Document, DocumentMetadata, HistoricalPeriod, VectorFilter};
use mao_agent::vector::store::VectorStore;
use std::sync::Arc;

#[tokio::test]
async fn test_fulltext_and_hybrid_pipeline() {
    let ft_index = FullTextIndex::new_in_ram().unwrap();

    let doc1 = Document {
        id: "doc_1".to_string(),
        metadata: DocumentMetadata {
            title: "论持久战".to_string(),
            author: "毛泽东".to_string(),
            date: "1938-05-26".to_string(),
            period: "抗日战争时期".to_string(),
            volume: "毛泽东选集第二卷".to_string(),
            category: "军事战略".to_string(),
            ..Default::default()
        },
        period_enum: HistoricalPeriod::WarOfResistance,
        headnote: None,
        content: "中日战争是持久战，战略相持阶段是关键。统一战线与武装斗争是克敌制胜的法宝。"
            .to_string(),
        footnotes: vec![],
        file_path: None,
    };

    let chunker = mao_agent::corpus::ChineseSemanticChunker::new(Default::default());
    let chunks = chunker.chunk_document(&doc1);

    ft_index.insert_batch(&chunks).unwrap();

    // 1. BM25 Search
    let bm25_res = ft_index.search("统一战线 武装斗争", 5, None).unwrap();
    assert!(!bm25_res.is_empty());
    assert_eq!(bm25_res[0].chunk.doc_title, "论持久战");

    // 2. Vector Search
    let store = Arc::new(VectorStore::new_deterministic(128));
    store.index_document(&doc1).await.unwrap();
    let vec_res = store.search("中日持久战与战略相持", 5, None).await.unwrap();
    assert!(!vec_res.is_empty());

    // 3. RRF Hybrid Fusion
    let hybrid = HybridSearchCoordinator::default();
    let fused = hybrid.fuse(vec_res, bm25_res, 5);
    assert!(!fused.is_empty());
    assert_eq!(fused[0].chunk.doc_title, "论持久战");
    assert!(fused[0].rrf_score > 0.0);
}

#[tokio::test]
async fn test_dialectical_agent_e2e() {
    let store = Arc::new(VectorStore::new_deterministic(128));

    let doc = Document {
        id: "doc_contradiction".to_string(),
        metadata: DocumentMetadata {
            title: "矛盾论".to_string(),
            author: "毛泽东".to_string(),
            date: "1937-08".to_string(),
            period: "土地革命战争时期".to_string(),
            volume: "毛泽东选集第一卷".to_string(),
            category: "马克思主义哲学".to_string(),
            ..Default::default()
        },
        period_enum: HistoricalPeriod::AgrarianRevolutionaryWar,
        headnote: None,
        content:
            "研究任何过程，必须用全力找出它的主要矛盾。捉住了这个主要矛盾，一切问题就迎刃而解了。"
                .to_string(),
        footnotes: vec![],
        file_path: None,
    };

    store.index_document(&doc).await.unwrap();

    let agent = DialecticalAgent::new(store, None, None, None, None, None);
    let answer = agent
        .ask("如何抓住复杂事物中的主要矛盾？", 3, None)
        .await
        .unwrap();

    assert!(answer.content.contains("调查研究"));
    assert!(answer.content.contains("主要矛盾分析"));
    assert!(answer.content.contains("矛盾论"));
    assert!(!answer.retrieved_chunks.is_empty());

    // Verify citation report
    for report in &answer.citation_reports {
        assert!(report.is_verified);
    }
}

#[tokio::test]
async fn test_dialectical_agent_with_hybrid_search() {
    let store = Arc::new(VectorStore::new_deterministic(128));
    let ft_index = Arc::new(FullTextIndex::new_in_ram().unwrap());

    let doc = Document {
        id: "doc_chijiuzhan".to_string(),
        metadata: DocumentMetadata {
            title: "论持久战".to_string(),
            author: "毛泽东".to_string(),
            date: "1938-05-26".to_string(),
            period: "抗日战争时期".to_string(),
            volume: "毛泽东选集第二卷".to_string(),
            category: "军事战略".to_string(),
            ..Default::default()
        },
        period_enum: HistoricalPeriod::WarOfResistance,
        headnote: None,
        content: "中日战争是持久战，战略相持阶段是关键。兵民是胜利之本。战争的伟力之最深厚的根源，存在于民众之中。".to_string(),
        footnotes: vec![],
        file_path: None,
    };

    let chunker = mao_agent::corpus::ChineseSemanticChunker::new(Default::default());
    let chunks = chunker.chunk_document(&doc);

    store.index_document(&doc).await.unwrap();
    ft_index.insert_batch(&chunks).unwrap();

    let agent = DialecticalAgent::new(store, Some(ft_index), None, None, None, None);
    let answer = agent
        .ask("兵民是胜利之本在持久战中的作用？", 3, None)
        .await
        .unwrap();

    assert!(answer.content.contains("调查研究"));
    assert!(answer.content.contains("主要矛盾分析"));
    assert!(answer.content.contains("论持久战"));
    assert!(!answer.retrieved_chunks.is_empty());

    for report in &answer.citation_reports {
        assert!(report.is_verified);
    }
}

#[test]
fn test_bm25_volume_substring_and_category_filtering() {
    let ft_index = FullTextIndex::new_in_ram().unwrap();

    let doc = Document {
        id: "doc_chijiuzhan_filter".to_string(),
        metadata: DocumentMetadata {
            title: "论持久战".to_string(),
            author: "毛泽东".to_string(),
            date: "1938-05-26".to_string(),
            period: "抗日战争时期".to_string(),
            volume: "毛泽东选集第二卷".to_string(),
            category: "军事战略".to_string(),
            ..Default::default()
        },
        period_enum: HistoricalPeriod::WarOfResistance,
        headnote: None,
        content: "中日战争是持久战，最后的胜利是中国的。".to_string(),
        footnotes: vec![],
        file_path: None,
    };

    let chunker = mao_agent::corpus::ChineseSemanticChunker::new(Default::default());
    let chunks = chunker.chunk_document(&doc);
    ft_index.insert_batch(&chunks).unwrap();

    // 1. Filter by substring volume: "第二卷" matches "毛泽东选集第二卷"
    let filter_vol = VectorFilter::new().with_volume("第二卷");
    let res_vol = ft_index.search("持久战", 5, Some(&filter_vol)).unwrap();
    assert_eq!(res_vol.len(), 1, "BM25 substring volume match failed");
    assert_eq!(res_vol[0].chunk.doc_title, "论持久战");

    // 2. Filter by category: "军事" matches "军事战略"
    let filter_cat = VectorFilter::new().with_category("军事");
    let res_cat = ft_index.search("持久战", 5, Some(&filter_cat)).unwrap();
    assert_eq!(res_cat.len(), 1, "BM25 substring category match failed");
    assert_eq!(res_cat[0].chunk.doc_title, "论持久战");

    // 3. Mismatched filter returns 0 results
    let filter_mismatch = VectorFilter::new().with_volume("第三卷");
    let res_none = ft_index
        .search("持久战", 5, Some(&filter_mismatch))
        .unwrap();
    assert!(
        res_none.is_empty(),
        "Mismatched volume must return 0 results"
    );

    // 4. Regex fallback path: arbitrary substring "二卷" (not in standard prefix candidate terms)
    // generate_volume_candidates("二卷") yields ["二卷", "毛泽东二卷", "毛泽东选集二卷", ...] - none of which is "毛泽东选集第二卷"
    // So doc_freq for all candidates is 0, triggering the RegexQuery fallback `.*二卷.*`, which succeeds!
    let filter_vol_regex_fallback = VectorFilter::new().with_volume("二卷");
    let res_regex = ft_index
        .search("持久战", 5, Some(&filter_vol_regex_fallback))
        .unwrap();
    assert_eq!(
        res_regex.len(),
        1,
        "Regex fallback for '二卷' must match '毛泽东选集第二卷'"
    );
    assert_eq!(res_regex[0].chunk.doc_title, "论持久战");

    // 5. Category regex fallback path: arbitrary substring "事战" (not exact term "军事战略")
    let filter_cat_regex_fallback = VectorFilter::new().with_category("事战");
    let res_cat_regex = ft_index
        .search("持久战", 5, Some(&filter_cat_regex_fallback))
        .unwrap();
    assert_eq!(
        res_cat_regex.len(),
        1,
        "Regex fallback for '事战' must match '军事战略'"
    );
    assert_eq!(res_cat_regex[0].chunk.doc_title, "论持久战");

    // 6. Category mismatch returns 0 results
    let filter_cat_mismatch = VectorFilter::new().with_category("经济建设");
    let res_cat_none = ft_index
        .search("持久战", 5, Some(&filter_cat_mismatch))
        .unwrap();
    assert!(
        res_cat_none.is_empty(),
        "Mismatched category must return 0 results"
    );
}
