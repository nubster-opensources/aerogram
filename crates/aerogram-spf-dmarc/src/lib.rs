//! Aerogram SPF, DMARC and ARC evaluation.
//!
//! Provides asynchronous evaluators for the three companion sender
//! authentication mechanisms layered on top of DKIM: SPF (RFC 7208),
//! DMARC (RFC 7489) and ARC (RFC 8617). All evaluators run against the
//! DNS via `hickory-resolver` and enforce the standard `void lookup` and
//! `MX query` budgets to prevent malicious senders from triggering
//! unbounded resolution work.

use std::net::IpAddr;

use serde::{Deserialize, Serialize};

/// SPF evaluation outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpfResult {
    /// SPF authorised the sender IP.
    Pass,
    /// SPF refused the sender IP.
    Fail,
    /// SPF soft-fails the sender IP (`~all`).
    SoftFail,
    /// SPF policy does not assert anything (`?all`).
    Neutral,
    /// No SPF policy published.
    None,
    /// Temporary error during evaluation (DNS failure).
    TempError(String),
    /// Permanent error during evaluation (malformed record).
    PermError(String),
}

/// SPF evaluator backed by an asynchronous DNS resolver.
#[derive(Debug, Clone, Default)]
pub struct SpfChecker;

impl SpfChecker {
    /// Evaluates the SPF policy for the given sender domain and IP.
    ///
    /// # Errors
    ///
    /// Returns [`SpfDmarcError::Dns`] when the DNS lookup fails in a way
    /// that cannot be encoded as a [`SpfResult::TempError`] or
    /// [`SpfResult::PermError`].
    #[allow(clippy::unused_async)]
    pub async fn check(&self, _domain: &str, _ip: IpAddr) -> Result<SpfResult, SpfDmarcError> {
        unimplemented!("M3: signature only")
    }
}

/// DMARC policy as published in the `_dmarc` TXT record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DmarcPolicy {
    /// `p=none`: monitor only.
    None,
    /// `p=quarantine`: route suspicious mail to spam.
    Quarantine,
    /// `p=reject`: drop suspicious mail outright.
    Reject,
}

/// DMARC evaluator.
#[derive(Debug, Clone, Default)]
pub struct DmarcEvaluator;

impl DmarcEvaluator {
    /// Evaluates the DMARC policy for the message identified by its `From`
    /// header domain, given the upstream SPF and DKIM results.
    ///
    /// # Errors
    ///
    /// Returns [`SpfDmarcError::Dns`] when the DNS lookup fails.
    #[allow(clippy::unused_async)]
    pub async fn evaluate(
        &self,
        _from_domain: &str,
        _spf: &SpfResult,
        _dkim_aligned: bool,
    ) -> Result<DmarcPolicy, SpfDmarcError> {
        unimplemented!("M3: signature only")
    }
}

/// ARC chain verifier (RFC 8617).
#[derive(Debug, Clone, Default)]
pub struct ArcVerifier;

impl ArcVerifier {
    /// Verifies the ARC chain on the given message bytes.
    ///
    /// # Errors
    ///
    /// Returns [`SpfDmarcError::Arc`] when the chain is malformed or fails
    /// cryptographic verification.
    #[allow(clippy::unused_async)]
    pub async fn verify(&self, _message: &[u8]) -> Result<ArcResult, SpfDmarcError> {
        unimplemented!("M3: signature only")
    }
}

/// ARC chain verification outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArcResult {
    /// Chain verified.
    Pass,
    /// Chain failed verification.
    Fail(String),
    /// No ARC chain present.
    None,
}

/// SPF / DMARC / ARC errors.
#[derive(Debug, thiserror::Error)]
pub enum SpfDmarcError {
    /// DNS resolution failure not covered by a per-protocol enum variant.
    #[error("dns error: {0}")]
    Dns(String),
    /// ARC verification error.
    #[error("arc error: {0}")]
    Arc(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dmarc_policy_serde_roundtrip() {
        let original = DmarcPolicy::Quarantine;
        let json = serde_json::to_string(&original).expect("serialize");
        let restored: DmarcPolicy = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(restored, original);
    }
}
