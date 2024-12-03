use upryzing_result::Result;

use crate::Snapshot;

mod mongodb;
mod reference;

#[async_trait]
pub trait AbstractSnapshot: Sync + Send {
    /// Insert a new snapshot into the database
    async fn insert_snapshot(&self, snapshot: &Snapshot) -> Result<()>;
}
