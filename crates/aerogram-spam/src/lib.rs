//! Aerogram native antispam.
//!
//! Aerogram does not rely on an external antispam daemon. Scoring is
//! decomposed into independent components: a Bayesian classifier with an
//! on-disk corpus per tenant, DNS blocklist lookups, a greylisting tracker
//! and a heuristics engine inspired by SpamAssassin-compatible rule sets.
//! Each component produces a partial score; the [`SpamScorer`] composes
//! them into a final [`SpamScore`] and a [`Verdict`].

use std::net::IpAddr;

use aerogram_core::TenantId;
use serde::{Deserialize, Serialize};

/// Final spam score in arbitrary units, accumulated from all components.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SpamScore(pub f32);

/// Final spam verdict combining the score with the tenant policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Verdict {
    /// Below the warning threshold; deliver normally.
    Ham,
    /// Between warning and reject thresholds; mark and deliver.
    Spam,
    /// Above the reject threshold; reject at SMTP RCPT TO time.
    Reject,
    /// Greylist policy active; ask the remote MTA to retry later.
    Greylist,
}

/// Composes individual scoring components into a final verdict.
#[derive(Debug, Clone)]
pub struct SpamScorer {
    _tenant_id: TenantId,
}

impl SpamScorer {
    /// Builds a scorer scoped to a single tenant.
    #[must_use]
    pub fn new(tenant_id: TenantId) -> Self {
        Self { _tenant_id: tenant_id }
    }

    /// Scores an inbound message. Errors only when a hard backend failure
    /// prevents any component from running.
    ///
    /// # Errors
    ///
    /// Returns [`SpamError::Backend`] when no component can produce a
    /// result.
    pub async fn score(
        &self,
        _envelope_from: &str,
        _client_ip: IpAddr,
        _message: &[u8],
    ) -> Result<(SpamScore, Verdict), SpamError> {
        unimplemented!("M7: signature only")
    }
}

/// Bayesian classifier with an on-disk corpus per tenant.
#[derive(Debug, Clone, Default)]
pub struct BayesianClassifier;

impl BayesianClassifier {
    /// Classifies a message and returns its partial score.
    ///
    /// # Errors
    ///
    /// Returns [`SpamError::Backend`] when the corpus cannot be loaded.
    pub async fn classify(&self, _message: &[u8]) -> Result<SpamScore, SpamError> {
        unimplemented!("M7: signature only")
    }
}

/// DNS blocklist checker.
#[derive(Debug, Clone, Default)]
pub struct DnsblChecker;

impl DnsblChecker {
    /// Checks the given IP against the configured DNSBL zones.
    ///
    /// # Errors
    ///
    /// Returns [`SpamError::Backend`] when DNS resolution fails.
    pub async fn check(&self, _ip: IpAddr) -> Result<SpamScore, SpamError> {
        unimplemented!("M7: signature only")
    }
}

/// Tracker for greylisting state, persisted in PostgreSQL.
#[derive(Debug, Clone, Default)]
pub struct GreylistTracker;

impl GreylistTracker {
    /// Returns `true` if the triplet `(sender, recipient, ip)` has been
    /// seen before and should be allowed through.
    ///
    /// # Errors
    ///
    /// Returns [`SpamError::Backend`] when the database is unreachable.
    pub async fn was_seen(
        &self,
        _from: &str,
        _to: &str,
        _ip: IpAddr,
    ) -> Result<bool, SpamError> {
        unimplemented!("M7: signature only")
    }
}

/// Spam scoring errors.
#[derive(Debug, thiserror::Error)]
pub enum SpamError {
    /// Backend failure (DNS, database, filesystem).
    #[error("spam backend error: {0}")]
    Backend(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verdict_is_serializable() {
        let v = Verdict::Spam;
        let json = serde_json::to_string(&v).expect("serialize");
        assert_eq!(json, "\"Spam\"");
    }
}
