//! Aerogram full-text search.
//!
//! Embedded full-text index over headers and message bodies built with
//! `tantivy`. Indexes live per-tenant on disk to keep storage isolated and
//! to make tenant lifecycle (deletion, export) tractable.

use aerogram_core::{MessageId, TenantId};
use serde::{Deserialize, Serialize};

/// A built search index for a single tenant.
#[derive(Debug)]
pub struct SearchIndex {
    _tenant_id: TenantId,
}

impl SearchIndex {
    /// Opens the on-disk index for the given tenant.
    ///
    /// # Errors
    ///
    /// Returns [`SearchError::Io`] when the underlying directory cannot be
    /// opened.
    pub fn open(_tenant_id: TenantId) -> Result<Self, SearchError> {
        unimplemented!("M4: signature only")
    }

    /// Runs the given query against the index.
    ///
    /// # Errors
    ///
    /// Returns [`SearchError::Query`] when the query cannot be parsed or
    /// when index reading fails.
    pub fn search(&self, _query: &Query) -> Result<Vec<SearchResult>, SearchError> {
        unimplemented!("M4: signature only")
    }
}

/// Helper that incrementally builds a [`SearchIndex`].
#[derive(Debug)]
pub struct IndexBuilder {
    _tenant_id: TenantId,
}

impl IndexBuilder {
    /// Creates a new builder for the given tenant.
    #[must_use]
    pub fn new(tenant_id: TenantId) -> Self {
        Self { _tenant_id: tenant_id }
    }
}

/// A parsed search query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// Free-text query string.
    pub q: String,
    /// Maximum number of results to return.
    pub limit: u32,
}

/// A single hit returned by [`SearchIndex::search`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Identifier of the matched message.
    pub message_id: MessageId,
    /// Relevance score in `[0.0, 1.0]`.
    pub score: f32,
}

/// Search-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    /// Underlying I/O or directory error.
    #[error("search io error: {0}")]
    Io(String),
    /// Query parsing or execution error.
    #[error("search query error: {0}")]
    Query(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_serde_roundtrip() {
        let q = Query { q: "subject:hello".to_string(), limit: 25 };
        let json = serde_json::to_string(&q).expect("serialize");
        assert!(json.contains("subject:hello"));
    }
}
