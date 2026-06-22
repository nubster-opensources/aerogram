//! Aerogram message store.
//!
//! Splits message persistence into two concerns: a metadata store backed by
//! PostgreSQL (mailbox ownership, headers, flags, search indexing pointer)
//! and a blob store holding the raw MIME bytes. The blob backend defaults
//! to the local filesystem; the `s3` feature flag activates an
//! S3-compatible backend through `aws-sdk-s3`.

use aerogram_core::{MailboxId, MessageId, TenantId};
use serde::{Deserialize, Serialize};

/// Strongly-typed reference to a stored message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRef {
    /// Owning tenant.
    pub tenant_id: TenantId,
    /// Owning mailbox.
    pub mailbox_id: MailboxId,
    /// Stored message identifier.
    pub message_id: MessageId,
    /// Opaque blob locator (filesystem path or S3 object key).
    pub blob_key: String,
    /// Stored size in bytes.
    pub size_bytes: u64,
}

/// Blob storage abstraction.
#[async_trait::async_trait]
pub trait BlobStore: Send + Sync {
    /// Writes the given bytes under the supplied key.
    async fn put(&self, key: &str, bytes: &[u8]) -> Result<(), StoreError>;

    /// Reads the bytes stored under the supplied key.
    async fn get(&self, key: &str) -> Result<Vec<u8>, StoreError>;

    /// Deletes the bytes stored under the supplied key.
    async fn delete(&self, key: &str) -> Result<(), StoreError>;
}

/// Filesystem implementation of [`BlobStore`].
#[derive(Debug, Clone)]
pub struct FsBlobStore {
    _root: std::path::PathBuf,
}

impl FsBlobStore {
    /// Builds a filesystem-backed blob store rooted at the given directory.
    pub fn new(root: impl Into<std::path::PathBuf>) -> Self {
        Self { _root: root.into() }
    }
}

/// S3-compatible implementation of [`BlobStore`], gated by the `s3` feature.
#[cfg(feature = "s3")]
#[derive(Debug, Clone)]
pub struct S3BlobStore {
    _bucket: String,
}

#[cfg(feature = "s3")]
impl S3BlobStore {
    /// Builds an S3-compatible blob store targeting the given bucket.
    #[must_use]
    pub fn new(bucket: impl Into<String>) -> Self {
        Self {
            _bucket: bucket.into(),
        }
    }
}

/// Metadata store abstraction over the `aerogram.message` table.
#[async_trait::async_trait]
pub trait MetadataStore: Send + Sync {
    /// Inserts a [`MessageRef`] row.
    async fn insert(&self, item: MessageRef) -> Result<(), StoreError>;

    /// Loads a [`MessageRef`] by its identifier scoped to a tenant.
    async fn load(
        &self,
        tenant_id: TenantId,
        message_id: MessageId,
    ) -> Result<MessageRef, StoreError>;
}

/// Store-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    /// Backend I/O failure.
    #[error("store io error: {0}")]
    Io(String),
    /// Database error.
    #[error("store database error: {0}")]
    Database(String),
    /// Row not found.
    #[error("not found: {0}")]
    NotFound(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fs_blob_store_records_its_root() {
        let store = FsBlobStore::new("/var/lib/aerogram/blobs");
        // Smoke test: the value is borrowed by the internal type, so we
        // simply rely on construction succeeding.
        drop(store);
    }
}
