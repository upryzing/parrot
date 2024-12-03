use upryzing_result::Result;

use crate::File;
use crate::FileUsedFor;
use crate::ReferenceDb;

use super::AbstractAttachments;

#[async_trait]
impl AbstractAttachments for ReferenceDb {
    /// Insert attachment into database.
    async fn insert_attachment(&self, attachment: &File) -> Result<()> {
        let mut attachments = self.files.lock().await;
        if attachments.contains_key(&attachment.id) {
            Err(create_database_error!("insert", "attachment"))
        } else {
            attachments.insert(attachment.id.to_string(), attachment.clone());
            Ok(())
        }
    }

    /// Fetch an attachment by its id.
    async fn fetch_attachment(&self, tag: &str, file_id: &str) -> Result<File> {
        let files = self.files.lock().await;
        if let Some(file) = files.get(file_id) {
            if file.tag == tag {
                Ok(file.clone())
            } else {
                Err(create_error!(NotFound))
            }
        } else {
            Err(create_error!(NotFound))
        }
    }

    /// Find an attachment by its details and mark it as used by a given parent.
    async fn find_and_use_attachment(
        &self,
        id: &str,
        tag: &str,
        used_for: FileUsedFor,
        uploader_id: String,
    ) -> Result<File> {
        let mut files = self.files.lock().await;
        if let Some(file) = files.get_mut(id) {
            if file.tag == tag {
                file.uploader_id = Some(uploader_id);
                file.used_for = Some(used_for);

                Ok(file.clone())
            } else {
                Err(create_error!(NotFound))
            }
        } else {
            Err(create_error!(NotFound))
        }
    }

    /// Mark an attachment as having been reported.
    async fn mark_attachment_as_reported(&self, id: &str) -> Result<()> {
        let mut files = self.files.lock().await;
        if let Some(file) = files.get_mut(id) {
            file.reported = Some(true);
            Ok(())
        } else {
            Err(create_error!(NotFound))
        }
    }

    /// Mark an attachment as having been deleted.
    async fn mark_attachment_as_deleted(&self, id: &str) -> Result<()> {
        let mut files = self.files.lock().await;
        if let Some(file) = files.get_mut(id) {
            file.deleted = Some(true);
            Ok(())
        } else {
            Err(create_error!(NotFound))
        }
    }

    /// Mark multiple attachments as having been deleted.
    async fn mark_attachments_as_deleted(&self, ids: &[String]) -> Result<()> {
        let mut files = self.files.lock().await;

        for id in ids {
            if !files.contains_key(id) {
                return Err(create_error!(NotFound));
            }
        }

        for id in ids {
            if let Some(file) = files.get_mut(id) {
                file.reported = Some(true);
            }
        }

        Ok(())
    }
}
