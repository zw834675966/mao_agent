use std::sync::Arc;

use async_trait::async_trait;
use axum::{Json, extract::State, http::HeaderMap};
use mao_agent::index::{FullTextIndex, HybridSearchCoordinator, HybridSearchResult};
use mao_agent::model::{Document, DocumentMetadata, HistoricalPeriod};
use mao_agent::rerank::Reranker;
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
async fn test_health_returns_503_when_index_empty() {
    let store = Arc::new(VectorStore::new_deterministic(128));
    let state = AppState::new(
        store,
        None,
        HybridSearchCoordinator::default(),
        None,
        "http://127.0.0.1:9".to_string(),
        None,
        "test-model".to_string(),
    );
    let (status, Json(body)) = health::handle_health(State(state)).await;
    assert_eq!(status, axum::http::StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body.status, "unavailable");
    assert!(!body.index_loaded);
    assert_eq!(body.total_vectors, 0);
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
        "event: reranked",
        "event: delta",
        "event: citation",
        "event: done",
    ] {
        assert!(text.contains(evt), "stream missing {evt}:\n{text}");
    }
    // event order: retrieved < reranked < delta < citation < done
    let pos = |m: &str| text.find(m).unwrap();
    assert!(pos("event: retrieved") < pos("event: reranked"));
    assert!(pos("event: reranked") < pos("event: delta"));
    assert!(pos("event: delta") < pos("event: citation"));
    assert!(pos("event: citation") < pos("event: done"));
    // no reranker in test_state → applied=false; scores omitted
    assert!(
        text.contains("\"applied\":false") || text.contains("\"applied\": false"),
        "reranked should report applied=false without AppState.reranker:\n{text}"
    );
    assert!(
        !text.contains("\"scores\""),
        "scores must be absent/None when rerank not applied:\n{text}"
    );
}

/// Mock reranker that reverses candidate order and stamps scores (no network).
struct ApiMockReranker;

#[async_trait]
impl Reranker for ApiMockReranker {
    fn model_name(&self) -> &str {
        "api-mock"
    }

    async fn rerank(
        &self,
        _query: &str,
        candidates: &[HybridSearchResult],
        top_k: usize,
    ) -> mao_agent::error::Result<Vec<HybridSearchResult>> {
        let mut out = candidates.to_vec();
        out.reverse();
        for (i, item) in out.iter_mut().enumerate() {
            item.rerank_score = Some(1.0 - i as f32 * 0.1);
            item.rank = i + 1;
        }
        out.truncate(top_k);
        Ok(out)
    }
}

async fn test_state_with_reranker() -> AppState {
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
        Some(Arc::new(ApiMockReranker)),
        "http://127.0.0.1:9".to_string(),
        None,
        "test-model".to_string(),
    )
}

#[tokio::test]
async fn test_search_mock_reranker_sets_scores() {
    let state = test_state_with_reranker().await;
    let (_s, Json(body)) =
        search::handle_search(State(state), Json(search_req("持久战的三个阶段", "hybrid")))
            .await
            .unwrap();
    assert!(!body.results.is_empty());
    assert!(
        body.results.iter().any(|h| h.rerank_score.is_some()),
        "MockReranker should populate rerank_score"
    );
}

#[tokio::test]
async fn test_search_no_rerank_skips_scores() {
    let state = test_state_with_reranker().await;
    let mut req = search_req("持久战的三个阶段", "hybrid");
    req.no_rerank = Some(true);
    let (_s, Json(body)) = search::handle_search(State(state), Json(req))
        .await
        .unwrap();
    assert!(!body.results.is_empty());
    assert!(
        body.results.iter().all(|h| h.rerank_score.is_none()),
        "no_rerank=true must leave rerank_score unset"
    );
}

#[tokio::test]
async fn test_search_without_reranker_degrades() {
    let state = test_state().await;
    assert!(state.reranker.is_none());
    let (_s, Json(body)) =
        search::handle_search(State(state), Json(search_req("持久战", "hybrid")))
            .await
            .unwrap();
    assert!(!body.results.is_empty());
    assert!(body.results.iter().all(|h| h.rerank_score.is_none()));
}

#[tokio::test]
async fn test_ask_stream_reports_rerank_applied() {
    use axum::response::IntoResponse;
    let state = test_state_with_reranker().await;
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
    let bytes = axum::body::to_bytes(resp.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let text = String::from_utf8(bytes.to_vec()).unwrap();
    assert!(text.contains("event: reranked"));
    assert!(
        text.contains("\"applied\":true") || text.contains("\"applied\": true"),
        "with MockReranker, applied should be true from stamped scores:\n{text}"
    );
    // Parse the reranked event data line and assert non-empty scores array
    let mut scores_ok = false;
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        if line.trim() == "event: reranked"
            && let Some(data_line) = lines.next()
        {
            let payload = data_line.strip_prefix("data:").unwrap_or(data_line).trim();
            let v: serde_json::Value = serde_json::from_str(payload)
                .unwrap_or_else(|e| panic!("reranked JSON parse failed: {e}; payload={payload}"));
            let scores = v.get("scores").and_then(|s| s.as_array());
            assert!(
                scores.is_some() && !scores.unwrap().is_empty(),
                "scores must be a non-empty array when MockReranker applied:\n{payload}"
            );
            scores_ok = true;
            break;
        }
    }
    assert!(
        scores_ok,
        "did not find parseable reranked event with scores:\n{text}"
    );
}

#[tokio::test]
async fn test_handler_concurrency_smoke() {
    let state = test_state().await;
    let mut handles = Vec::new();
    for i in 0..50 {
        let s = state.clone();
        handles.push(tokio::spawn(async move {
            if i % 2 == 0 {
                search::handle_search(State(s), Json(search_req("持久战", "hybrid")))
                    .await
                    .map(|_| ())
                    .map_err(|e| e.status)
            } else {
                let req = AskRequest {
                    question: "抗日战争为什么是持久战？".to_string(),
                    top_k: Some(1),
                    period: None,
                    volume: None,
                    base_url: None,
                    model: None,
                    api_key: None,
                };
                ask::handle_ask(State(s), HeaderMap::new(), Json(req))
                    .await
                    .map(|_| ())
                    .map_err(|e| e.status)
            }
        }));
    }
    for h in handles {
        let r = h.await.expect("join");
        assert!(r.is_ok(), "handler returned err: {r:?}");
    }
}
