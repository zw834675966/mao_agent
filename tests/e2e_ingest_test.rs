use mao_agent::model::HistoricalPeriod;
use mao_agent::vector::embedder::{DeterministicEmbedder, create_embedder_arc};
use mao_agent::vector::store::VectorStore;
use tempfile::tempdir;

#[tokio::test]
async fn test_end_to_end_ingest_and_retrieval() {
    let tmp = tempdir().unwrap();
    let corpus_dir = tmp.path().join("corpus");
    std::fs::create_dir_all(&corpus_dir).unwrap();

    let doc1_content = r#"---
title: "论持久战"
author: "毛泽东"
date: "1938-05-26"
period: "抗日战争时期"
volume: "第二卷"
category: "军事战略"
---

# 为什么是持久战

中日战争是持久战，最后的胜利是中国的。
日本国小物匮，经不起长期战争；中国地大物博人多，能够支持长期战争。
"#;

    let doc2_content = r#"---
title: "矛盾论"
author: "毛泽东"
date: "1937-08"
period: "土地革命战争时期"
volume: "第一卷"
category: "哲学"
---

# 矛盾的对立统一

事物的矛盾法则，即对立统一的法则，是唯物辩证法的最根本的法则。
"#;

    std::fs::write(corpus_dir.join("doc1.md"), doc1_content).unwrap();
    std::fs::write(corpus_dir.join("doc2.md"), doc2_content).unwrap();

    let embedder = create_embedder_arc(DeterministicEmbedder::new(256));
    let store = VectorStore::new(embedder, None);

    let indexed = store.index_corpus_dir(&corpus_dir, 10).await.unwrap();
    assert_eq!(indexed, 2);

    let index_file = tmp.path().join("store.bin");
    store.save_to_file(&index_file).await.unwrap();

    // Reload from file
    let embedder2 = create_embedder_arc(DeterministicEmbedder::new(256));
    let loaded_store = VectorStore::load_from_file(&index_file, embedder2).unwrap();

    // Search query
    let results = loaded_store
        .search("长期战争与日本国小", 2, None)
        .await
        .unwrap();
    assert!(!results.is_empty());
    assert_eq!(results[0].chunk.doc_title, "论持久战");
    assert_eq!(results[0].chunk.period, HistoricalPeriod::WarOfResistance);
}
