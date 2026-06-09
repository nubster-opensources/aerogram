//! Aerogram core domain types.
//!
//! This crate defines the value types shared by every other Aerogram crate:
//! tenant and mailbox identifiers, addresses, envelopes, headers, MIME types
//! and the common error enum. It has no runtime dependencies beyond the
//! Rust ecosystem essentials and is safe to depend on from anywhere in the
//! workspace.

use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Opaque tenant identifier carried in every multi-tenant operation.
///
/// The value is sourced from a verified JWT claim or from the operator-side
/// CLI; Aerogram never trusts a tenant identifier supplied in a request body
/// or query parameter.
///
/// # Examples
///
/// ```
/// use aerogram_core::TenantId;
/// use uuid::Uuid;
///
/// let tenant = TenantId::new(Uuid::nil());
/// assert_eq!(tenant.as_uuid(), Uuid::nil());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TenantId(Uuid);

impl TenantId {
    /// Wraps a raw [`Uuid`] as a [`TenantId`].
    #[must_use]
    pub const fn new(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns the wrapped [`Uuid`].
    #[must_use]
    pub const fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for TenantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Opaque mailbox identifier (UUID v7, monotonic).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MailboxId(Uuid);

impl MailboxId {
    /// Wraps a raw [`Uuid`] as a [`MailboxId`].
    #[must_use]
    pub const fn new(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns the wrapped [`Uuid`].
    #[must_use]
    pub const fn as_uuid(&self) -> Uuid {
        self.0
    }
}

/// Opaque outbound message identifier returned by the transactional API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmailId(Uuid);

impl EmailId {
    /// Wraps a raw [`Uuid`] as an [`EmailId`].
    #[must_use]
    pub const fn new(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns the wrapped [`Uuid`].
    #[must_use]
    pub const fn as_uuid(&self) -> Uuid {
        self.0
    }
}

/// Opaque on-disk message identifier in the metadata store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId(Uuid);

impl MessageId {
    /// Wraps a raw [`Uuid`] as a [`MessageId`].
    #[must_use]
    pub const fn new(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns the wrapped [`Uuid`].
    #[must_use]
    pub const fn as_uuid(&self) -> Uuid {
        self.0
    }
}

/// Parsed RFC 5321 mailbox: `local-part@domain`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address {
    /// The local-part of the address (before the `@`).
    pub local: String,
    /// The domain of the address (after the `@`).
    pub domain: Domain,
}

/// Parsed and validated mail domain.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Domain(String);

impl Domain {
    /// Returns the domain as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// SMTP envelope: sender and recipient list as observed on the wire.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Envelope {
    /// `MAIL FROM` address (the return path).
    pub from: Address,
    /// `RCPT TO` addresses.
    pub to: Vec<Address>,
}

/// A single MIME header field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    /// Header field name (case-insensitive comparison upstream).
    pub name: String,
    /// Raw header value (unfolded but not decoded).
    pub value: String,
}

/// Parsed `Content-Type` with parameters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MimeType {
    /// The top-level type (`text`, `application`, `multipart`, ...).
    pub kind: String,
    /// The subtype (`plain`, `html`, `pdf`, ...).
    pub subtype: String,
    /// Optional parameters (`charset`, `boundary`, ...).
    pub params: Vec<(String, String)>,
}

/// Common Aerogram error type re-exported by every functional crate that
/// returns a value to outside callers.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid input from a request, a config file or a parsing routine.
    #[error("invalid input: {0}")]
    Invalid(String),
    /// Resource was not found.
    #[error("not found: {0}")]
    NotFound(String),
    /// Permission was denied for the requested operation.
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    /// Internal error that should never reach a caller untouched.
    #[error("internal error: {0}")]
    Internal(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tenant_id_roundtrips_through_uuid() {
        let raw = Uuid::nil();
        let id = TenantId::new(raw);
        assert_eq!(id.as_uuid(), raw);
    }

    #[test]
    fn email_id_serde_roundtrip() {
        let original = EmailId::new(Uuid::nil());
        let json = serde_json::to_string(&original).expect("serialize");
        let restored: EmailId = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(restored, original);
    }
}
