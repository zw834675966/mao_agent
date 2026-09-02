use mao_agent::corpus::chunker::{ChineseSemanticChunker, ChunkerConfig};
use mao_agent::model::{Document, DocumentMetadata, HistoricalPeriod};

#[test]
fn test_hierarchical_chinese_chunking() {
    let doc = Document {
        id: "doc_method".to_string(),
        metadata: DocumentMetadata {
            title: "关心群众生活，注意工作方法".to_string(),
            author: "毛泽东".to_string(),
            date: "1934-01-27".to_string(),
            period: "土地革命战争时期".to_string(),
            volume: "毛泽东选集第一卷".to_string(),
            category: "群众路线".to_string(),
            ..Default::default()
        },
        period_enum: HistoricalPeriod::AgrarianRevolutionaryWar,
        headnote: None,
        content: r#"# 问题的中心是关心群众的实际生活

我们现在的中心任务是动员广大群众参加革命战争。
要得到群众的拥护，就必须关心群众的痛痒，就必须同群众在一起，关心群众的生活。

# 工作方法是过河的桥或船

我们不但要提出任务，而且要解决完成任务的方法问题。
我们的任务是过河，但是没有桥或没有船就不能过。不解决桥或船的问题，过河就是一句空话。不解决方法问题，下车伊始，哇啦哇啦，就解决不了任何问题。
"#
        .to_string(),
        footnotes: vec![],
        file_path: None,
    };

    let config = ChunkerConfig {
        max_chars: 300,
        min_chars: 50,
        overlap_chars: 20,
        inject_context_header: true,
    };
    let chunker = ChineseSemanticChunker::new(config);
    let chunks = chunker.chunk_document(&doc);

    assert_eq!(chunks.len(), 2);

    let chunk0 = &chunks[0];
    assert!(
        chunk0
            .contextualized_text
            .contains("【文献】《关心群众生活，注意工作方法》")
    );
    assert!(chunk0.contextualized_text.contains("1934-01-27"));
    assert!(chunk0.contextualized_text.contains("土地革命战争时期"));
    assert!(
        chunk0
            .contextualized_text
            .contains("问题的中心是关心群众的实际生活")
    );
    assert!(chunk0.raw_text.contains("动员广大群众参加革命战争"));

    let chunk1 = &chunks[1];
    assert!(
        chunk1
            .contextualized_text
            .contains("工作方法是过河的桥或船")
    );
    assert!(chunk1.raw_text.contains("过河就是一句空话"));
}
