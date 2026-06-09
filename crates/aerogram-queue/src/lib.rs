//! Aerogram outbound delivery queue.
//!
//! Persistent queue backed by PostgreSQL. Items represent outbound messages
//! awaiting delivery, with retry tracking and exponential backoff. The
//! design follows the same `SELECT ... FOR UPDATE SKIP LOCKED` pattern as
//! `hexeract-outbox-postgres` so multiple workers can pull from the same
//! queue without coordination.

use std::time::Duration;

use aerogram_core::{EmailId, TenantId};
use serde::{Deserialize, Serialize};

/// Backoff policy used when a delivery attempt fails transiently.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackoffPolicy {
    /// Maximum number of attempts before dead-lettering.
    pub max_attempts: u32,
    /// Initial backoff duration.
    #[serde(with = "humantime_serde")]
    pub initial: Duration,
    /// Maximum backoff duration (caps the exponential growth).
    #[serde(with = "humantime_serde")]
    pub max: Duration,
}

/// A queue row, observed by a worker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueItem {
    /// Unique identifier of the queued message.
    pub email_id: EmailId,
    /// Tenant owning the message.
    pub tenant_id: TenantId,
    /// Number of delivery attempts already performed.
    pub attempts: u32,
    /// Wall-clock time after which the item becomes eligible again.
    pub visible_after: chrono::DateTime<chrono::Utc>,
}

/// Recorded attempt result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryAttempt {
    /// Identifier of the queue item.
    pub email_id: EmailId,
    /// Attempt number (1-indexed).
    pub attempt: u32,
    /// Wall-clock time of the attempt.
    pub at: chrono::DateTime<chrono::Utc>,
    /// Outcome: SMTP response or transport error.
    pub outcome: DeliveryOutcome,
}

/// Outcome of a delivery attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeliveryOutcome {
    /// Delivered with the recorded SMTP response.
    Delivered(String),
    /// Soft bounce: retry later.
    SoftBounce(String),
    /// Hard bounce: terminal failure.
    HardBounce(String),
    /// Transport error (TLS failure, timeout, ...).
    Transport(String),
}

/// Storage abstraction over the queue rows.
#[async_trait::async_trait]
pub trait QueueStore: Send + Sync {
    /// Pulls the next eligible items, up to `batch_size`.
    async fn pull(&self, batch_size: u32) -> Result<Vec<QueueItem>, QueueError>;

    /// Records a successful or failed delivery attempt and updates the
    /// visibility window accordingly.
    async fn record_attempt(&self, attempt: DeliveryAttempt) -> Result<(), QueueError>;

    /// Moves the given item to the dead-letter table.
    async fn dead_letter(&self, email_id: EmailId, reason: String) -> Result<(), QueueError>;
}

/// In-process queue facade used by the SMTP sender worker.
#[derive(Debug, Clone)]
pub struct Queue<S> {
    _store: S,
    _policy: BackoffPolicy,
}

impl<S> Queue<S> {
    /// Builds a queue facade from a store and a backoff policy.
    pub fn new(store: S, policy: BackoffPolicy) -> Self {
        Self { _store: store, _policy: policy }
    }
}

/// Queue-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum QueueError {
    /// Database error.
    #[error("queue database error: {0}")]
    Database(String),
    /// Malformed item.
    #[error("invalid queue item: {0}")]
    Invalid(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_policy_holds_durations() {
        let policy = BackoffPolicy {
            max_attempts: 5,
            initial: Duration::from_secs(30),
            max: Duration::from_secs(3600),
        };
        assert_eq!(policy.max_attempts, 5);
        assert!(policy.max > policy.initial);
    }
}
