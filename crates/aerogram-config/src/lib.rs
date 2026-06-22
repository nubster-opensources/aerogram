//! Aerogram configuration model.
//!
//! Parses the single TOML configuration file consumed by `aerogram start`,
//! validates cross-section invariants (TLS configured when ports require it,
//! at least one DKIM key per configured domain, OIDC settings consistent),
//! and exposes a hot-reload hook for non-structural settings.

use std::path::Path;
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Root configuration consumed by `aerogram start`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Listener configuration (SMTP, IMAP, HTTP).
    pub server: ServerConfig,
    /// TLS termination configuration.
    pub tls: TlsConfig,
    /// DKIM signing configuration per domain.
    pub dkim: DkimConfig,
    /// Store backend configuration.
    pub store: StoreConfig,
    /// Outbound queue configuration.
    pub queue: QueueConfig,
    /// Authentication configuration.
    pub auth: AuthConfig,
}

impl Config {
    /// Parses the configuration from a TOML file on disk.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if the file cannot be read,
    /// [`ConfigError::Parse`] if the TOML is malformed, or
    /// [`ConfigError::Invalid`] if the cross-section validation fails.
    pub fn from_path(_path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        unimplemented!("M0: signature only")
    }

    /// Validates cross-section invariants. Called by [`Config::from_path`].
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Invalid`] if the configuration is inconsistent.
    pub fn validate(&self) -> Result<(), ConfigError> {
        unimplemented!("M0: signature only")
    }
}

/// Listener binds for the SMTP, IMAP and HTTP services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Bind for the inbound SMTP listener on port 25.
    pub smtp_inbound_bind: String,
    /// Bind for the submission listener on port 587.
    pub smtp_submission_bind: String,
    /// Bind for the implicit TLS submission listener on port 465.
    pub smtp_submission_tls_bind: String,
    /// Bind for the `IMAP4rev2` listener on port 993.
    pub imap_bind: String,
    /// Bind for the HTTP listener serving JMAP, API, admin and webmail.
    pub http_bind: String,
}

/// TLS termination certificates and policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Path to the PEM-encoded certificate chain.
    pub cert_path: String,
    /// Path to the PEM-encoded private key.
    pub key_path: String,
}

/// DKIM signing configuration per domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DkimConfig {
    /// Directory storing per-domain DKIM private keys.
    pub key_dir: String,
}

/// Store backend configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreConfig {
    /// PostgreSQL connection URL.
    pub postgres_url: String,
    /// Path to the blob storage root when using the filesystem backend.
    pub blob_dir: Option<String>,
    /// Optional S3-compatible blob backend (gated by the `s3` feature).
    pub s3: Option<S3Config>,
}

/// S3-compatible storage backend configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Config {
    /// S3 endpoint URL.
    pub endpoint: String,
    /// Bucket name.
    pub bucket: String,
    /// AWS region (or a value compatible with the chosen provider).
    pub region: String,
}

/// Outbound queue configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    /// Maximum number of delivery attempts before dead-lettering.
    pub max_attempts: u32,
    /// Initial backoff between two delivery attempts.
    #[serde(with = "humantime_serde")]
    pub initial_backoff: Duration,
    /// Maximum backoff between two delivery attempts.
    #[serde(with = "humantime_serde")]
    pub max_backoff: Duration,
}

/// Authentication configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// OIDC issuer URL (any `RFC 6749` / OpenID Connect compliant provider).
    pub oidc_issuer: String,
    /// OIDC client identifier for the admin and API surfaces.
    pub oidc_client_id: String,
    /// JWT claim used to resolve the tenant identifier.
    pub tenant_claim: String,
}

/// Errors raised by configuration parsing and validation.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// Filesystem access failure.
    #[error("io error: {0}")]
    Io(String),
    /// Malformed TOML input.
    #[error("parse error: {0}")]
    Parse(String),
    /// Cross-section validation failure.
    #[error("invalid configuration: {0}")]
    Invalid(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_error_displays_message() {
        let err = ConfigError::Invalid("missing tls".into());
        assert_eq!(err.to_string(), "invalid configuration: missing tls");
    }
}
