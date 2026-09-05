use std::sync::Arc;
use std::time::Instant;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
};
use futures::stream::Stream;

use crate::agent::DialecticalAgent;
use crate::model::VectorFilter;
use crate::server::dto::{
    AskRequest, AskResponse, SseCitationEvent, SseDeltaEvent, SseDoneEvent, SseRerankedEvent,
    SseRetrievedEvent,
};
use crate::server::error::{ApiError, ApiResult};
use crate::server::state::AppState;

fn build_filter(req: &AskRequest) -> Option<VectorFilter> {
    if req.period.is_none() && req.volume.is_none() {
        return None;
    }
    let mut f = VectorFilter::new();
    if let Some(ref p) = req.period {
        f.period = Some(crate::model::HistoricalPeriod::from_str_or_date(p));
    }
    if let Some(ref v) = req.volume {
        f.volume = Some(v.clone());
    }
    Some(f)
}

fn resolve_chat_overrides(
    state: &AppState,
    req: &AskRequest,
    header_api_key: Option<String>,
) -> (String, Option<String>, String) {
    let base_url = req
        .base_url
        .clone()
        .unwrap_or_else(|| state.chat_base_url.clone());
    let api_key = req
        .api_key
        .clone()
        .or(header_api_key)
        .or_else(|| state.chat_api_key.clone());
    let model = req
        .model
        .clone()
        .unwrap_or_else(|| state.chat_model.clone());
    (base_url, api_key, model)
}

// ── Blocking JSON ────────────────────────────────────────────────────────

async fn handle_ask_inner(
    state: AppState,
    req: AskRequest,
    header_api_key: Option<String>,
) -> ApiResult<(StatusCode, Json<AskResponse>)> {
    if req.question.trim().is_empty() {
        return Err(ApiError::bad_request("question must not be empty"));
    }
    let top_k = req.top_k.unwrap_or(3).clamp(1, 10);
    let filter = build_filter(&req);
    let (base_url, api_key, model) = resolve_chat_overrides(&state, &req, header_api_key);
    let start = Instant::now();

    let agent = DialecticalAgent::new(
        Arc::clone(&state.store),
        state.tantivy.clone(),
        Some(base_url),
        api_key,
        Some(model),
        state.reranker.clone(),
    );
    let answer = agent
        .ask(&req.question, top_k, filter.as_ref())
        .await
        .map_err(ApiError::from)?;

    let elapsed_ms = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);
    let resp = AskResponse {
        question: answer.question,
        content: answer.content,
        retrieved_chunks: answer.retrieved_chunks,
        citation_reports: answer.citation_reports,
        is_fully_grounded: answer.is_fully_grounded,
        elapsed_ms,
    };
    Ok((StatusCode::OK, Json(resp)))
}

// Blocking ask; API key precedence: body `api_key` > `Authorization: Bearer` header > server default.
pub async fn handle_ask(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<AskRequest>,
) -> ApiResult<(StatusCode, Json<AskResponse>)> {
    let header_api_key = extract_bearer(&headers);
    handle_ask_inner(state, req, header_api_key).await
}

fn extract_bearer(headers: &axum::http::HeaderMap) -> Option<String> {
    let v = headers
        .get(axum::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?;
    let token = v.strip_prefix("Bearer ")?;
    let t = token.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

// ── Streaming SSE ────────────────────────────────────────────────────────

pub async fn handle_ask_stream(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<AskRequest>,
) -> ApiResult<Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>>> {
    if req.question.trim().is_empty() {
        return Err(ApiError::bad_request("question must not be empty"));
    }
    let header_api_key = extract_bearer(&headers);
    let top_k = req.top_k.unwrap_or(3).clamp(1, 10);
    let filter = build_filter(&req);
    let (base_url, api_key, model) = resolve_chat_overrides(&state, &req, header_api_key);
    let question = req.question.clone();

    let stream = async_stream::stream! {
        let start = Instant::now();
        let agent = DialecticalAgent::new(
        Arc::clone(&state.store),
        state.tantivy.clone(),
        Some(base_url),
        api_key,
        Some(model),
        state.reranker.clone(),
    );

        // 1) Retrieve + generate (reuses DialecticalAgent::ask for now; future: true streaming LLM)
        let answer = match agent.ask(&question, top_k, filter.as_ref()).await {
            Ok(a) => a,
            Err(e) => {
                let err_json = serde_json::json!({"error": e.to_string()});
                if let Ok(ev) = Event::default().event("error").json_data(err_json) {
                    yield Ok(ev);
                }
                return;
            }
        };

        // event: retrieved
        if let Ok(data) = serde_json::to_string(&SseRetrievedEvent { chunks: answer.retrieved_chunks.clone() }) {
            yield Ok(Event::default().event("retrieved").data(data));
        }

        // event: reranked — always emit final evidence order; applied when AppState has a reranker
        let chunk_ids: Vec<String> = answer
            .retrieved_chunks
            .iter()
            .map(|c| c.chunk_id.clone())
            .collect();
        let applied = state.reranker.is_some();
        if let Ok(data) = serde_json::to_string(&SseRerankedEvent {
            applied,
            chunk_ids,
            scores: None,
        }) {
            yield Ok(Event::default().event("reranked").data(data));
        }

        // event: delta — split by lines to simulate streaming stages
        // 识别四个阶段标题，按阶段打 stage 标签
        let stage_for_line = |line: &str| -> Option<String> {
            if line.contains("调查研究") { Some("investigation".to_string()) }
            else if line.contains("主要矛盾") { Some("contradiction".to_string()) }
            else if line.contains("理论综合") { Some("synthesis".to_string()) }
            else if line.contains("指导实践") || line.contains("方针") { Some("policy".to_string()) }
            else { None }
        };
        let mut current_stage: Option<String> = None;
        for line in answer.content.lines() {
            if line.trim().is_empty() { continue; }
            if let Some(s) = stage_for_line(line) { current_stage = Some(s); }
            let evt = SseDeltaEvent { delta: format!("{line}\n"), stage: current_stage.clone() };
            if let Ok(data) = serde_json::to_string(&evt) {
                yield Ok(Event::default().event("delta").data(data));
            }
        }

        // event: citation
        if let Ok(data) = serde_json::to_string(&SseCitationEvent {
            reports: answer.citation_reports.clone(),
            is_fully_grounded: answer.is_fully_grounded,
        }) {
            yield Ok(Event::default().event("citation").data(data));
        }

        // event: done
        let elapsed_ms = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);
        if let Ok(data) = serde_json::to_string(&SseDoneEvent { is_fully_grounded: answer.is_fully_grounded, elapsed_ms }) {
            yield Ok(Event::default().event("done").data(data));
        }
    };

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}
