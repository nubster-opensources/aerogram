//! Aerogram outgoing events.
//!
//! Aerogram emits its domain events in the CloudEvents v1.0 envelope
//! (RFC-style spec hosted by the CNCF). The default sink is the
//! operator-configured webhook URL signed with HMAC-SHA-256. The `hexeract`
//! feature flag activates an optional Hexeract Outbox adapter that also
//! lands the event in a PostgreSQL outbox table for downstream consumers.

use aerogram_core::{EmailId, TenantId};
use serde::{Deserialize, Serialize};

/// Sink that publishes Aerogram domain events.
#[async_trait::async_trait]
pub trait EventEmitter: Send + Sync {
    /// Emits an event. Implementations may persist, push or fan-out.
    async fn emit(&self, event: CloudEvent) -> Result<(), EventError>;
}

/// CloudEvents v1.0 envelope as emitted by Aerogram.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudEvent {
    /// `specversion` field (always `1.0`).
    pub specversion: String,
    /// `type` field, for example `mail.delivered`.
    #[serde(rename = "type")]
    pub kind: String,
    /// `source` field, for example `aerogram://mail.example.org`.
    pub source: String,
    /// `id` field, unique per event.
    pub id: String,
    /// `time` field, RFC 3339 timestamp.
    pub time: chrono::DateTime<chrono::Utc>,
    /// `datacontenttype` field.
    pub datacontenttype: String,
    /// Free-form payload.
    pub data: serde_json::Value,
}

/// Helper that builds a [`CloudEvent`] from typed payloads.
#[derive(Debug, Clone, Default)]
pub struct CloudEventBuilder;

impl CloudEventBuilder {
    /// Builds a `mail.delivered` event.
    #[must_use]
    pub fn delivered(_email_id: EmailId, _tenant_id: TenantId) -> CloudEvent {
        unimplemented!("M2: signature only")
    }

    /// Builds a `mail.bounced` event.
    #[must_use]
    pub fn bounced(_email_id: EmailId, _tenant_id: TenantId, _reason: String) -> CloudEvent {
        unimplemented!("M2: signature only")
    }

    /// Builds a `mail.complained` event.
    #[must_use]
    pub fn complained(_email_id: EmailId, _tenant_id: TenantId) -> CloudEvent {
        unimplemented!("M2: signature only")
    }
}

/// Typed payload of the `mail.delivered` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailDeliveredEvent {
    /// Identifier of the delivered message.
    pub email_id: EmailId,
    /// Recipients the delivery succeeded for.
    pub to: Vec<String>,
    /// SMTP response captured at delivery time.
    pub smtp_response: String,
}

/// Typed payload of the `mail.bounced` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailBouncedEvent {
    /// Identifier of the bounced message.
    pub email_id: EmailId,
    /// Recipients the delivery failed for.
    pub to: Vec<String>,
    /// Bounce category (`hard`, `soft`).
    pub category: String,
    /// Reason captured from the remote MTA.
    pub reason: String,
}

/// Typed payload of the `mail.complained` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailComplainedEvent {
    /// Identifier of the message the complaint refers to.
    pub email_id: EmailId,
    /// Recipient that complained.
    pub recipient: String,
}

/// Hexeract Outbox adapter, gated by the `hexeract` feature.
#[cfg(feature = "hexeract")]
#[derive(Debug, Clone, Default)]
pub struct HexeractAdapter;

/// Event emission errors.
#[derive(Debug, thiserror::Error)]
pub enum EventError {
    /// Sink transport failure (HTTP, AMQP).
    #[error("event transport error: {0}")]
    Transport(String),
    /// Serialisation failure.
    #[error("event serialise error: {0}")]
    Serialise(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mail_delivered_event_serialises() {
        let event = MailDeliveredEvent {
            email_id: EmailId::new(uuid::Uuid::nil()),
            to: vec!["alice@example.com".into()],
            smtp_response: "250 2.0.0 OK".into(),
        };
        let json = serde_json::to_string(&event).expect("serialize");
        assert!(json.contains("alice@example.com"));
    }
}
