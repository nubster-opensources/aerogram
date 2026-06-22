//! Aerogram authentication and provisioning.
//!
//! Aerogram authenticates admin sessions and API callers through any
//! OpenID Connect-compliant identity provider. The crate exposes a thin
//! [`OidcClient`] for the discovery flow, a [`JwksValidator`] for bearer
//! token validation and a SCIM 2.0 [`ScimEndpoint`] axum router for
//! provisioning. There is no compile-time dependency on Nubster Identity.

use aerogram_core::TenantId;
use serde::{Deserialize, Serialize};

/// OIDC discovery and authorisation client.
#[derive(Debug, Clone)]
pub struct OidcClient {
    _issuer: String,
    _client_id: String,
}

impl OidcClient {
    /// Builds an OIDC client for the given issuer and client identifier.
    pub fn new(issuer: impl Into<String>, client_id: impl Into<String>) -> Self {
        Self {
            _issuer: issuer.into(),
            _client_id: client_id.into(),
        }
    }
}

/// JWKS-backed JWT validator.
#[derive(Debug, Clone)]
pub struct JwksValidator {
    _issuer: String,
}

impl JwksValidator {
    /// Builds a validator for tokens issued by the given OIDC issuer.
    pub fn new(issuer: impl Into<String>) -> Self {
        Self {
            _issuer: issuer.into(),
        }
    }

    /// Validates the given JWT against the issuer's JWKS and returns the
    /// resolved [`TenantClaim`].
    ///
    /// # Errors
    ///
    /// Returns [`AuthError::InvalidToken`] when the token cannot be
    /// validated.
    #[allow(clippy::unused_async)]
    pub async fn validate(&self, _token: &str) -> Result<TenantClaim, AuthError> {
        unimplemented!("M8: signature only")
    }
}

/// Resolved claims extracted from a verified JWT.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantClaim {
    /// Tenant identifier extracted from the configured claim.
    pub tenant_id: TenantId,
    /// Subject identifier (`sub` claim).
    pub subject: String,
    /// Granted scopes.
    pub scopes: Vec<String>,
}

/// Bearer-token extractor used as an axum extension.
#[derive(Debug, Clone, Default)]
pub struct BearerExtractor;

/// SCIM 2.0 provisioning endpoint mounted under `/scim/v2/`.
#[derive(Debug, Clone, Default)]
pub struct ScimEndpoint;

impl ScimEndpoint {
    /// Returns an axum router exposing the SCIM 2.0 endpoints.
    #[must_use = "the router must be mounted on the application"]
    pub fn router(&self) -> axum::Router {
        axum::Router::new()
    }
}

/// Authentication and authorisation errors.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// Token failed validation (signature, expiry, audience).
    #[error("invalid token: {0}")]
    InvalidToken(String),
    /// Required scope missing.
    #[error("missing scope: {0}")]
    MissingScope(String),
    /// OIDC discovery or JWKS retrieval failure.
    #[error("discovery error: {0}")]
    Discovery(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oidc_client_constructs() {
        let client = OidcClient::new("https://id.example.org", "aerogram-admin");
        drop(client);
    }
}
