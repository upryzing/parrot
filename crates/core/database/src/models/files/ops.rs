use upryzing_result::Result;

use crate::File;

use super::FileUsedFor;

mod mongodb;
mod reference;

#[async_trait]
pub trait AbstractAttachments: Sync + Send {
    /// Insert attachment into database.
    async fn insert_attachment(&self, attachment: &File) -> Result<()>;

    /// Fetch an attachment by its id.
    async fn fetch_attachment(&self, tag: &str, file_id: &str) -> Result<File>;

    /// Find an attachment by its details and mark it as used by a given parent.
    async fn find_and_use_attachment(
        &self,
        id: &str,
        tag: &str,
        used_for: FileUsedFor,
        uploader_id: String,
    ) -> Result<File>;

    /// Mark an attachment as having been reported.
    async fn mark_attachment_as_reported(&self, id: &str) -> Result<()>;

    /// Mark an attachment as having been deleted.
    async fn mark_attachment_as_deleted(&self, id: &str) -> Result<()>;

    /// Mark multiple attachments as having been deleted.
    async fn mark_attachments_as_deleted(&self, ids: &[String]) -> Result<()>;
}
