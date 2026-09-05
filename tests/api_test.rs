use std::sync::Arc;

use axum::{Json, extract::State, http::HeaderMap};
use mao_agent::index::{FullTextIndex, HybridSearchCoordinator};
use mao_agent::model::{Document, DocumentMetadata, HistoricalPeriod};
use mao_agent::server::dto::{AskRequest, SearchRequest, VerifyRequest};
use mao_agent::server::handlers::{ask, health, search, verify};
use mao_agent::server::state::AppState;
use mao_agent::vector::VectorStore;

fn doc_lun_chi_jiu_zhan() -> Document {
    Document {
        id: "doc_lcjz".to_string(),
        metadata: DocumentMetadata {
            title: "论持久战".to_string(),
            author: "毛泽东".to_string(),
            date: "1938-05-26".to_string(),
            period: "抗日战争时期".to_string(),
            volume: "毛泽东选集第二卷".to_string(),
            category: "军事".to_string(),
            tags: vec!["持久战".to_string()],
            ..Default::default()
        },
        period_enum: HistoricalPeriod::WarOfResistance,
        headnote: None,
        content:
            "中日战争是持久战，最后的胜利是中国的。战争的三个阶段：战略防御、战略相持、战略反攻。"
                .to_string(),
        footnotes: vec![],
        file_path: None,
    }
}

fn doc_mao_dun_lun() -> Document {
    Document {
        id: "doc_mdl".to_string(),
        metadata: DocumentMetadata {
            title: "矛盾论".to_string(),
            author: "毛泽东".to_string(),
            date: "1937-08".to_string(),
            period: "土地革命战争时期".to_string(),
            volume: "毛泽东选集第一卷".to_string(),
            category: "哲学".to_string(),
            tags: vec!["唯物辩证法".to_string()],
            ..Default::default()
        },
        period_enum: HistoricalPeriod::AgrarianRevolutionaryWar,
        headnote: None,
        content:
            "事物的矛盾法则，即对立统一的法则，是唯物辩证法的最根本的法则。主要矛盾和主要矛盾方面。"
                .to_string(),
        footnotes: vec![],
        file_path: None,
    }
}

async fn test_state() -> AppState {
    let docs = [doc_lun_chi_jiu_zhan(), doc_mao_dun_lun()];
    let store = Arc::new(VectorStore::new_deterministic(128));
    for d in &docs {
        store.index_document(d).await.unwrap();
    }
    let chunker = mao_agent::corpus::ChineseSemanticChunker::new(Default::default());
    let ft = FullTextIndex::new_in_ram().unwrap();
    for d in &docs {
        ft.insert_batch(&chunker.chunk_document(d)).unwrap();
    }
    AppState::new(
        store,
        Some(Arc::new(ft)),
        HybridSearchCoordinator::default(),
        None,
        "http://127.0.0.1:9".to_string(),
        None,
        "test-model".to_string(),
    )
}

fn search_req(query: &str, mode: &str) -> SearchRequest {
    SearchRequest {
        query: query.to_string(),
        top_k: 5,
        mode: mode.to_string(),
        period: None,
        volume: None,
        category: None,
        tags: None,
        start_date: None,
        end_date: None,
        doc_id: None,
        keyword: None,
        min_score: None,
        no_rerank: None,
    }
}

#[tokio::test]
async fn test_health_reports_loaded_index() {
    let state = test_state().await;
    let (status, Json(body)) = health::handle_health(State(state)).await;
    assert_eq!(status, axum::http::StatusCode::OK);
    assert_eq!(body.status, "ok");
    assert!(body.index_loaded);
    assert!(body.tantivy_loaded);
    assert_eq!(body.total_documents, 2);
    assert!(body.total_vectors >= 2);
}

#[tokio::test]
async fn test_search_hybrid_top_hit() {
    let state = test_state().await;
    let (_status, Json(body)) =
        search::handle_search(State(state), Json(search_req("持久战的三个阶段", "hybrid")))
            .await
            .unwrap();
    assert_eq!(body.mode, "hybrid");
    assert!(!body.results.is_empty());
    assert_eq!(body.results[0].chunk.doc_title, "论持久战");
    assert!(body.results[0].rrf_score.is_some());
}

#[tokio::test]
async fn test_search_vector_and_bm25_modes() {
    let state = test_state().await;
    let (_s, Json(v)) = search::handle_search(
        State(state.clone()),
        Json(search_req("矛盾与唯物辩证法", "vector")),
    )
    .await
    .unwrap();
    assert_eq!(v.mode, "vector");
    assert!(!v.results.is_empty());
    assert!(v.results[0].vector_score.is_some());

    let (_s, Json(b)) =
        search::handle_search(State(state), Json(search_req("持久战 三个阶段", "bm25")))
            .await
            .unwrap();
    assert_eq!(b.mode, "bm25");
    assert!(!b.results.is_empty());
    assert_eq!(b.results[0].chunk.doc_title, "论持久战");
    assert!(b.results[0].bm25_score.is_some());
}

#[tokio::test]
async fn test_search_rejects_bad_input() {
    let state = test_state().await;
    let err = search::handle_search(State(state.clone()), Json(search_req("", "hybrid")))
        .await
        .unwrap_err();
    assert_eq!(err.status, axum::http::StatusCode::BAD_REQUEST);

    let err = search::handle_search(State(state), Json(search_req("x", "nope")))
        .await
        .unwrap_err();
    assert_eq!(err.status, axum::http::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_search_period_filter() {
    let state = test_state().await;
    let mut req = search_req("战争", "hybrid");
    req.period = Some("抗日".to_string());
    let (_s, Json(body)) = search::handle_search(State(state), Json(req))
        .await
        .unwrap();
    assert!(!body.results.is_empty());
    for hit in &body.results {
        assert_eq!(hit.chunk.period, HistoricalPeriod::WarOfResistance);
    }
}

#[tokio::test]
async fn test_verify_real_and_hallucinated_quotes() {
    let state = test_state().await;
    let (_s, Json(found)) =
        search::handle_search(State(state.clone()), Json(search_req("持久战", "hybrid")))
            .await
            .unwrap();
    let chunk = found.results[0].chunk.clone();

    let real = VerifyRequest {
        quote: chunk.raw_text.chars().take(20).collect(),
        claimed_title: chunk.doc_title.clone(),
        context_chunks: vec![chunk.clone()],
        min_confidence: None,
    };
    let (_s, Json(ok)) = verify::handle_verify(State(state.clone()), Json(real))
        .await
        .unwrap();
    assert!(ok.report.is_verified);

    let fake = VerifyRequest {
        quote: "互联网技术是未来战争胜负的决定性力量".to_string(),
        claimed_title: chunk.doc_title.clone(),
        context_chunks: vec![chunk],
        min_confidence: None,
    };
    let (_s, Json(bad)) = verify::handle_verify(State(state), Json(fake))
        .await
        .unwrap();
    assert!(!bad.report.is_verified);
    assert!(bad.report.warning.is_some());
}

#[tokio::test]
async fn test_ask_offline_grounded() {
    let state = test_state().await;
    let req = AskRequest {
        question: "抗日战争为什么是持久战？".to_string(),
        top_k: Some(2),
        period: None,
        volume: None,
        base_url: None,
        model: None,
        api_key: None,
    };
    let (_s, Json(body)) = ask::handle_ask(State(state), HeaderMap::new(), Json(req))
        .await
        .unwrap();
    assert!(!body.retrieved_chunks.is_empty());
    assert!(body.content.contains("调查研究"));
    assert!(body.is_fully_grounded);
    assert!(!body.citation_reports.is_empty());

    // empty question -> 400
    let state2 = test_state().await;
    let bad = AskRequest {
        question: "   ".to_string(),
        top_k: None,
        period: None,
        volume: None,
        base_url: None,
        model: None,
        api_key: None,
    };
    let err = ask::handle_ask(State(state2), HeaderMap::new(), Json(bad))
        .await
        .unwrap_err();
    assert_eq!(err.status, axum::http::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_ask_stream_emits_event_sequence() {
    use axum::response::IntoResponse;
    let state = test_state().await;
    let req = AskRequest {
        question: "test".to_string(),
        top_k: Some(1),
        period: None,
        volume: None,
        base_url: None,
        model: None,
        api_key: None,
    };
    let sse = ask::handle_ask_stream(State(state), HeaderMap::new(), Json(req))
        .await
        .unwrap();
    let resp = sse.into_response();
    assert_eq!(
        resp.headers()
            .get(axum::http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or(""),
        "text/event-stream"
    );
    let bytes = axum::body::to_bytes(resp.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let text = String::from_utf8(bytes.to_vec()).unwrap();
    for evt in [
        "event: retrieved",
        "event: delta",
        "event: citation",
        "event: done",
    ] {
        assert!(text.contains(evt), "stream missing {evt}:\n{text}");
    }
    // event order: retrieved < delta < citation < done
    let pos = |m: &str| text.find(m).unwrap();
    assert!(pos("event: retrieved") < pos("event: delta"));
    assert!(pos("event: delta") < pos("event: citation"));
    assert!(pos("event: citation") < pos("event: done"));
}
