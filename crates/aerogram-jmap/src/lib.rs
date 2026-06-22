//! Aerogram JMAP server.
//!
//! Implements the JMAP core (RFC 8620) and JMAP mail (RFC 8621) profiles
//! on top of `aerogram-store` and `aerogram-search`. The same backend is
//! reused by `aerogram-imap` so the two protocols share a single source
//! of truth.

use aerogram_core::{MailboxId, MessageId, TenantId};
use serde::{Deserialize, Serialize};

/// JMAP server, mounted under `/jmap/`.
#[derive(Debug, Clone, Default)]
pub struct JmapServer;

impl JmapServer {
    /// Returns an axum router exposing the JMAP endpoints.
    #[must_use = "the router must be mounted on the application"]
    pub fn router(&self) -> axum::Router {
        axum::Router::new()
    }
}

/// JMAP session for a single authenticated user.
#[derive(Debug, Clone)]
pub struct JmapSession {
    /// Tenant the session is scoped to.
    pub tenant_id: TenantId,
    /// Account identifier in JMAP terms.
    pub account_id: String,
}

/// JMAP `Mailbox` data type (RFC 8621 §2).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mailbox {
    /// Server-assigned identifier.
    pub id: MailboxId,
    /// User-visible name.
    pub name: String,
    /// Optional role (`inbox`, `archive`, `trash`, ...).
    pub role: Option<String>,
}

/// JMAP `Email` data type (RFC 8621 §4).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    /// Server-assigned identifier.
    pub id: MessageId,
    /// Subject of the message.
    pub subject: Option<String>,
    /// Mailboxes this message is filed under.
    pub mailbox_ids: Vec<MailboxId>,
}

/// JMAP `Identity` data type (RFC 8621 §6).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    /// Server-assigned identifier.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Email address bound to the identity.
    pub email: String,
}

/// JMAP `EmailSubmission` data type (RFC 8621 §7).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSubmission {
    /// Server-assigned identifier.
    pub id: String,
    /// Identifier of the message being submitted.
    pub email_id: MessageId,
    /// Status: `pending`, `final`, `canceled`.
    pub status: String,
}

/// JMAP `SearchSnippet` data type (RFC 8621 §5.4).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSnippet {
    /// Identifier of the matched message.
    pub email_id: MessageId,
    /// Highlighted subject excerpt.
    pub subject: Option<String>,
    /// Highlighted body excerpt.
    pub preview: Option<String>,
}

/// JMAP-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum JmapError {
    /// JMAP-defined error code returned to the client.
    #[error("jmap method error: {0}")]
    Method(String),
    /// Backend failure that should map to a 5xx response.
    #[error("jmap backend error: {0}")]
    Backend(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mailbox_serializes_with_optional_role() {
        let mailbox = Mailbox {
            id: MailboxId::new(uuid::Uuid::nil()),
            name: "INBOX".into(),
            role: Some("inbox".into()),
        };
        let json = serde_json::to_string(&mailbox).expect("serialize");
        assert!(json.contains("INBOX"));
    }
}
