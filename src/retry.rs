//! Bounded HTTP retry with exponential backoff (Cohere chat / rerank).

use std::future::Future;
use std::time::Duration;

use tracing::warn;

/// Retry policy for transient upstream HTTP failures.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Total attempts including the first try (minimum 1).
    pub max_attempts: u32,
    pub initial_backoff: Duration,
    pub max_backoff: Duration,
    /// When true, adds up to 50% of the current backoff as jitter.
    pub jitter: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::cohere_http()
    }
}

impl RetryPolicy {
    /// Production defaults for Cohere chat + rerank: 3 attempts, 100ms → 2s, with jitter.
    pub fn cohere_http() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(2),
            jitter: true,
        }
    }

    /// Tiny delays for unit tests (no jitter).
    pub fn fast_test() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(1),
            max_backoff: Duration::from_millis(8),
            jitter: false,
        }
    }

    /// Whether an HTTP status is worth retrying (429 / 5xx).
    pub fn should_retry_status(status: reqwest::StatusCode) -> bool {
        status.as_u16() == 429 || status.is_server_error()
    }

    /// Backoff before the attempt at `attempt_index` (0-based; sleep before attempt 1+).
    pub fn backoff_before_attempt(&self, attempt_index: u32) -> Duration {
        if attempt_index == 0 {
            return Duration::ZERO;
        }
        let shift = (attempt_index - 1).min(16);
        let base = self
            .initial_backoff
            .saturating_mul(2u32.saturating_pow(shift));
        let capped = base.min(self.max_backoff);
        if !self.jitter || capped.is_zero() {
            return capped;
        }
        let nanos = capped.as_nanos();
        let jitter_span = nanos / 2;
        if jitter_span == 0 {
            return capped;
        }
        // Cheap LCG from attempt + time bits — enough for desync, not crypto.
        let seed = (attempt_index as u128)
            .wrapping_mul(0x9E37_79B9)
            .wrapping_add(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos())
                    .unwrap_or(0),
            );
        let extra = seed % (jitter_span + 1);
        Duration::from_nanos(u64::try_from(nanos.saturating_add(extra)).unwrap_or(u64::MAX))
    }

    /// Run `op` up to `max_attempts` times. `op` receives the 0-based attempt index.
    /// Retries only when `is_retryable` returns true for the error.
    pub async fn run<T, E, F, Fut, R>(&self, mut op: F, mut is_retryable: R) -> Result<T, E>
    where
        F: FnMut(u32) -> Fut,
        Fut: Future<Output = Result<T, E>>,
        R: FnMut(&E) -> bool,
        E: std::fmt::Display,
    {
        let attempts = self.max_attempts.max(1);
        let mut last_err: Option<E> = None;
        for attempt in 0..attempts {
            let delay = self.backoff_before_attempt(attempt);
            if !delay.is_zero() {
                tokio::time::sleep(delay).await;
            }
            match op(attempt).await {
                Ok(v) => return Ok(v),
                Err(e) => {
                    let retry = attempt + 1 < attempts && is_retryable(&e);
                    if retry {
                        warn!(
                            attempt = attempt + 1,
                            max_attempts = attempts,
                            error = %e,
                            "transient failure; retrying with backoff"
                        );
                        last_err = Some(e);
                        continue;
                    }
                    return Err(e);
                }
            }
        }
        Err(last_err.expect("max_attempts >= 1"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[tokio::test]
    async fn retries_then_succeeds() {
        let hits = AtomicU32::new(0);
        let policy = RetryPolicy::fast_test();
        let out = policy
            .run(
                |_| async {
                    let n = hits.fetch_add(1, Ordering::SeqCst);
                    if n < 2 { Err("boom") } else { Ok(42) }
                },
                |_| true,
            )
            .await
            .unwrap();
        assert_eq!(out, 42);
        assert_eq!(hits.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn stops_when_not_retryable() {
        let hits = AtomicU32::new(0);
        let policy = RetryPolicy::fast_test();
        let err = policy
            .run(
                |_| async {
                    hits.fetch_add(1, Ordering::SeqCst);
                    Err::<(), _>("nope")
                },
                |_| false,
            )
            .await
            .unwrap_err();
        assert_eq!(err, "nope");
        assert_eq!(hits.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn backoff_grows_then_caps() {
        let policy = RetryPolicy {
            max_attempts: 5,
            initial_backoff: Duration::from_millis(10),
            max_backoff: Duration::from_millis(25),
            jitter: false,
        };
        assert_eq!(policy.backoff_before_attempt(0), Duration::ZERO);
        assert_eq!(policy.backoff_before_attempt(1), Duration::from_millis(10));
        assert_eq!(policy.backoff_before_attempt(2), Duration::from_millis(20));
        assert_eq!(policy.backoff_before_attempt(3), Duration::from_millis(25));
    }
}
