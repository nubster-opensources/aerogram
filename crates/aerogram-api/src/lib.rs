//! Aerogram transactional REST API.
//!
//! Mounts the public `/v1/` surface: `POST /v1/messages` for transactional
//! sending, `GET /v1/messages/{id}` for status tracking, suppression list
//! management and webhook management. Webhook payloads are signed with
//! HMAC-SHA-256 over the raw HTTP body via [`WebhookSigner`].

use aerogram_core::EmailId;
use serde::{Deserialize, Serialize};

/// Public API server, mounted under `/v1/`.
#[derive(Debug, Clone, Default)]
pub struct ApiServer;

impl ApiServer {
    /// Returns an axum router exposing the API endpoints.
    #[must_use = "the router must be mounted on the application"]
    pub fn router(&self) -> axum::Router {
        axum::Router::new()
    }
}

/// Router for the `/v1/messages` subtree.
#[derive(Debug, Clone, Default)]
pub struct MessagesRouter;

/// HMAC-SHA-256 signer for outbound webhook payloads.
#[derive(Debug, Clone)]
pub struct WebhookSigner {
    _secret: Vec<u8>,
}

impl WebhookSigner {
    /// Builds a signer from the configured shared secret.
    #[must_use]
    pub fn new(secret: impl Into<Vec<u8>>) -> Self {
        Self {
            _secret: secret.into(),
        }
    }

    /// Computes the `X-Aerogram-Signature` header value for the given raw
    /// body bytes and timestamp.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Sign`] when the underlying HMAC initialisation
    /// fails (only possible with a zero-length key).
    pub fn sign(&self, _timestamp: u64, _body: &[u8]) -> Result<String, ApiError> {
        unimplemented!("M2: signature only")
    }
}

/// Payload structure for an outbound webhook call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    /// CloudEvents event type, for example `mail.delivered`.
    #[serde(rename = "type")]
    pub kind: String,
    /// Identifier of the message the event refers to.
    pub email_id: EmailId,
    /// Free-form JSON body matching the schema documented for the event type.
    pub data: serde_json::Value,
}

/// API-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// Request validation failed.
    #[error("invalid request: {0}")]
    Invalid(String),
    /// Recipient address is on the suppression list.
    #[error("recipient suppressed: {0}")]
    Suppressed(String),
    /// Signing operation failed.
    #[error("webhook sign error: {0}")]
    Sign(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn webhook_payload_serializes() {
        let payload = WebhookPayload {
            kind: "mail.delivered".into(),
            email_id: EmailId::new(uuid::Uuid::nil()),
            data: serde_json::json!({ "to": ["alice@example.com"] }),
        };
        let json = serde_json::to_string(&payload).expect("serialize");
        assert!(json.contains("mail.delivered"));
    }
}
