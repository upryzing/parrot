use upryzing_result::Result;

use crate::FileHash;

mod mongodb;
mod reference;

#[async_trait]
pub trait AbstractAttachmentHashes: Sync + Send {
    /// Insert a new attachment hash into the database.
    async fn insert_attachment_hash(&self, hash: &FileHash) -> Result<()>;

    /// Fetch an attachment hash entry by sha256 hash.
    async fn fetch_attachment_hash(&self, hash: &str) -> Result<FileHash>;

    /// Update an attachment hash nonce value.
    async fn set_attachment_hash_nonce(&self, hash: &str, nonce: &str) -> Result<()>;
}
