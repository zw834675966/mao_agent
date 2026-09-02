use mao_agent::model::{
    Document, DocumentChunk, DocumentMetadata, HistoricalPeriod, VectorEntry, VectorFilter,
};
use mao_agent::vector::embedder::{DeterministicEmbedder, create_embedder_arc};
use mao_agent::vector::index::VectorIndex;
use mao_agent::vector::store::VectorStore;
use tempfile::tempdir;

fn make_test_chunk(
    id: &str,
    title: &str,
    period: HistoricalPeriod,
    vol: &str,
    cat: &str,
    text: &str,
) -> DocumentChunk {
    DocumentChunk {
        chunk_id: id.to_string(),
        doc_id: format!("doc_{}", id),
        doc_title: title.to_string(),
        author: "毛泽东".to_string(),
        period,
        date: "1938-05-26".to_string(),
        volume: vol.to_string(),
        category: cat.to_string(),
        tags: vec!["战略".to_string(), "军事".to_string()],
        chunk_index: 0,
        total_chunks: 1,
        char_count: text.chars().count(),
        raw_text: text.to_string(),
        contextualized_text: format!("【文献】《{}》\n【正文】{}", title, text),
        section_path: vec!["核心论断".to_string()],
    }
}

#[tokio::test]
async fn test_vector_index_basic_operations() {
    let mut index = VectorIndex::new(4);
    assert_eq!(index.len(), 0);
    assert!(index.is_empty());

    let entry = VectorEntry {
        id: "c1".to_string(),
        vector: vec![1.0, 0.0, 0.0, 0.0],
        chunk: make_test_chunk(
            "c1",
            "论持久战",
            HistoricalPeriod::WarOfResistance,
            "第二卷",
            "军事",
            "持久战论述",
        ),
    };

    index.insert(entry).unwrap();
    assert_eq!(index.len(), 1);
    assert!(!index.is_empty());

    let found = index.get("c1");
    assert!(found.is_some());
    assert_eq!(found.unwrap().chunk.doc_title, "论持久战");

    let deleted = index.delete("c1");
    assert!(deleted);
    assert_eq!(index.len(), 0);
}

#[tokio::test]
async fn test_vector_index_dimension_mismatch() {
    let mut index = VectorIndex::new(4);
    let invalid_entry = VectorEntry {
        id: "bad".to_string(),
        vector: vec![1.0, 2.0], // Expected 4, got 2
        chunk: make_test_chunk("bad", "测试", HistoricalPeriod::Unknown, "", "", "短向量"),
    };

    let result = index.insert(invalid_entry);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_vector_store_multi_attribute_filtering() {
    let store = VectorStore::new_deterministic(64);

    let doc_resistance = Document {
        id: "doc_res".to_string(),
        metadata: DocumentMetadata {
            title: "论持久战".to_string(),
            date: "1938-05-26".to_string(),
            period: "抗日战争时期".to_string(),
            volume: "毛泽东选集第二卷".to_string(),
            category: "军事战略".to_string(),
            tags: vec!["持久战".to_string(), "抗战".to_string()],
            ..Default::default()
        },
        period_enum: HistoricalPeriod::WarOfResistance,
        headnote: None,
        content: "中日战争是持久战，最后的胜利是中国的。".to_string(),
        footnotes: vec![],
        file_path: None,
    };

    let doc_philosophy = Document {
        id: "doc_phil".to_string(),
        metadata: DocumentMetadata {
            title: "矛盾论".to_string(),
            date: "1937-08".to_string(),
            period: "土地革命战争时期".to_string(),
            volume: "毛泽东选集第一卷".to_string(),
            category: "哲学".to_string(),
            tags: vec!["辩证法".to_string()],
            ..Default::default()
        },
        period_enum: HistoricalPeriod::AgrarianRevolutionaryWar,
        headnote: None,
        content: "事物的矛盾法则，即对立统一的法则，是唯物辩证法的最根本的法则。".to_string(),
        footnotes: vec![],
        file_path: None,
    };

    store
        .index_documents(&[doc_resistance, doc_philosophy], 10)
        .await
        .unwrap();
    assert_eq!(store.len().await, 2);

    // 1. Period filter
    let filter_period = VectorFilter::new().with_period(HistoricalPeriod::WarOfResistance);
    let res1 = store
        .search("战争与辩证法", 10, Some(&filter_period))
        .await
        .unwrap();
    assert_eq!(res1.len(), 1);
    assert_eq!(res1[0].chunk.doc_title, "论持久战");

    // 2. Volume filter
    let filter_vol = VectorFilter::new().with_volume("第一卷");
    let res2 = store
        .search("事物与矛盾", 10, Some(&filter_vol))
        .await
        .unwrap();
    assert_eq!(res2.len(), 1);
    assert_eq!(res2[0].chunk.doc_title, "矛盾论");

    // 3. Category filter
    let filter_cat = VectorFilter::new().with_category("哲学");
    let res3 = store
        .search("中国抗日战略", 10, Some(&filter_cat))
        .await
        .unwrap();
    assert_eq!(res3.len(), 1);
    assert_eq!(res3[0].chunk.doc_title, "矛盾论");

    // 4. Date range filter
    let filter_date = VectorFilter::new().with_date_range("1938-01-01", "1938-12-31");
    let res4 = store
        .search("中日战争与哲学", 10, Some(&filter_date))
        .await
        .unwrap();
    assert_eq!(res4.len(), 1);
    assert_eq!(res4[0].chunk.doc_title, "论持久战");
}

#[tokio::test]
async fn test_persistence_atomic_save_and_reload() {
    let tmp = tempdir().unwrap();
    let index_file = tmp.path().join("nested").join("test_store.bin");

    let store = VectorStore::new_deterministic(128);

    let doc = Document {
        id: "doc_spark".to_string(),
        metadata: DocumentMetadata {
            title: "星星之火，可以燎原".to_string(),
            date: "1930-01-05".to_string(),
            period: "土地革命战争时期".to_string(),
            volume: "毛泽东选集第一卷".to_string(),
            category: "政论".to_string(),
            ..Default::default()
        },
        period_enum: HistoricalPeriod::AgrarianRevolutionaryWar,
        headnote: None,
        content: "它是站在海岸遥望海中已经看得见桅杆尖头了的一只航船，它是立于高山之巅远看东方已见光芒四射喷薄欲出的一轮朝日。".to_string(),
        footnotes: vec![],
        file_path: None,
    };

    store.index_document(&doc).await.unwrap();
    assert_eq!(store.len().await, 1);

    // Save, then overwrite the same path (Windows rename cannot replace).
    store.save_to_file(&index_file).await.unwrap();
    assert!(index_file.exists());
    store.save_to_file(&index_file).await.unwrap();

    // Load
    let embedder = create_embedder_arc(DeterministicEmbedder::new(128));
    let reloaded = VectorStore::load_from_file(&index_file, embedder).unwrap();
    assert_eq!(reloaded.len().await, 1);

    let search_res = reloaded.search("航船与红日", 1, None).await.unwrap();
    assert_eq!(search_res.len(), 1);
    assert_eq!(search_res[0].chunk.doc_title, "星星之火，可以燎原");
}

#[tokio::test]
async fn test_load_rejects_dimension_mismatch() {
    let tmp = tempdir().unwrap();
    let index_file = tmp.path().join("dim_mismatch.bin");

    let store = VectorStore::new_deterministic(128);
    store.save_to_file(&index_file).await.unwrap();

    let embedder = create_embedder_arc(DeterministicEmbedder::new(64));
    let err = VectorStore::load_from_file(&index_file, embedder).unwrap_err();
    match err {
        VectorError::DimensionMismatch { .. } | VectorError::IdentityMismatch { .. } => {}
        other => panic!("expected DimensionMismatch or IdentityMismatch, got {other:?}"),
    }
}

#[tokio::test]
async fn test_load_rejects_identity_mismatch_same_dim_different_model() {
    let tmp = tempdir().unwrap();
    let index_file = tmp.path().join("identity_mismatch.bin");

    let store = VectorStore::new_deterministic(128);
    store.save_to_file(&index_file).await.unwrap();

    let embedder = create_embedder_arc(OtherModelEmbedder { dimension: 128 });
    let err = VectorStore::load_from_file(&index_file, embedder).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("ingest"), "{msg}");
    match err {
        VectorError::IdentityMismatch {
            snapshot_model,
            snapshot_dimension,
            source_model,
            source_dimension,
        } => {
            assert_eq!(snapshot_model, "deterministic-hash-128");
            assert_eq!(snapshot_dimension, 128);
            assert_eq!(source_model, "other-model");
            assert_eq!(source_dimension, 128);
        }
        other => panic!("expected IdentityMismatch, got {other:?}"),
    }
}

#[tokio::test]
async fn test_load_legacy_unheadered_bincode_uses_dim_only_check() {
    let tmp = tempdir().unwrap();
    let index_file = tmp.path().join("legacy_store.bin");

    let mut index = VectorIndex::new(64);
    index
        .insert(VectorEntry {
            id: "c1".to_string(),
            vector: vec![1.0; 64],
            chunk: make_test_chunk(
                "c1",
                "实践论",
                HistoricalPeriod::AgrarianRevolutionaryWar,
                "第一卷",
                "哲学",
                "实践的观点是辩证唯物论的认识论之第一的和基本的观点",
            ),
        })
        .unwrap();
    let raw = bincode::serialize(&index).unwrap();
    assert!(
        !raw.starts_with(b"MAOVS01\0"),
        "legacy fixture must be unheadered bincode"
    );
    std::fs::write(&index_file, raw).unwrap();

    let embedder = create_embedder_arc(DeterministicEmbedder::new(64));
    let loaded = VectorStore::load_from_file(&index_file, embedder).unwrap();
    assert_eq!(loaded.len().await, 1);
}
