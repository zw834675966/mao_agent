//! In-process HTTP metrics for /search and /ask (no heavy deps).

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use axum::Json;
use axum::http::StatusCode;
use serde::Serialize;

/// Counters + latency summary for search and ask endpoints.
#[derive(Debug, Default)]
pub struct HttpMetrics {
    pub search_requests: AtomicU64,
    pub search_errors: AtomicU64,
    pub search_latency_sum_ms: AtomicU64,
    pub search_latency_count: AtomicU64,
    pub search_latency_max_ms: AtomicU64,

    pub ask_requests: AtomicU64,
    pub ask_errors: AtomicU64,
    pub ask_latency_sum_ms: AtomicU64,
    pub ask_latency_count: AtomicU64,
    pub ask_latency_max_ms: AtomicU64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RouteMetricsSnapshot {
    pub requests: u64,
    pub errors: u64,
    pub latency_sum_ms: u64,
    pub latency_count: u64,
    pub latency_max_ms: u64,
    pub latency_avg_ms: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct MetricsSnapshot {
    pub search: RouteMetricsSnapshot,
    pub ask: RouteMetricsSnapshot,
}

impl HttpMetrics {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    fn observe_max(slot: &AtomicU64, value: u64) {
        let mut cur = slot.load(Ordering::Relaxed);
        while value > cur {
            match slot.compare_exchange_weak(cur, value, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(_) => break,
                Err(actual) => cur = actual,
            }
        }
    }

    pub fn record_search(&self, started: Instant, is_error: bool) {
        let ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
        self.search_requests.fetch_add(1, Ordering::Relaxed);
        self.search_latency_sum_ms.fetch_add(ms, Ordering::Relaxed);
        self.search_latency_count.fetch_add(1, Ordering::Relaxed);
        Self::observe_max(&self.search_latency_max_ms, ms);
        if is_error {
            self.search_errors.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_ask(&self, started: Instant, is_error: bool) {
        let ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
        self.ask_requests.fetch_add(1, Ordering::Relaxed);
        self.ask_latency_sum_ms.fetch_add(ms, Ordering::Relaxed);
        self.ask_latency_count.fetch_add(1, Ordering::Relaxed);
        Self::observe_max(&self.ask_latency_max_ms, ms);
        if is_error {
            self.ask_errors.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn route_snapshot(
        requests: &AtomicU64,
        errors: &AtomicU64,
        sum: &AtomicU64,
        count: &AtomicU64,
        max: &AtomicU64,
    ) -> RouteMetricsSnapshot {
        let requests = requests.load(Ordering::Relaxed);
        let errors = errors.load(Ordering::Relaxed);
        let latency_sum_ms = sum.load(Ordering::Relaxed);
        let latency_count = count.load(Ordering::Relaxed);
        let latency_max_ms = max.load(Ordering::Relaxed);
        let latency_avg_ms = if latency_count == 0 {
            0.0
        } else {
            latency_sum_ms as f64 / latency_count as f64
        };
        RouteMetricsSnapshot {
            requests,
            errors,
            latency_sum_ms,
            latency_count,
            latency_max_ms,
            latency_avg_ms,
        }
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            search: Self::route_snapshot(
                &self.search_requests,
                &self.search_errors,
                &self.search_latency_sum_ms,
                &self.search_latency_count,
                &self.search_latency_max_ms,
            ),
            ask: Self::route_snapshot(
                &self.ask_requests,
                &self.ask_errors,
                &self.ask_latency_sum_ms,
                &self.ask_latency_count,
                &self.ask_latency_max_ms,
            ),
        }
    }

    /// Prometheus text exposition format (0.0.4).
    pub fn render_prometheus(&self) -> String {
        let s = self.snapshot();
        let mut out = String::with_capacity(1024);
        Self::prom_line(
            &mut out,
            "Total /api/v1/search requests",
            "counter",
            "mao_search_requests_total",
            s.search.requests,
        );
        Self::prom_line(
            &mut out,
            "Total /api/v1/search errors",
            "counter",
            "mao_search_errors_total",
            s.search.errors,
        );
        Self::prom_line(
            &mut out,
            "Sum of /api/v1/search latency in milliseconds",
            "counter",
            "mao_search_latency_ms_sum",
            s.search.latency_sum_ms,
        );
        Self::prom_line(
            &mut out,
            "Count of /api/v1/search latency samples",
            "counter",
            "mao_search_latency_ms_count",
            s.search.latency_count,
        );
        Self::prom_line(
            &mut out,
            "Max /api/v1/search latency in milliseconds",
            "gauge",
            "mao_search_latency_ms_max",
            s.search.latency_max_ms,
        );
        Self::prom_line(
            &mut out,
            "Total /api/v1/ask (+ stream) requests",
            "counter",
            "mao_ask_requests_total",
            s.ask.requests,
        );
        Self::prom_line(
            &mut out,
            "Total /api/v1/ask (+ stream) errors",
            "counter",
            "mao_ask_errors_total",
            s.ask.errors,
        );
        Self::prom_line(
            &mut out,
            "Sum of /api/v1/ask latency in milliseconds",
            "counter",
            "mao_ask_latency_ms_sum",
            s.ask.latency_sum_ms,
        );
        Self::prom_line(
            &mut out,
            "Count of /api/v1/ask latency samples",
            "counter",
            "mao_ask_latency_ms_count",
            s.ask.latency_count,
        );
        Self::prom_line(
            &mut out,
            "Max /api/v1/ask latency in milliseconds",
            "gauge",
            "mao_ask_latency_ms_max",
            s.ask.latency_max_ms,
        );
        out
    }

    fn prom_line(out: &mut String, help: &str, ty: &str, name: &str, value: u64) {
        out.push_str("# HELP ");
        out.push_str(name);
        out.push(' ');
        out.push_str(help);
        out.push('\n');
        out.push_str("# TYPE ");
        out.push_str(name);
        out.push(' ');
        out.push_str(ty);
        out.push('\n');
        out.push_str(name);
        out.push(' ');
        out.push_str(&value.to_string());
        out.push('\n');
    }
}

pub async fn handle_metrics_prometheus(
    axum::extract::State(state): axum::extract::State<crate::server::state::AppState>,
) -> (
    StatusCode,
    [(axum::http::header::HeaderName, &'static str); 1],
    String,
) {
    (
        StatusCode::OK,
        [(
            axum::http::header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8",
        )],
        state.metrics.render_prometheus(),
    )
}

pub async fn handle_metrics_json(
    axum::extract::State(state): axum::extract::State<crate::server::state::AppState>,
) -> (StatusCode, Json<MetricsSnapshot>) {
    (StatusCode::OK, Json(state.metrics.snapshot()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_and_prometheus_contains_counters() {
        let m = HttpMetrics::new();
        let t0 = Instant::now();
        m.record_search(t0, false);
        m.record_search(t0, true);
        m.record_ask(t0, false);
        let snap = m.snapshot();
        assert_eq!(snap.search.requests, 2);
        assert_eq!(snap.search.errors, 1);
        assert_eq!(snap.ask.requests, 1);
        let text = m.render_prometheus();
        assert!(text.contains("mao_search_requests_total 2"));
        assert!(text.contains("mao_search_errors_total 1"));
        assert!(text.contains("mao_ask_requests_total 1"));
    }
}
