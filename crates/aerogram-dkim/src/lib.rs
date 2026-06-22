//! Aerogram DKIM signing and verification.
//!
//! Implements DKIM as specified by RFC 6376 (RSA-SHA256) and RFC 8463
//! (Ed25519-SHA256). Provides a [`DkimSigner`] to attach a `DKIM-Signature`
//! header to an outbound message and a [`DkimVerifier`] to evaluate the
//! signature carried by an inbound message against the `_domainkey` DNS
//! record published by the signing domain.

use serde::{Deserialize, Serialize};

/// DKIM signing algorithm identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DkimAlgorithm {
    /// `rsa-sha256` (RFC 6376).
    RsaSha256,
    /// `ed25519-sha256` (RFC 8463).
    Ed25519Sha256,
}

/// Opaque DKIM signing key paired with a selector and a domain.
#[derive(Debug, Clone)]
pub struct DkimKey {
    /// Algorithm of the key.
    pub algorithm: DkimAlgorithm,
    /// DNS selector (the `<selector>._domainkey.<domain>` label component).
    pub selector: String,
    /// Domain owning the key.
    pub domain: String,
    /// Raw key material in PKCS8 PEM.
    pub pkcs8_pem: Vec<u8>,
}

/// Signs outbound messages with a configured set of [`DkimKey`].
#[derive(Debug, Clone)]
pub struct DkimSigner {
    _keys: Vec<DkimKey>,
}

impl DkimSigner {
    /// Builds a signer from the configured keys.
    #[must_use]
    pub fn new(keys: Vec<DkimKey>) -> Self {
        Self { _keys: keys }
    }

    /// Signs the provided MIME message bytes, returning the
    /// `DKIM-Signature` header value to prepend to the message.
    ///
    /// # Errors
    ///
    /// Returns [`DkimError::Sign`] when the signing operation fails.
    pub fn sign(&self, _message: &[u8]) -> Result<String, DkimError> {
        unimplemented!("M1: signature only")
    }
}

/// Verifies DKIM signatures on inbound messages.
#[derive(Debug, Clone, Default)]
pub struct DkimVerifier;

impl DkimVerifier {
    /// Verifies all `DKIM-Signature` headers on the given message bytes
    /// against the published `_domainkey` DNS records.
    ///
    /// # Errors
    ///
    /// Returns [`DkimError::Verify`] when the verification fails for any
    /// of the signatures or when the DNS lookup fails.
    #[allow(clippy::unused_async)]
    pub async fn verify(&self, _message: &[u8]) -> Result<DkimResult, DkimError> {
        unimplemented!("M3: signature only")
    }
}

/// Outcome of a DKIM verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DkimResult {
    /// Signature verified successfully.
    Pass,
    /// Signature does not verify.
    Fail(String),
    /// Verification could not run (DNS failure, malformed signature).
    TempError(String),
    /// Permanent error (key not found, algorithm unsupported).
    PermError(String),
    /// Message had no DKIM signature to verify.
    None,
}

/// DKIM-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum DkimError {
    /// Signing failure (key parsing, hash, signature).
    #[error("dkim sign error: {0}")]
    Sign(String),
    /// Verification failure (parse, key lookup, signature mismatch).
    #[error("dkim verify error: {0}")]
    Verify(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dkim_algorithm_is_copy() {
        let a = DkimAlgorithm::Ed25519Sha256;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn dkim_result_pass_displays_predictably() {
        let result = DkimResult::Pass;
        let json = serde_json::to_string(&result).expect("serialize");
        assert_eq!(json, "\"Pass\"");
    }
}
