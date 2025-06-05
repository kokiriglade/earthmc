//! # Retry strategy
//!
//! The default retry strategy and how to write your own retry logic.
use derive_builder::Builder;
use parking_lot::Mutex;
use std::{sync::Arc, time::Duration};

use rand::{Rng, SeedableRng, rngs::StdRng};

/// Trait to define a retry strategy.
pub trait RetryStrategy: Send + Sync {
    /// This function is called every time a request to the EarthMC API fails
    /// to determine if a retry attempt should be made and how much time to wait
    /// before the next attempt is made.
    ///
    /// When this function returns `None` there will be no more retries and the
    /// execution fails.
    /// When this function returns `Some(duration)` the client will wait as
    /// long as the specified duration before performing the request again.
    ///
    /// You could write a very simple retry strategy that always retries
    /// immediately as follow:
    ///
    /// ```rust
    /// use earthmc::retry_strategy::RetryStrategy;
    /// use std::time::Duration;
    ///
    /// struct AlwaysRetry {}
    ///
    /// impl RetryStrategy for AlwaysRetry {
    ///     fn should_retry_after(&mut self, attempt: usize) -> Option<Duration> {
    ///         Some(Duration::from_secs(0))
    ///     }
    /// }
    /// ```
    ///
    /// Or a strategy to never retry:
    ///
    /// ```rust
    /// use earthmc::retry_strategy::RetryStrategy;
    /// use std::time::Duration;
    ///
    /// struct NeverRetry {}
    ///
    /// impl RetryStrategy for NeverRetry {
    ///     fn should_retry_after(&mut self, attempt: usize) -> Option<Duration> {
    ///         None
    ///     }
    /// }
    /// ```
    ///
    /// Check out the [`JitteredBackoff`] retry strategy and the `examples`
    /// directory for more examples.
    fn should_retry_after(&mut self, attempt: usize) -> Option<Duration>;
}

/// The default retry strategy.
///
/// It performs a Jittered backoff for a given maximum number of times.
///
/// The wait duration is calculated using the formula (in milliseconds):
///
/// ```plain
/// 2 ^ (retry_count) * 1000 - random_jitter
/// ```
///
/// Where `random_jitter` is a random number between `0` and `999`.
#[derive(Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct JitteredBackoff {
    /// The maximum number of retries before giving up.
    #[builder(default = 5)]
    max_retry: usize,
    /// The random number generator to use.
    #[builder(default = Arc::new(Mutex::new(StdRng::from_os_rng())))]
    rng: Arc<Mutex<StdRng>>,
}

impl RetryStrategy for JitteredBackoff {
    fn should_retry_after(&mut self, retry_count: usize) -> Option<Duration> {
        if retry_count >= self.max_retry {
            return None;
        }

        let mut guard = self.rng.lock();

        let jitter_ms = guard.random_range(0..1000);

        let base_ms = 2_u64
            .saturating_pow(retry_count as u32)
            .saturating_mul(1_000);

        let wait_ms = base_ms.saturating_sub(jitter_ms);

        Some(Duration::from_millis(wait_ms))
    }
}

/// Creates a new [`JitteredBackoff`] with the default maximum number of
/// retries (5).
impl Default for JitteredBackoff {
    fn default() -> Self {
        JitteredBackoffBuilder::default()
            .build()
            .expect("Builder defaults are valid")
    }
}
